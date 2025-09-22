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
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryFromObjects, CategoryTrait};
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

pub async fn apply_product<Category: CategoryTrait + Eq + Hash>(
    category: &mut Category,
    source_object: &Arc<Category::Object>,
    fixed_object: Arc<Category::Object>,
) -> Result<HashMap<Arc<Category::Morphism>, Arc<Category::Morphism>>, Errors>
where
    <<<Category as CategoryTrait>::Object as CategoryTrait>::Object as CategoryTrait>::Object:
        From<ObjectId>, // <SourCategory as CategoryTrait>::Object: CategoryTrait<Identifier = String>,
{
    // we take a product of the source category and the fixed category
    // and map the objects and morphisms of the source category to the target category
    let mut target_object = Category::Object::new().await?;

    let fixed_objects = fixed_object.get_all_objects().await?;

    let mut source_object_mapping = HashMap::new();

    // first map the objects from the source category to the target category
    for source_sub_identity_morphism in source_object.get_all_identity_morphisms().await? {
        for fixed_sub_object in &fixed_objects {
            // product object (_) * fixed_object
            let new_value = (*source_sub_identity_morphism.source_object().clone())
                .category_id()
                .to_owned()
                + (*fixed_sub_object).clone().category_id().clone();
            let new_category =
                <Category::Object as CategoryTrait>::Object::from_object(new_value).await?;
            let new_category_rc = Arc::new(new_category);

            let target_sub_identity_morphism =
                target_object.add_object(new_category_rc.clone()).await?;
            source_object_mapping
                .entry(source_sub_identity_morphism)
                .or_insert_with(Vec::new)
                .push(target_sub_identity_morphism);
        }
    }

    let source_object_mapped_product = Arc::new(target_object);
    category
        .add_object(source_object_mapped_product.clone())
        .await?;

    // should map other related object
    let mut morphism_mapping = HashMap::new();

    let object_morphisms = category
        .get_object_morphisms(&*source_object)
        .await?
        .into_iter()
        .map(|m| m.clone())
        .collect::<Vec<_>>();

    for morphism in object_morphisms {
        dbg!(&morphism);
        if morphism.is_identity() {
            let identity_mapped_object = category
                .get_identity_morphism(&*source_object_mapped_product)
                .await?;
            morphism_mapping.insert(morphism.clone(), identity_mapped_object.clone());
            continue;
        }
        let arrow_mapping = morphism.arrow_mappings();
        let mut new_mapping = HashMap::new();
        let mut target_object = Category::Object::new().await?;
        for (source_sub_morphism, target_sub_morphism) in arrow_mapping.into_iter().flatten() {
            let mut mapped_objects = Vec::new();
            for fixed_sub_object in &fixed_objects {
                // product object (_) * fixed_object
                let new_value = (*target_sub_morphism.source_object().clone())
                    .category_id()
                    .to_owned()
                    + (*fixed_sub_object).clone().category_id().clone();
                let new_category =
                    <Category::Object as CategoryTrait>::Object::from_object(new_value).await?;
                let new_category_rc = Arc::new(new_category);
                target_object.add_object(new_category_rc.clone()).await?;
                let mapped_morphism = target_object
                    .get_identity_morphism(&*new_category_rc)
                    .await?;
                mapped_objects.push(mapped_morphism.clone());
            }

            if let Some(source_mapped_morphism) = source_object_mapping.get(source_sub_morphism) {
                for (index, source_mapped_object) in source_mapped_morphism.iter().enumerate() {
                    let target_mapped_morphism = &mapped_objects[index];
                    new_mapping.insert(
                        source_mapped_object.clone().clone(),
                        target_mapped_morphism.clone(),
                    );
                }
            } else {
                return Err(Errors::InvalidFunctor("Invalid mapping".to_string()));
            }
        }
        let target_object = Arc::new(target_object);

        category.add_object(target_object.clone()).await?;

        let new_morphism = Arc::new(Category::Morphism::new(
            String::generate(),
            source_object_mapped_product.clone(),
            target_object.clone(),
            new_mapping,
        ));
        category.add_morphism(new_morphism.clone()).await?;

        morphism_mapping.insert(morphism.clone(), new_morphism.clone());
    }
    Ok(morphism_mapping)
}
