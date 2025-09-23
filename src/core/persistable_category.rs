use crate::DB;
use crate::core::arrow::Morphism;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::string::ToString;
use std::sync::Arc;
use surrealdb::RecordId;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq,
{
    category: InnerCategory,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryObject {
    object_id: ObjectId,
}

impl PersistableCategoryObject {
    const TABLE_NAME: &'static str = "object";

    fn object_thing<Category: CategoryTrait>(category: &Category) -> Thing {
        Thing::from((
            Self::TABLE_NAME.to_string(),
            category.category_id().to_string(),
        ))
    }

    fn resource(&self) -> (String, String) {
        (
            Self::TABLE_NAME.to_string(),
            self.object_id.to_string(),
        )
    }

    pub async fn persist(&self) -> Result<(), Errors> {
        let record: Option<Record> = DB
            .create(self.resource())
            .content(PersistableCategoryObject {
                object_id: self.object_id.clone(),
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(record);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableArrow {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RecordId>,
    #[serde(rename = "in")]
    source: Thing,
    #[serde(rename = "out")]
    target: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    functor: Option<Thing>,
    category: Thing,
    is_identity: bool,
}

impl PersistableArrow {
    const MORPHISM_TABLE_NAME: &'static str = "morphism";

    const FUNCTOR_TABLE_NAME: &'static str = "functor";

    const ARROW_MAPPING_TABLE_NAME: &'static str = "arrow_mapping";

    fn morphism_resource<Category: CategoryTrait, Morphism: ArrowTrait<Category, Category>>(
        morphism: &Morphism,
    ) -> (String, String) {
        (
            Self::MORPHISM_TABLE_NAME.to_string(),
            morphism.arrow_id().clone(),
        )
    }

    fn functor_resource<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> (String, String) {
        (
            Self::FUNCTOR_TABLE_NAME.to_string(),
            functor.arrow_id().clone(),
        )
    }

    fn functor_thing<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> Thing {
        Thing::from(Self::functor_resource(functor))
    }

    async fn create_functor<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> Result<Thing, Errors> {
        let functor_record: Option<Record> = DB
            .create(PersistableArrow::functor_resource(functor))
            .content(PersistableCategoryFunctor {
                id: None,
                source: PersistableCategoryObject::object_thing(&**functor.source_object()),
                target: PersistableCategoryObject::object_thing(&**functor.target_object()),
                is_identity: functor.is_identity(),
            })
            .await?;
        dbg!(&functor_record);

        let record: Option<Record> = DB
            .create(PersistableArrow::ARROW_MAPPING_TABLE_NAME)
            .content(
                functor
                    .morphisms_mappings()
                    .iter()
                    .map(
                        |(source_morphism, target_morphism)| PersistableCategoryFunctorMapping {
                            functor_id: Thing::from(PersistableArrow::functor_resource(functor)),
                            source_morphism: Thing::from(PersistableArrow::morphism_resource(
                                &**source_morphism,
                            )),
                            target_morphism: Thing::from(PersistableArrow::morphism_resource(
                                &**target_morphism,
                            )),
                        },
                    )
                    .collect::<Vec<_>>(),
            )
            .await?;
        dbg!(record);
        Ok(Thing::from(Self::functor_resource(functor)))
    }

    async fn create_morphism<Category: CategoryTrait, Morphism: ArrowTrait<Category, Category>>(
        morphism: &Morphism,
        category: &Category,
    ) -> Result<Thing, Errors> {
        // create morphism using relate query as using create relations are not visible

        let sql = r#"
            RELATE $src->$rel_id->$dst
            SET category = $category,
                is_identity = $is_identity,
                functor = $functor,
                created_at = time::now(),
                weight = $weight;
        "#;
        // let res =
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryFunctor {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RecordId>,
    #[serde(rename = "in")]
    source: Thing,
    #[serde(rename = "out")]
    target: Thing,
    is_identity: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryFunctorMapping {
    functor_id: Thing,
    source_morphism: Thing,
    target_morphism: Thing,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

impl<InnerCategory> PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone,
{
    pub async fn new() -> Result<Self, Errors> {
        let category = PersistableCategory {
            category: InnerCategory::new().await?,
        };
        let object = PersistableCategoryObject {
            object_id: category.category.category_id().clone(),
        };
        object.persist().await?;
        Ok(category)
    }

    pub fn inner_category(&self) -> &InnerCategory {
        &self.category
    }

    fn thing(&self) -> Thing {
        PersistableCategoryObject::object_thing(&self.category)
    }

    async fn create_record<Category: CategoryTrait>(
        &self,
        object: &Category,
    ) -> Result<(), Errors> {
        // now persist the object
        let sql = r#"
        LET $object = (UPSERT type::thing($table_name, $id));
        RELATE $object ->object_in-> $category_id;
        RETURN $object;
        "#;

        let response = DB
            .query(sql)
            .bind(("table_name", PersistableCategoryObject::TABLE_NAME))
            .bind(("id", object.category_id().to_string()))
            .bind(("category_id", self.thing()))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(response);
        Ok(())
    }

    async fn create_morphism(&self, morphism: &InnerCategory::Morphism) -> Result<(), Errors> {
        let sql = r#"
        RELATE $src->$rel_id->$dst
        SET category = $category,
            is_identity = $is_identity,
            created_at = time::now()
        RETURN *
        "#;

        let response = DB
            .query(sql)
            .bind(("rel_id", Self::arrow_thing(&*morphism)))
            .bind(("src", PersistableCategoryObject::object_thing(&**morphism.source_object())))
            .bind(("dst", PersistableCategoryObject::object_thing(&**morphism.target_object())))
            .bind(("category", self.category.category_id().to_string()))
            .bind(("is_identity", morphism.is_identity()))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(response);
        Ok(())
    }

    fn arrow_thing(morphism: &InnerCategory::Morphism) -> Thing {
        Thing::from(PersistableArrow::morphism_resource(morphism))
    }

    fn functor_thing<Functor: FunctorTrait<InnerCategory, InnerCategory>>(
        functor: &Functor,
    ) -> Thing {
        Thing::from(PersistableArrow::functor_resource(functor))
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
        PersistableCategoryObject {
            object_id: self.category_id().clone(),
        }.persist().await?;
        Ok(())
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        self.create_record(&*object).await?;
        Ok(self.category.add_object(object.clone()).await?)
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        // create morphism in the inner category before persisting
        self.category.add_morphism(morphism.clone()).await?;

        // if functor is not none and does not exist in the database create it
        if let Some(functor) = morphism.functor() {
            let functor_record: Option<serde_json::Value> = DB
                .select(PersistableArrow::functor_resource(functor))
                .await?;
            dbg!(&functor_record);
            if functor_record.is_none() {
                // if its none create it.
                PersistableArrow::create_functor(functor).await?;
            }
        }

        let record = self.create_morphism(&*morphism).await?;
        dbg!(record);
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
        PersistableCategory { category: s.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dynamic_category::DynamicCategory;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_persistable_category() {
        crate::init_db().await.unwrap();
        let mut category: PersistableCategory<DynamicCategory> =
            PersistableCategory::new().await.unwrap();

        let identity_morphism1 = category
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();
        let identity_morphism2 = category
            .add_object(Arc::new("TestObject2".into()))
            .await
            .unwrap();

        // add morphism between the two objects
        let morphism = Arc::new(Morphism::new(
            String::generate(),
            identity_morphism1.source_object().clone(),
            identity_morphism2.source_object().clone(),
            None,
        ));
        category.add_morphism(morphism).await.unwrap();

        // try get the first object
        let obj = category.get_object(&"TestObject".into()).await;
        assert!(obj.is_ok());
        let obj = obj.unwrap();
        assert_eq!(obj.category_id(), &ObjectId::Str("TestObject".to_string()));

        let mut category2: PersistableCategory<DynamicCategory> =
            PersistableCategory::new().await.unwrap();
        // create same object as in category 2 to make sure they are independent
        category2
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();

        // now create a higher category that contains the two previous categories as objects
        let mut higher_category: PersistableCategory<DynamicCategory> =
            PersistableCategory::new().await.unwrap();
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
