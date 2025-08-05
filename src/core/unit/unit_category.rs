use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::unit::unit_identifier::UnitIdentifier;
use crate::core::unit::unit_morphism::UnitMorphism;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitCategory {}

impl<'a> NCategory<'a> for UnitCategory {
    type Identifier = UnitIdentifier;

    type Object = Self;

    type Arrow = UnitMorphism<UnitIdentifier>;

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: &'a Self::Object) -> Result<(), NCategoryError> {
        todo!()
    }

    fn add_arrow(&mut self, cell: Self::Arrow) -> Result<Self::Identifier, NCategoryError> {
        todo!()
    }

    fn get_identity_arrow(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Arrow, NCategoryError> {
        todo!()
    }

    fn get_all_object_ids(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        todo!()
    }

    fn get_all_arrows(&self) -> Result<HashSet<&Self::Arrow>, NCategoryError> {
        todo!()
    }

    fn get_object_arrows(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Arrow>, NCategoryError> {
        todo!()
    }

    fn get_arrow(&self, cell_id: &Self::Identifier) -> Result<&Self::Arrow, NCategoryError> {
        todo!()
    }

    fn nested_level() -> usize {
        0
    }
}
