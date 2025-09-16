use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::arrow::{Arrow, Morphism, Functor};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Identifier> ArrowTrait<UnitCategory, UnitCategory> for UnitMorphism<T> {
    fn source_object(&self) -> &Rc<UnitCategory> {
        todo!()
    }

    fn target_object(&self) -> &Rc<UnitCategory> {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn arrow_id(&self) -> &String {
        todo!()
    }

    fn compose(&self, other: &impl ArrowTrait<UnitCategory, UnitCategory>) -> Result<Rc<UnitMorphism<T>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&UnitMorphism<T>> {
        todo!()
    }

    fn arrow_mappings(&self) -> &HashMap<Rc<<UnitCategory as CategoryTrait>::Morphism>, Rc<<UnitCategory as CategoryTrait>::Morphism>> {
        todo!()
    }

    fn validate_mappings(&self) -> Result<(), Errors> {
        todo!()
    }
}
