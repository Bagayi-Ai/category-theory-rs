use crate::core::errors::Errors;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::unit::unit_identifier::UnitIdentifier;
use crate::core::unit::unit_morphism::UnitMorphism;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitCategory {}

impl CategoryTrait for UnitCategory {
    type Identifier = UnitIdentifier;

    type Object = Self;

    type Morphism = UnitMorphism<UnitIdentifier>;

    fn new() -> Self {
        UnitCategory {}
    }

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        todo!()
    }

    fn add_morphism(&mut self, cell: Rc<Self::Morphism>) -> Result<&Rc<Self::Morphism>, Errors> {
        todo!()
    }

    fn get_identity_morphism(
        &self,
        object_id: &Self::Object,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        todo!()
    }

    fn get_object(&self, object: Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_hom_set(
        &self,
        source_object: &Self::Object,
        target_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Object,
    ) -> Result<Vec<&Self::Morphism>, Errors> {
        todo!()
    }

    fn nested_level() -> usize {
        0
    }
}
