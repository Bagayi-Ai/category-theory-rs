use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::unit::unit_category::UnitCategory;
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

    fn compose(
        &self,
        other: &impl ArrowTrait<UnitCategory, UnitCategory>,
    ) -> Result<Rc<UnitMorphism<T>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&UnitMorphism<T>> {
        todo!()
    }
}

impl<T: Identifier> MorphismTrait<UnitCategory> for UnitMorphism<T> {
    fn functor(&self) -> Result<&Rc<UnitMorphism<T>>, Errors> {
        todo!()
    }
}

impl<Id: Identifier> FunctorTrait<UnitCategory, UnitCategory> for UnitMorphism<Id> {
    fn new(
        source_category: Rc<UnitCategory>,
        target_category: Rc<UnitCategory>,
        mappings: HashMap<Rc<Morphism<UnitCategory>>, Rc<Morphism<UnitCategory>>>,
    ) -> Result<Self, Errors>
    where
        Self: Sized,
    {
        todo!()
    }

    fn source_category(&self) -> &Rc<UnitCategory> {
        todo!()
    }

    fn target_category(&self) -> &Rc<UnitCategory> {
        todo!()
    }

    fn arrow_mappings(&self) -> &HashMap<Rc<Morphism<UnitCategory>>, Rc<Morphism<UnitCategory>>> {
        todo!()
    }
}
