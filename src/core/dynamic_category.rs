use crate::core::arrow::{Arrow, Functor, Morphism};
use crate::core::base_category::BaseCategory;
use crate::core::epic_monic_category::EpicMonicCategory;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use crate::core::traits::factorization_system_trait::FactorizationSystemTrait;
use async_trait::async_trait;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicType {
    Category,
    Functor,
    EpicMonicCategory,
}

pub enum DynamicCategoryEnum {
    Category(
        Box<dyn CategoryTrait<Object = DynamicCategory, Morphism = Morphism<DynamicCategory>>>,
    ),
    EpicMonicCategory(
        Box<
            dyn FactorizationSystemTrait<
                    Object = DynamicCategory,
                    Morphism = Morphism<DynamicCategory>,
                >,
        >,
    ),
}

impl Clone for DynamicCategoryEnum {
    fn clone(&self) -> Self {
        match self {
            DynamicCategoryEnum::Category(cat) => {
                DynamicCategoryEnum::Category(dyn_clone::clone_box(&**cat))
            }
            DynamicCategoryEnum::EpicMonicCategory(cat) => {
                DynamicCategoryEnum::EpicMonicCategory(dyn_clone::clone_box(&**cat))
            }
        }
    }
}

#[derive(Clone)]
pub struct DynamicCategory {
    inner_category: DynamicCategoryEnum,
    dynamic_type: DynamicType,
    functor: Option<Arc<Arrow<DynamicCategory, DynamicCategory>>>,
}

impl Debug for DynamicCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for DynamicCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for DynamicCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for DynamicCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for DynamicCategory {}

impl DynamicCategory {
    pub fn new_with_id(id: ObjectId) -> Self {
        DynamicCategory {
            inner_category: DynamicCategoryEnum::Category(Box::new(BaseCategory::new_with_id(
                id.clone(),
            ))),
            dynamic_type: DynamicType::Category,
            functor: None,
        }
    }

    pub fn new() -> Self {
        Self::new_with_id(ObjectId::Str(uuid::Uuid::new_v4().to_string()))
    }

    pub async fn new_epic_monic_category(id: ObjectId) -> Result<Self, Errors> {
        let epic_monic_category = EpicMonicCategory::<DynamicCategory>::new().await?;
        let mut result = DynamicCategory::new_with_id(id);
        result.dynamic_type = DynamicType::EpicMonicCategory;
        result.inner_category =
            DynamicCategoryEnum::EpicMonicCategory(Box::new(epic_monic_category));
        Ok(result)
    }

    pub fn functor_to_category(
        functor: Arc<Functor<DynamicCategory, DynamicCategory>>,
    ) -> Result<Self, Errors> {
        let mut result = DynamicCategory::new_with_id(functor.arrow_id().clone().into());
        result.dynamic_type = DynamicType::Functor;
        result.functor = Some(functor);
        Ok(result)
    }

    pub fn id(&self) -> &ObjectId {
        &self.inner_category().category_id()
    }

    pub fn dynamic_type(&self) -> &DynamicType {
        &self.dynamic_type
    }

    pub fn inner_category(
        &self,
    ) -> &dyn CategoryTrait<Object = DynamicCategory, Morphism = Morphism<DynamicCategory>> {
        match &self.inner_category {
            DynamicCategoryEnum::Category(cat) => cat.as_ref(),
            DynamicCategoryEnum::EpicMonicCategory(cat) => cat.as_ref(),
        }
    }

    pub fn inner_category_mut(
        &mut self,
    ) -> &mut dyn CategoryTrait<Object = DynamicCategory, Morphism = Morphism<DynamicCategory>>
    {
        match &mut self.inner_category {
            DynamicCategoryEnum::Category(cat) => cat.as_mut(),
            DynamicCategoryEnum::EpicMonicCategory(cat) => cat.as_mut(),
        }
    }

    pub fn inner_factorization_system(
        &self,
    ) -> Option<
        &dyn FactorizationSystemTrait<
            Object = DynamicCategory,
            Morphism = Morphism<DynamicCategory>,
        >,
    > {
        match &self.inner_category {
            DynamicCategoryEnum::EpicMonicCategory(cat) => Some(cat.as_ref()),
            _ => None,
        }
    }
    pub fn expecting_category_type(&self) -> Result<(), Errors> {
        if self.dynamic_type != DynamicType::Category {
            return Err(Errors::InvalidOperation(
                "Expected a category type".to_string(),
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl CategoryTrait for DynamicCategory {
    type Object = DynamicCategory;

    type Morphism = Morphism<DynamicCategory>;

    async fn new() -> Result<Self, Errors>
    where
        Self: Sized,
    {
        Ok(DynamicCategory::new())
    }

    fn category_id(&self) -> &ObjectId {
        self.id()
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        self.inner_category_mut().update_category_id(new_id).await
    }

    async fn add_object(
        &mut self,
        object: Arc<DynamicCategory>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        self.inner_category_mut().add_object(object).await
    }

    async fn add_morphism(&mut self, morphism: Arc<Morphism<Self::Object>>) -> Result<(), Errors> {
        self.inner_category_mut().add_morphism(morphism).await
    }

    async fn get_object(&self, object: &DynamicCategory) -> Result<&Arc<DynamicCategory>, Errors> {
        self.inner_category().get_object(object).await
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<DynamicCategory>>, Errors> {
        self.inner_category().get_all_objects().await
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors> {
        self.inner_category().get_all_morphisms().await
    }

    async fn get_hom_set_x(
        &self,
        source_object: &DynamicCategory,
    ) -> Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors> {
        self.inner_category().get_hom_set_x(source_object).await
    }

    async fn get_object_morphisms(
        &self,
        object: &DynamicCategory,
    ) -> Result<Vec<&Arc<Morphism<Self::Object>>>, Errors> {
        self.inner_category().get_object_morphisms(object).await
    }
}

impl FactorizationSystemTrait for DynamicCategory {
    fn morphism_factors(
        &self,
        morphism: &Morphism<Self::Object>,
    ) -> Result<&(Arc<Morphism<Self::Object>>, Arc<Morphism<Self::Object>>), Errors> {
        if let Some(factorization_system) = &self.inner_factorization_system() {
            factorization_system.morphism_factors(morphism)
        } else {
            Err(Errors::InvalidOperation(
                "This category does not support factorization systems".to_string(),
            ))
        }
    }
}

impl From<ObjectId> for DynamicCategory {
    fn from(value: ObjectId) -> Self {
        DynamicCategory::new_with_id(value)
    }
}

impl From<String> for DynamicCategory {
    fn from(s: String) -> Self {
        DynamicCategory::new_with_id(ObjectId::Str(s))
    }
}

impl From<&str> for DynamicCategory {
    fn from(s: &str) -> Self {
        DynamicCategory::new_with_id(ObjectId::Str(s.to_string()))
    }
}

impl From<i32> for DynamicCategory {
    fn from(n: i32) -> Self {
        DynamicCategory::new_with_id(ObjectId::Int(n))
    }
}

impl From<Arc<DynamicCategory>> for DynamicCategory {
    fn from(rc: Arc<DynamicCategory>) -> Self {
        (*rc).clone()
    }
}
