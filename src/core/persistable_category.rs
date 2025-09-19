use crate::DB;
use crate::core::arrow::Morphism;
use crate::core::errors::Errors;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::string::ToString;
use std::sync::Arc;
use surrealdb::RecordId;
use surrealdb::sql::Thing;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::traits::functor_trait::FunctorTrait;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq,
{
    category: InnerCategory
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryResource {
    category_id: ObjectId,
}

impl PersistableCategoryResource {

    const TABLE_NAME: &'static str = "category";

    fn resource<Category: CategoryTrait>(category: &Category) -> (String, String) {
        (
            Self::TABLE_NAME.to_string(),
            category.category_id().clone().to_string(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryObject {
    object_id: ObjectId,
    category: Thing,
}

impl PersistableCategoryObject
{
    const TABLE_NAME: &'static str = "object";
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableArrow {
    arrow_id: String,
    source: Thing,
    target: Thing,
    category: Thing,
    is_identity: bool,
}

impl PersistableArrow
{
    const MORPHISM_TABLE_NAME: &'static str = "morphism";

    const FUNCTOR_TABLE_NAME: &'static str = "functor";

    fn morphism_resource<Category: CategoryTrait, Morphism: ArrowTrait<Category, Category>>(morphism: &Morphism) -> (String, String) {
        (
            Self::MORPHISM_TABLE_NAME.to_string(),
            morphism.arrow_id().clone(),
        )
    }

    fn functor_resource<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(functor: &Functor) -> (String, String) {
        (
            Self::FUNCTOR_TABLE_NAME.to_string(),
            functor.arrow_id().clone(),
        )
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

impl <InnerCategory> PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq
{

    pub async fn new() -> Result<Self, Errors> {
        let category = PersistableCategory {
            category: InnerCategory::new().await?,
        };
        category.persist().await?;
        Ok(category)
    }

    pub async fn persist(&self) -> Result<(), Errors> {
        let record: Option<Record> = DB.create(self.resource()).content(PersistableCategoryResource{
            category_id: self.category.category_id().clone(),
        })
            .await.map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(record);
        Ok(())
    }

    fn resource(&self) -> (String, String) {
        (
            PersistableCategoryResource::TABLE_NAME.to_string(),
            self.category.category_id().clone().to_string(),
        )
    }

    pub fn inner_category(&self) -> &InnerCategory {
        &self.category
    }
}

#[async_trait]
impl<InnerCategory> CategoryTrait for PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone,
{
    type Object = InnerCategory::Object;
    type Morphism = InnerCategory::Morphism;

    async fn new() -> Result<Self, Errors>
    where
        Self: Sized,
    {
        Ok(PersistableCategory::new().await.unwrap())
    }

    fn category_id(&self) -> &ObjectId {
        &self.category.category_id()
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        // updating category id should result in a new record in the database
        self.category.update_category_id(new_id).await?;
        self.persist().await.unwrap();
        Ok(())
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        // create object in the inner category before persisting
        let morphism_created = self.category.add_object(object.clone()).await?;

        // now persist the object
        let record: Option<Record> = DB
            .create(PersistableCategoryObject::TABLE_NAME)
            .content(PersistableCategoryObject {
                object_id: object.category_id().clone(),
                category: Thing::from(self.resource()),
            })
            .await
            .unwrap();
        dbg!(record);

        Ok(morphism_created)
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        // create morphism in the inner category before persisting
        self.category.add_morphism(morphism.clone()).await?;
        let record: Option<Record> = DB
            .create(PersistableArrow::MORPHISM_TABLE_NAME)
            .content(PersistableArrow {
                arrow_id: morphism.arrow_id().to_string(),
                source: Thing::from(PersistableCategoryResource::resource(&**morphism.source_object())),
                target: Thing::from(PersistableCategoryResource::resource(&**morphism.target_object())),
                category: Thing::from(self.resource()),
                is_identity: morphism.is_identity(),
            })
            .await
            .unwrap();
        dbg!(record);

        // if it's not identity morphism there is a functor that needs to be created
        if let Some(functor) = morphism.functor() {
            let morphism_mapping = functor.morphisms_mappings();
            if !morphism_mapping.is_empty() {
                let record: Option<Record> = DB
                    .create(PersistableArrow::FUNCTOR_TABLE_NAME)
                    .content(
                        functor
                            .morphisms_mappings()
                            .iter()
                            .map(|(source_morphism, target_morphism)| PersistableArrow {
                                arrow_id: functor.arrow_id().clone(),
                                source: Thing::from(PersistableArrow::morphism_resource(&**source_morphism)),
                                target: Thing::from(PersistableArrow::morphism_resource(&**target_morphism)),
                                category: Thing::from(self.resource()),
                                is_identity: false,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .await
                    .unwrap();
                dbg!(record);
            }
        }
        Ok(())
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        self.category.get_object(object).await
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        self.category.get_all_objects().await
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        self.category.get_all_morphisms().await
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        self.category.get_hom_set_x(source_object).await
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Self::Morphism>>, Errors> {
        self.category.get_object_morphisms(object).await
    }
}

impl<InnerCategory> From<String> for PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + From<String>,
{
    fn from(s: String) -> Self {
        PersistableCategory{
            category: s.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::core::dynamic_category::DynamicCategory;

    #[tokio::test]
    async fn test_persistable_category() {
        crate::init_db().await.unwrap();
        let mut category: PersistableCategory<DynamicCategory> = PersistableCategory::new().await.unwrap();

        let identity_morphism1 = category
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();
        let identity_morphism2 = category
            .add_object(Arc::new("TestObject2".into()))
            .await
            .unwrap();

        // add morphism between the two objects
        let morphism = Arc::new(Morphism::new_with_mappings(
            identity_morphism1.source_object().clone(),
            identity_morphism2.source_object().clone(),
            HashMap::new(),
        ));
        category.add_morphism(morphism).await.unwrap();

        // try get the first object
        let obj = category
            .get_object(&"TestObject".into())
            .await;
        assert!(obj.is_ok());
        let obj = obj.unwrap();
        assert_eq!(obj.category_id(), &ObjectId::Str("TestObject".to_string()));

        let mut category2: PersistableCategory<DynamicCategory> = PersistableCategory::new().await.unwrap();
        // create same object as in category 2 to make sure they are independent
        category2
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();

        // now create a higher category that contains the two previous categories as objects
        let mut higher_category: PersistableCategory<DynamicCategory> = PersistableCategory::new().await.unwrap();
        higher_category
            .add_object(Arc::new(category.inner_category().clone()))
            .await
            .unwrap();
        higher_category
            .add_object(Arc::new(category2.inner_category().clone()))
            .await
            .unwrap();
    }
}
