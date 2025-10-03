/*
Functor that takes an object and create a new object with the same structure as the original object
but with more objects and morphisms.
 */
use std::sync::Arc;
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryCloneWithNewId, CategoryTrait};
use crate::core::identifier::Identifier;

pub async fn inclusion_functor<Category>(
    category: Arc<Category>,
    objects: Vec<Arc<Category::Object>>) -> Result<Functor<Category, Category>, Errors>
where
    Category: CategoryTrait + CategoryCloneWithNewId
{
    let mut new_category = category.clone_with_new_id().await?;
    for object in objects {
        new_category.add_object(object).await?;
    }

    // now we create a mapping from the original category to the new category
    // for each morphism in the original category, we find the corresponding morphism in the new category
    // and create a mapping from the original morphism to the new morphism
    let mut morphism_mapping = std::collections::HashMap::new();
    for morphism in category.get_all_morphisms().await? {
        let target_morphism = new_category.get_morphism(morphism.arrow_id()).await?;
        morphism_mapping.insert(morphism.clone(), target_morphism.clone());
    }

    Ok(Functor::new(String::generate(), category, Arc::new(new_category), morphism_mapping))
}