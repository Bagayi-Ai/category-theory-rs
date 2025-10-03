use crate::DB;
use crate::core::arrow::{Arrow, Morphism};
use crate::core::epic_monic_category::EpicMonicCategory;
use crate::core::errors::Errors;
use crate::core::object_id::ObjectId;
use crate::core::persistable_category::{PersistableCategory, PersistableCategoryObject};
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::factorization_system_trait::FactorizationSystemTrait;
use async_trait::async_trait;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use surrealdb::opt::auth::Record;
use surrealdb::rpc::format::cbor::res;
use surrealdb::sql::Thing;

#[derive(Debug, Clone)]
pub struct PersistableFactorizationCategory<InnerCategory>
where
    InnerCategory: FactorizationSystemTrait + Clone + Hash + Eq,
{
    category: PersistableCategory<InnerCategory>,
}

impl<InnerCategory> PersistableFactorizationCategory<InnerCategory>
where
    InnerCategory: FactorizationSystemTrait + Clone + Hash + Eq,
{
    const IsFactorizationSystemColumn: &'static str = "IsFactorizationSystem";

    fn resource(&self) -> (String, String) {
        (
            PersistableCategoryObject::TABLE_NAME.to_string(),
            self.category.category_id().to_string(),
        )
    }

    pub async fn persist(&self) -> Result<(), Errors> {
        let record: Option<serde_json::Value> = DB
            .update(self.resource())
            .merge(serde_json::json!({
                Self::IsFactorizationSystemColumn: true,
            }))
            .await?;
        Ok(())
    }

    pub async fn persist_morphism_factor(
        &self,
        morphism: InnerCategory::Morphism,
        left_morphism: InnerCategory::Morphism,
        right_morphism: InnerCategory::Morphism,
    ) -> Result<(), Errors> {
        let sql = r#"
        RELATE $morphism -> left_factor -> $left_morphism
            Set category = $category;
        RELATE $morphism -> right_factor -> $right_morphism
            Set category = $category;
        "#;

        let response = DB
            .query(sql)
            .bind((
                "morphism",
                PersistableCategory::<InnerCategory>::arrow_thing(&morphism),
            ))
            .bind((
                "left_morphism",
                PersistableCategory::<InnerCategory>::arrow_thing(&left_morphism),
            ))
            .bind((
                "right_morphism",
                PersistableCategory::<InnerCategory>::arrow_thing(&right_morphism),
            ))
            .bind(("category", self.category.thing()))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(&response);

        Ok(())
    }
}

#[async_trait]
impl<InnerCategory> CategoryTrait for PersistableFactorizationCategory<InnerCategory>
where
    InnerCategory: FactorizationSystemTrait + Clone + Hash + Eq,
{
    type Object = InnerCategory::Object;
    type Morphism = InnerCategory::Morphism;

    async fn new() -> Result<Self, Errors>
    where
        Self: Sized,
    {
        let persistable_category = PersistableCategory::new().await?;
        let result = Self {
            category: persistable_category,
        };
        result.persist().await?;
        Ok(result)
    }

    fn category_id(&self) -> &ObjectId {
        self.category.category_id()
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        self.category.add_object(object).await
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        self.category.add_morphism(morphism.clone()).await?;
        let (epic, monic) = self.category.inner_category().morphism_factors(&morphism)?;

        // now persist the epic and monic morphisms

        Ok(())
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        self.get_object(object).await
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        self.category.get_all_objects().await
    }

    async fn get_morphism(&self, morphism_id: &String) -> Result<&Arc<Self::Morphism>, Errors> {
        self.category.get_morphism(morphism_id).await
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

impl<InnerCategory> FactorizationSystemTrait for PersistableFactorizationCategory<InnerCategory>
where
    InnerCategory: FactorizationSystemTrait + Clone + Hash + Eq,
{
    fn morphism_factors(
        &self,
        morphism: &Self::Morphism,
    ) -> Result<&(Arc<Self::Morphism>, Arc<Self::Morphism>), Errors> {
        self.category.inner_category().morphism_factors(morphism)
    }
}
