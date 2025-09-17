use crate::core::arrow::{Arrow, Functor, Morphism};
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
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

    fn new_instance(
        source: Rc<UnitCategory>,
        target: Rc<UnitCategory>,
        id: &str,
        mappings: HashMap<
            std::rc::Rc<Arrow<UnitCategory, UnitCategory>>,
            std::rc::Rc<Arrow<UnitCategory, UnitCategory>>,
        >,
    ) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn new(
        id: String,
        source: Rc<UnitCategory>,
        target: Rc<UnitCategory>,
        mappings: HashMap<
            std::rc::Rc<Arrow<UnitCategory, UnitCategory>>,
            std::rc::Rc<Arrow<UnitCategory, UnitCategory>>,
        >,
    ) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn arrow_id(&self) -> &String {
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

    fn arrow_mappings(
        &self,
    ) -> &HashMap<
        Rc<<UnitCategory as CategoryTrait>::Morphism>,
        Rc<<UnitCategory as CategoryTrait>::Morphism>,
    > {
        todo!()
    }

    fn validate_mappings(&self) -> Result<(), Errors> {
        todo!()
    }
}
