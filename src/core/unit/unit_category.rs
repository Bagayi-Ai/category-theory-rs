use crate::core::errors::Errors;
use crate::core::object_id::ObjectId;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::unit::unit_identifier::UnitIdentifier;
use crate::core::unit::unit_morphism::UnitMorphism;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use crate::core::arrow::Morphism;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitCategory {}

impl CategoryTrait for UnitCategory {
    type Object = UnitCategory;
    
    type Morphism = Morphism<Self::Object>;

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        todo!()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        todo!()
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Morphism<Self::Object>>,
    ) -> Result<&Rc<Morphism<Self::Object>>, Errors> {
        todo!()
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        todo!()
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Morphism<Self::Object>>>, Errors> {
        todo!()
    }

    fn nested_level() -> usize {
        0
    }
}
