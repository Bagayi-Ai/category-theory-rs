use crate::DB;
use crate::core::arrow::Morphism;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::errors::Errors;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::string::ToString;
use std::sync::Arc;
use surrealdb::RecordId;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PersistableCategory {
    category_id: ObjectId,
    #[serde(skip)]
    is_category: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryObject {
    object_id: ObjectId,
    category: Thing,
    is_category: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_category: Option<Thing>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableArrow {
    arrow_id: String,
    source: Thing,
    target: Thing,
    category: Thing,
    is_identity: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

impl PersistableCategory {
    const TABLE_NAME: &'static str = "category";

    const OBJECT_TABLE_NAME: &'static str = "object";

    const MORPHISM_TABLE_NAME: &'static str = "morphism";

    const FUNCTOR_TABLE_NAME: &'static str = "functor";

    pub async fn new() -> Result<Self, surrealdb::Error> {
        let category = PersistableCategory {
            category_id: ObjectId::Str(uuid::Uuid::new_v4().to_string()),
            is_category: true,
        };
        category.persist().await?;
        Ok(category)
    }

    pub async fn persist(&self) -> Result<(), surrealdb::Error> {
        let record: Option<Record> = DB.create(self.resource()).content(self.clone()).await?;
        dbg!(record);
        Ok(())
    }

    fn resource(&self) -> (String, String) {
        (
            Self::TABLE_NAME.to_string(),
            self.category_id.clone().to_string(),
        )
    }

    fn morphism_resource(&self, morphism: &Morphism<PersistableCategory>) -> (String, String) {
        (
            Self::MORPHISM_TABLE_NAME.to_string(),
            morphism.arrow_id().clone(),
        )
    }
}

#[async_trait]
impl CategoryTrait for PersistableCategory {
    type Object = PersistableCategory;
    type Morphism = Morphism<PersistableCategory>;

    async fn new() -> Result<Self, Errors>
    where
        Self: Sized,
    {
        Ok(PersistableCategory::new().await.unwrap())
    }

    fn category_id(&self) -> &ObjectId {
        &self.category_id
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        // updating category id should result in a new record in the database
        self.category_id = new_id;
        self.persist().await.unwrap();
        Ok(())
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        let record: Option<Record> = DB
            .create(Self::OBJECT_TABLE_NAME)
            .content(PersistableCategoryObject {
                object_id: object.category_id.clone(),
                category: Thing::from(self.resource()),
                is_category: object.is_category,
                reference_category: if object.is_category {
                    Some(Thing::from(object.resource()))
                } else {
                    None
                },
            })
            .await
            .unwrap();

        let morphism = Morphism::new_identity(object.clone());
        self.add_morphism(morphism.clone()).await?;
        Ok(morphism)
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        let record: Option<Record> = DB
            .create(Self::MORPHISM_TABLE_NAME)
            .content(PersistableArrow {
                arrow_id: morphism.arrow_id().to_string(),
                source: Thing::from(morphism.source_object().resource()),
                target: Thing::from(morphism.target_object().resource()),
                category: Thing::from(self.resource()),
                is_identity: morphism.is_identity(),
            })
            .await
            .unwrap();
        dbg!(record);

        // if it's not identity morphism there is a functor that needs to be created
        if let Some(functor) = morphism.get_functor() {
            let arrow_mappings = functor.arrow_mappings();
            if !arrow_mappings.is_empty() {
                let record: Option<Record> = DB
                    .create(Self::FUNCTOR_TABLE_NAME)
                    .content(
                        functor
                            .arrow_mappings()
                            .iter()
                            .map(|(source_morphism, target_morphism)| PersistableArrow {
                                arrow_id: functor.arrow_id().clone(),
                                source: Thing::from(self.morphism_resource(&*source_morphism)),
                                target: Thing::from(self.morphism_resource(&*target_morphism)),
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
        todo!()
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        todo!()
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        todo!()
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        todo!()
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Self::Morphism>>, Errors> {
        todo!()
    }
}

impl From<ObjectId> for PersistableCategory {
    fn from(value: ObjectId) -> Self {
        PersistableCategory {
            category_id: value,
            is_category: false,
        }
    }
}

impl From<String> for PersistableCategory {
    fn from(s: String) -> Self {
        ObjectId::Str(s).into()
    }
}

impl From<&str> for PersistableCategory {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[tokio::test]
    async fn test_persistable_category() {
        crate::init_db().await.unwrap();
        let mut category = PersistableCategory::new().await.unwrap();

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

        let mut category2 = PersistableCategory::new().await.unwrap();
        // create same object as in category 2 to make sure they are independent
        category2
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();

        // // now create a higher category that contains the two previous categories as objects
        let mut higher_category = PersistableCategory::new().await.unwrap();
        higher_category
            .add_object(Arc::new(category))
            .await
            .unwrap();
        higher_category
            .add_object(Arc::new(category2))
            .await
            .unwrap();
    }
}
