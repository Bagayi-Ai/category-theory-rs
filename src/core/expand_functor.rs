use crate::core::discrete_category::DiscreteCategory;
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn expand_functor(
    category: &Rc<DiscreteCategory<String>>,
) -> Result<HashSet<Functor<String, DiscreteCategory<String>, DiscreteCategory<String>>>, Errors> {
    /*
    This expands each object in a category to its own object.
     */
    let mut expanded_category = DiscreteCategory::new();
    let mut object_mappings = HashMap::new();
    let mut max_functor = 0;
    for (index, object) in category.get_all_objects()?.into_iter().enumerate() {
        for sub_object in object.get_all_objects()? {
            let current_mapping = object_mappings.entry(object).or_insert_with(Vec::new);
            match expanded_category.get_object(sub_object) {
                Ok(_) => continue, // object already exists
                Err(Errors::ObjectNotFound) => {
                    // object does not exist, we can add it
                    expanded_category.add_object(sub_object.clone())?;
                    current_mapping.push(sub_object.clone());
                    sub_object // return the newly added object
                }
                Err(e) => return Err(e), // some other error occurred
            };
        }
        let current_count = object_mappings.len();
        if current_count > max_functor {
            max_functor = current_count;
        }
    }

    let expanded_category = Rc::new(expanded_category);

    let mut functor_mapping = HashMap::new();

    for (object, sub_objects) in object_mappings {
        let identity_morphism = category.get_identity_morphism(object)?;
        for index in 0..max_functor {
            let current_mapping = functor_mapping.entry(index).or_insert_with(HashMap::new);
            let mapped_object = if sub_objects.len() <= index {
                sub_objects.get(index).ok_or(Errors::ObjectNotFound)?
            } else {
                sub_objects.first().ok_or(Errors::ObjectNotFound)?
            };
            let mapped_object_identity = expanded_category.get_identity_morphism(mapped_object)?;
            current_mapping.insert(identity_morphism.clone(), mapped_object_identity.clone());
        }
    }

    let mut functors = HashSet::new();
    for (index, mapping) in functor_mapping {
        let functor = Functor::new(
            String::generate(),
            category.clone(),
            expanded_category.clone(),
            mapping,
        );
        functors.insert(functor);
    }

    Ok(functors)
}
