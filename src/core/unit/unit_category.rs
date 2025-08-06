use crate::core::errors::Errors;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::unit::unit_functor::UnitFunctor;
use crate::core::unit::unit_identifier::UnitIdentifier;
use crate::core::unit::unit_morphism::UnitMorphism;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitCategory {}

impl<'a> CategoryTrait<'a> for UnitCategory {
    type Identifier = UnitIdentifier;

    type Object = Self;

    type Morphism = UnitMorphism<UnitIdentifier>;

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn identity_endofunctor(&self) -> &UnitFunctor {
        todo!()
    }

    fn add_object(&mut self, object: &'a Self::Object) -> Result<(), Errors> {
        todo!()
    }

    fn add_morphism(&mut self, cell: Self::Morphism) -> Result<Self::Identifier, Errors> {
        todo!()
    }

    fn get_identity_morphism(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Morphism, Errors> {
        todo!()
    }

    fn get_all_object_ids(&self) -> Result<HashSet<&Self::Identifier>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Morphism>, Errors> {
        todo!()
    }

    fn get_moprhism(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, Errors> {
        todo!()
    }

    fn nested_level() -> usize {
        0
    }
}
