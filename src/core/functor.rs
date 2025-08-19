use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Functor<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    id: Id,
    source_category: Rc<SourceCategory>,
    target_category: Rc<TargetCategory>,
    mappings: HashMap<Rc<SourceCategory::Morphism>, Rc<TargetCategory::Morphism>>,
}

impl<Id, SourceCategory, TargetCategory> Functor<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    pub fn new(
        id: Id,
        source_category: Rc<SourceCategory>,
        target_category: Rc<TargetCategory>,
        mappings: HashMap<Rc<SourceCategory::Morphism>, Rc<TargetCategory::Morphism>>,
    ) -> Self {
        Functor {
            id,
            source_category,
            target_category,
            mappings,
        }
    }
}

impl<'a, Id, SourceCategory, TargetCategory> ArrowTrait
    for Functor<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    type SourceObject = SourceCategory;
    type TargetObject = TargetCategory;

    fn source_object(&self) -> &Rc<Self::SourceObject> {
        &self.source_category
    }

    fn target_object(&self) -> &Rc<Self::TargetObject> {
        &self.target_category
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject = Self::SourceObject, TargetObject = Self::TargetObject>,
    ) -> Result<Functor<Id, Self::SourceObject, Self::TargetObject>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Functor<Id, Self::SourceObject, Self::TargetObject>> {
        todo!()
    }
}

impl<Id, SourceCategory, TargetCategory> FunctorTrait
    for Functor<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn arrow_mappings(
        &self,
    ) -> &HashMap<Rc<SourceCategory::Morphism>, Rc<TargetCategory::Morphism>> {
        &self.mappings
    }
}

impl<Id, SourceCategory, TargetCategory> Hash for Functor<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source_category.hash(state);
        self.target_category.hash(state);
        self.id.hash(state);
    }
}
