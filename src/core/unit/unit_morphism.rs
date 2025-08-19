use crate::core::category::Category;
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::unit::unit_category::UnitCategory;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Identifier> ArrowTrait for UnitMorphism<T> {
    type SourceObject = UnitCategory;
    type TargetObject = Self::SourceObject;

    fn source_object(&self) -> &Rc<Self::SourceObject> {
        todo!()
    }

    fn target_object(&self) -> &Rc<Self::TargetObject> {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject = Self::SourceObject, TargetObject = Self::TargetObject>,
    ) -> Result<UnitMorphism<T>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&UnitMorphism<T>> {
        todo!()
    }
}

impl<T: Identifier> MorphismTrait for UnitMorphism<T> {
    fn functor(&self) -> Result<&Rc<UnitMorphism<T>>, Errors> {
        todo!()
    }
}

impl<Id: Identifier> FunctorTrait for UnitMorphism<Id> {
    fn arrow_mappings(
        &self,
    ) -> &HashMap<Rc<MorphismAlias<Self::SourceObject>>, Rc<MorphismAlias<Self::TargetObject>>>
    {
        todo!()
    }
}
