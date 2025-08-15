use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ProductObject<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    id: Id,
    source_category: Rc<SourceCategory>,
    target_category: Rc<TargetCategory>,
}

impl<Id, SourceCategory, TargetCategory> ProductObject<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    pub fn new(
        id: Id,
        source_category: Rc<SourceCategory>,
        target_category: Rc<TargetCategory>,
    ) -> Self {
        ProductObject::<Id, SourceCategory, TargetCategory> {
            id,
            source_category,
            target_category,
        }
    }

    pub fn source_category(&self) -> &Rc<SourceCategory> {
        &self.source_category
    }

    pub fn target_category(&self) -> &Rc<TargetCategory> {
        &self.target_category
    }
}

impl<Id, SourceCategory, TargetCategory> CategoryTrait
    for ProductObject<Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait,
    TargetCategory: CategoryTrait,
{
    type Identifier = Id;
    type Object = Self;
    type Morphism = Morphism<Id, Self>;

    fn new() -> Self {
        todo!()
    }

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        todo!()
    }

    fn add_morphism(&mut self, morphism: Rc<Self::Morphism>) -> Result<Self::Identifier, Errors> {
        todo!()
    }

    fn get_identity_morphism(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Morphism>, Errors> {
        todo!()
    }

    fn get_moprhism(&self, morphism_id: &Self::Identifier) -> Result<&Rc<Self::Morphism>, Errors> {
        todo!()
    }
}
