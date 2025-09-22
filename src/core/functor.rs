use crate::core::arrow::Morphism;
use crate::core::errors::Errors;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    id: ObjectId,
    source_category: Arc<SourceCategory>,
    target_category: Arc<TargetCategory>,
    is_identity: bool,
    morphism_mapping: HashMap<Arc<SourceCategory::Morphism>, Arc<TargetCategory::Morphism>>,
}

impl<SourceCategory, TargetCategory> PartialEq for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<SourceCategory, TargetCategory> Eq for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
}

impl<SourceCategory, TargetCategory> Clone for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<SourceCategory, TargetCategory> Debug for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<SourceCategory, TargetCategory> Hash for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<SourceCategory, TargetCategory> Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    pub fn new(
        id: String,
        source_category: Arc<SourceCategory>,
        target_category: Arc<TargetCategory>,
        morphism_mapping: HashMap<Arc<SourceCategory::Morphism>, Arc<TargetCategory::Morphism>>,
    ) -> Self {
        Functor {
            id: ObjectId::Str(id),
            source_category,
            target_category,
            morphism_mapping,
            is_identity: false,
        }
    }
}

impl<SourceCategory, TargetCategory> ArrowTrait<SourceCategory, TargetCategory>
    for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn source_object(&self) -> &Arc<SourceCategory> {
        &self.source_category
    }

    fn target_object(&self) -> &Arc<TargetCategory> {
        &self.target_category
    }

    fn new_instance(
        source: Arc<SourceCategory>,
        target: Arc<TargetCategory>,
        id: &str,
        mappings: HashMap<Arc<SourceCategory::Morphism>, Arc<TargetCategory::Morphism>>,
    ) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn new(
        id: String,
        source: Arc<SourceCategory>,
        target: Arc<TargetCategory>,
        mappings: HashMap<Arc<SourceCategory::Morphism>, Arc<TargetCategory::Morphism>>,
    ) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn is_identity(&self) -> bool {
        self.is_identity
    }

    fn arrow_id(&self) -> &String {
        match &self.id {
            ObjectId::Str(s) => s,
            _ => panic!("Arrow ID is not a string"),
        }
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceCategory, TargetCategory>,
    ) -> Result<Arc<Functor<SourceCategory, TargetCategory>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Functor<SourceCategory, TargetCategory>> {
        todo!()
    }

    fn functor(&self) -> Option<&Functor<SourceCategory, TargetCategory>> {
        None
    }
}

impl<SourceCategory, TargetCategory> FunctorTrait<SourceCategory, TargetCategory>
    for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn morphisms_mappings(
        &self,
    ) -> &HashMap<Arc<SourceCategory::Morphism>, Arc<TargetCategory::Morphism>> {
        &self.morphism_mapping
    }
}

#[async_trait]
impl<SourceCategory, TargetCategory> CategoryTrait for Functor<SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait + Eq + Hash + Clone + Sync + Send,
{
    type Object = TargetCategory;
    type Morphism = Morphism<Self::Object>;

    async fn new() -> Result<Self, Errors>
    where
        Self: Sized,
    {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        &self.id
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        todo!()
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        todo!()
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        todo!()
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
