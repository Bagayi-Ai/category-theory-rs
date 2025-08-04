use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::core::discrete_category::DiscreteCategory;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::unit::unit_morphism::UnitMorphism;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::unit::unit_identifier::UnitIdentifier;
use crate::core::unit::unit_functor::UnitFunctor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitCategory<T: Identifier>
{
    pub category_id: T,
}

impl<'a, T: Identifier + 'a> NCategory<'a> for UnitCategory<T> {
    type Identifier = T;

    type Object = Self;

    type Morphism = UnitMorphism<T>;

    fn category_id(&self) -> Self::Identifier {
        self.category_id.clone()
    }

    fn add_object(&mut self, object: &'a Self::Object) -> Result<(), NCategoryError> {
        todo!()
    }

    fn add_morphism(&mut self, cell: Self::Morphism) -> Result<Self::Identifier, NCategoryError> {
        todo!()
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<&Self::Object, NCategoryError> {
        todo!()
    }

    fn get_identity_morphism(
        &self,
        object_id: &Self::Object,
    ) -> Result<&Self::Morphism, NCategoryError> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::Object>, NCategoryError> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Object,
    ) -> Result<Vec<&Self::Morphism>, NCategoryError> {
        todo!()
    }

    fn get_moprhism(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError> {
        todo!()
    }


    fn nested_level() -> usize {
        0
    }
}


impl <'a, T: Identifier + 'a> Morphism<'a> for UnitCategory<T>
{
    type Object = Self;
    type Identifier = T;
    type Functor = UnitFunctor<'a, T, Self, Self>;

    fn cell_id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn source_object(&self) -> &Self::Object {
        &self
    }

    fn target_object(&self) -> &Self::Object {
        &self
    }

    fn is_identity(&self) -> bool {
        true
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> From<T> for UnitCategory<T> {
    fn from(object: T) -> Self {
        UnitCategory{category_id: object}
    }
}