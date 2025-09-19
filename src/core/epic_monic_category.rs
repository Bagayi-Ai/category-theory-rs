use crate::core::arrow::{Arrow, Morphism};
use crate::core::errors::Errors;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait};
use crate::core::traits::factorization_system_trait::FactorizationSystemTrait;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq,
{
    category: InnerCategory,
    morphism_factors: HashMap<
        Arc<Morphism<InnerCategory::Object>>,
        (
            Arc<Morphism<InnerCategory::Object>>,
            Arc<Morphism<InnerCategory::Object>>,
        ),
    >,
}

impl<InnerCategory> EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + 'static,
{
    pub async fn new() -> Result<Self, Errors> {
        Ok(EpicMonicCategory {
            category: InnerCategory::new().await?,
            morphism_factors: HashMap::new(),
        })
    }

    pub fn category(&self) -> &InnerCategory {
        &self.category
    }

    async fn factorize(
        &mut self,
        morphism: &Morphism<InnerCategory::Object>,
    ) -> Result<(Arc<InnerCategory::Morphism>, Arc<InnerCategory::Morphism>), Errors> {
        // we factorize to image of f and from image of f to target object
        let source_object = morphism.source_object();
        let target_object = morphism.target_object();
        let mappings = morphism.arrow_mappings();

        let mut all_target_objects = target_object.get_all_objects().await?.clone();

        let mut image_object = InnerCategory::Object::new().await?;
        let mut image_mapping = HashMap::new();
        // possibility the target object is the image of source object
        let mut target_as_image = false;

        // epic can also be just the target object if all objects are in the image
        for (source_morphism, target_morphism) in mappings {
            if source_morphism.is_identity() {
                if !target_morphism.is_identity() {
                    return Err(Errors::InvalidArrowNoFunctorFound);
                }
                // remove it from all_target_objects if it exist
                if target_as_image && all_target_objects.contains(&target_morphism.target_object())
                {
                    all_target_objects.remove(&target_morphism.target_object());
                } else {
                    target_as_image = false;
                }

                // if object has already been added to image_object
                if let Ok(object) = image_object
                    .get_object(&**target_morphism.target_object())
                    .await
                {
                    let target_morphism = image_object.get_identity_morphism(&**object).await?;
                    image_mapping.insert((*source_morphism).clone(), target_morphism.clone());
                } else {
                    image_object
                        .add_object(target_morphism.target_object().clone())
                        .await?;
                    image_mapping.insert((*source_morphism).clone(), (*target_morphism).clone());
                }
            } else {
                panic!("to implement later")
            }
        }
        // let image_object = if target_as_image && !all_target_objects.is_empty() {
        //     target_object.clone()
        // } else {
        //     let result = Arc::new(image_object);
        //     self.category.add_object(result.clone())?;
        //     result
        // };

        // add the image object to the category
        let image_object = Arc::new(image_object);
        self.category.add_object(image_object.clone()).await?;
        let epic_morphism = Arc::new(InnerCategory::Morphism::new_with_mappings(
            source_object.clone(),
            image_object.clone(),
            image_mapping,
        ));

        // now mapping from image to target object
        let monic_mapping = {
            let mut monic_mapping = HashMap::new();
            for obj in image_object.get_all_objects().await? {
                let source_morphism = image_object.get_identity_morphism(&**obj).await?;
                let target_morphism = target_object.get_identity_morphism(&**obj).await?;
                monic_mapping.insert(source_morphism.clone(), target_morphism.clone());
            }
            monic_mapping
        };

        let monic_morphism = Arc::new(InnerCategory::Morphism::new_with_mappings(
            image_object.clone(),
            target_object.clone(),
            monic_mapping,
        ));
        self.category.add_morphism(epic_morphism.clone()).await?;
        self.category.add_morphism(monic_morphism.clone()).await?;
        Ok((epic_morphism, monic_morphism))
    }
}

#[async_trait]
impl<InnerCategory> CategoryTrait for EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait<
            Morphism = Arrow<
                <InnerCategory as CategoryTrait>::Object,
                <InnerCategory as CategoryTrait>::Object,
            >,
        > + Hash
        + Eq
        + Clone
        + 'static,
    <InnerCategory as CategoryTrait>::Object: Clone,
{
    type Object = InnerCategory::Object;

    type Morphism = Morphism<Self::Object>;

    async fn new() -> Result<Self, Errors> {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        self.category.category_id()
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        self.category.update_category_id(new_id).await
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        self.category.add_object(object).await
    }

    async fn add_morphism(
        &mut self,
        morphism: Arc<Morphism<InnerCategory::Object>>,
    ) -> Result<(), Errors> {
        // here we need to factor it to epic and monic morphisms
        let (epic, monic) = self.factorize(&morphism).await?;
        // self.hash_map.insert(morphism.id().clone(), (epic, monic));
        self.morphism_factors
            .insert(morphism.clone(), (epic.into(), monic.into()));
        self.category.add_morphism(morphism).await
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        self.category.get_object(object).await
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        self.category.get_all_objects().await
    }

    async fn get_all_morphisms(
        &self,
    ) -> Result<HashSet<&Arc<Morphism<InnerCategory::Object>>>, Errors> {
        self.category.get_all_morphisms().await
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Morphism<InnerCategory::Object>>>, Errors> {
        self.category.get_hom_set_x(source_object).await
    }

    async fn get_object_morphisms(
        &self,
        object_id: &Self::Object,
    ) -> Result<Vec<&Arc<Morphism<InnerCategory::Object>>>, Errors> {
        self.category.get_object_morphisms(object_id).await
    }
}

impl<InnerCategory> FactorizationSystemTrait for EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait<
            Morphism = Arrow<
                <InnerCategory as CategoryTrait>::Object,
                <InnerCategory as CategoryTrait>::Object,
            >,
        > + Hash
        + Eq
        + Clone
        + 'static,
    <InnerCategory as CategoryTrait>::Object: Clone,
{
    fn morphism_factors(
        &self,
        morphism: &Morphism<InnerCategory::Object>,
    ) -> Result<
        &(
            Arc<Morphism<InnerCategory::Object>>,
            Arc<Morphism<InnerCategory::Object>>,
        ),
        Errors,
    > {
        self.morphism_factors
            .get(morphism)
            .ok_or(Errors::InvalidFactorization)
    }
}

impl<Object: CategoryTrait + Hash + Eq> Hash for EpicMonicCategory<Object> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.category.hash(state);
    }
}

mod tests {
    use super::*;
    use crate::core::dynamic_category::DynamicCategory;
    use crate::core::traits::category_trait::CategoryFromObjects;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_epic_monic_category() {
        let mut epic_monic_category = EpicMonicCategory::<DynamicCategory>::new().await.unwrap();

        let object_ab: Arc<DynamicCategory> =
            Arc::new(DynamicCategory::from_objects(vec!["a", "b"]).await.unwrap());

        epic_monic_category
            .add_object(object_ab.clone())
            .await
            .expect("Failed to add object A");
    }
}
