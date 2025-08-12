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
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::HashMap;
use std::rc::Rc;

pub fn apply_endofunctor<
    Id: Identifier,
    SourCategory: CategoryTrait,
    FixeCategory: CategoryTrait,
>(
    id: Id,
    source_category: Rc<SourCategory>,
    fixed_category: Rc<FixeCategory>,
) -> Result<Rc<Functor<Id, SourCategory, SourCategory>>, Errors> {
    // we take a product of the source category and the fixed category
    // and map the objects and morphisms of the source category to the target category
    let mut target_category = Rc::new(SourCategory::new());

    let mut mappings: HashMap<Rc<SourCategory::Morphism>, Rc<SourCategory::Morphism>> =
        HashMap::new();

    let fixed_identity_morphisms = fixed_category.get_all_identity_morphisms()?;

    for source_morphism in source_category.get_all_morphisms()? {
        if (*source_morphism).is_identity() {
            // we map it to the identity morphism in target category
            // first we create an object which is a discrete category of the object product.

            let source_object = source_morphism.source_object();

            for fixed_identity in &fixed_identity_morphisms {
                // // create a new object in the target category
                // let new_object = target_category.add_object(
                //     source_object.clone())?;

                // create a new identity morphism in the target category
                let morphism = source_morphism.clone();
                let new_morphism = Rc::get_mut(&mut target_category)
                    .unwrap()
                    .add_morphism(morphism.clone())?;
                // let new_identity_morphism = target_category.add_morphism(new_object)?;
                // let new_identity_morphism = target_category.get_all_identity_morphisms()?.iter().last().unwrap();

                // map the source identity morphism to the new identity morphism
                mappings.insert(source_morphism.clone(), morphism);
            }
        }
    }

    Ok(Rc::new(Functor::new(
        id,
        source_category,
        target_category,
        mappings,
    )))
}
