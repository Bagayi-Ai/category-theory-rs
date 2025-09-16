use crate::core::errors::Errors;
use crate::core::arrow::{Arrow, Morphism, Functor};
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

pub fn expand_functor<
    SubObject: CategoryTrait + Eq + Hash,
    Object: CategoryTrait<Object = SubObject> + Eq + Hash + From<Rc<SubObject>>,
    Category: CategoryTrait<Object = Object> + Eq + Hash,
>(
    category: &Rc<Category>,
) -> Result<HashSet<Functor<Category, Category>>, Errors> {
    //  /*
    //  This expands each object in a category to its own object.
    //
    //  Example
    //  prefix: [a, e]
    //
    //  alphabet: [a,b]
    //
    //  Applying product of prefix and alphabet:
    //  a -> a*a
    //  result category functor should result in same numer of objects.
    //  source: [e, a, ab]
    //
    // target should have three objects so it will have two objects
    // target: [
    //      [a, b],
    //      [aa, ab]
    //      [ba, bb]
    // ]
    //
    //
    // To expand is to create a functor.
    // treat category as initial
    //
    //
    //   */
    let mut expanded_category = Category::new();
    let mut object_mappings = HashMap::new();
    let mut max_functor = 0;
    for (index, object) in category.get_all_objects()?.into_iter().enumerate() {
        for sub_object in object.get_all_objects()? {
            let current_mapping = object_mappings.entry(object).or_insert_with(Vec::new);
            // convert sub_object to Object
            let new_object =
                Object::try_from(sub_object.clone()).map_err(|_| Errors::ConversionError)?;
            match expanded_category.get_object(&new_object) {
                Ok(_) => continue, // object already exists
                Err(Errors::ObjectNotFound) => {
                    let new_object = Rc::new(new_object);
                    // object does not exist, we can add it
                    expanded_category.add_object(new_object.clone())?;
                    current_mapping.push(new_object.clone());
                    new_object // return the newly added object
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
    // todo!()
}
