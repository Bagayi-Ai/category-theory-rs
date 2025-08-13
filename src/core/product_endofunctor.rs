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
use std::collections::{HashMap, HashSet};
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::rc::Rc;
use crate::core::category::Category;
use crate::core::product_object::ProductObject;

pub fn apply_product<
    Id: Identifier<Id = Id>,
    SourCategory: CategoryTrait,
    FixeCategory: CategoryTrait,
>(
    source_category: Rc<SourCategory>,
    fixed_category: Rc<FixeCategory>,
) -> Result<Rc<Category<Id, ProductObject<Id, SourCategory::Object, FixeCategory::Object>>>, Errors> {
    // we take a product of the source category and the fixed category
    // and map the objects and morphisms of the source category to the target category
    let mut target_category = Category::new();

    let fixed_objects = fixed_category.get_all_objects()?;

    let mut duplicate_check = HashMap::new();

    // first map the objects from the source category to the target category
    for source_object in source_category.get_all_objects()? {
        for fixed_object in &fixed_objects {
            let id = (source_object.category_id(), fixed_object.category_id());
            if duplicate_check.contains_key(&id) {
                continue; // skip if the product object already exists
            }
            // product object (_) * fixed_object
            let product_object = Rc::new(ProductObject::new(
                Id::generate(),
                source_object.clone(),
                (*fixed_object).clone()));
            // check if the product object already exists

            target_category.add_object(product_object.clone())?;
            duplicate_check.insert(id.clone(), product_object);
        }
    }

    Ok(Rc::new(target_category))
}
