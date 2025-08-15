/*
Endo functor maps objects and morphisms of a category to itself.
This is a specific case of a functor where the source and target categories are the same.

For our category framework since we always have a definite set of objects and morphisms

Endo functor will create a new category with the objects and morphisms of the original category.
and the new objects and morphisms mapped to the new values.

For instance if category A has objects {a, b} and morphisms {f: a -> b, g: b -> a},
an endo functor F would map these to a new category B with objects {F(a), F(b)} and morphisms {F(f): F(a) -> F(b), F(g): F(b) -> F(a)}.
This is useful for creating new categories that are derived from the original category, such as the category of sets or the category of groups.
 */
use crate::core::category::Category;
use crate::core::discrete_category::DiscreteCategory;
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn apply_product(
    source_category: Rc<DiscreteCategory<String>>,
    fixed_category: Rc<DiscreteCategory<String>>,
) -> Result<Rc<Functor<String, DiscreteCategory<String>, DiscreteCategory<String>>>, Errors>
where
    // <SourCategory as CategoryTrait>::Object: CategoryTrait<Identifier = String>,
{
    // we take a product of the source category and the fixed category
    // and map the objects and morphisms of the source category to the target category
    let mut target_category = DiscreteCategory::new();

    let fixed_objects = fixed_category.get_all_objects()?;

    let mut mappings = HashMap::new();

    // first map the objects from the source category to the target category
    for source_object in source_category.get_all_objects()? {
        let mut target_object = Rc::new(DiscreteCategory::new_with_id(
            source_object.category_id().clone(),
        ));
        for fixed_object in &fixed_objects {
            // product object (_) * fixed_object
            let new_value = (*source_object.clone()).category_id().to_owned()
                + (*fixed_object).clone().category_id();
            Rc::get_mut(&mut target_object)
                .ok_or(Errors::ObjectNotFound)?
                .add_object(Rc::new(new_value.into()))?;
        }
        target_category.add_object(target_object.clone())?;

        let identity_morphism =
            source_category.get_identity_morphism(source_object.category_id())?;
        let target_identity_morphism =
            target_category.get_identity_morphism(target_object.category_id())?;
        mappings.insert(identity_morphism.clone(), target_identity_morphism.clone());
    }

    // todo add mapping of morphisms from source category to target category
    // for source_morphism in source_category.get_all_morphisms()?{
    //     if source_morphism.is_identity() {
    //         continue; // skip identity morphisms
    //     }
    //     let source_object = source_morphism.source_object();
    //     let target_object = source_morphism.target_object();
    //
    //
    // }

    Ok(Rc::new(Functor::new(
        String::generate(),
        source_category,
        Rc::new(target_category),
        mappings,
    )))
}
