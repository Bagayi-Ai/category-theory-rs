use std::collections::HashMap;
use std::rc::Rc;
use crate::core::dynamic_category::dynamic_category::DynamicCategory;
use crate::core::errors::Errors;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::MorphismAlias;
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;

type DynamicMorphism = DynamicCategory;
type DynamicFunctor = DynamicCategory;

impl ArrowTrait for DynamicCategory {
    type SourceObject = Self;
    type TargetObject = Self;

    fn source_object(&self) -> &Rc<Self::SourceObject> {
        todo!()
    }

    fn target_object(&self) -> &Rc<Self::TargetObject> {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn compose(&self, other: &impl ArrowTrait) -> Result<Self, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Self> {
        todo!()
    }
}

impl MorphismTrait for DynamicCategory {
    fn functor(&self) -> Result<&Rc<Self>, Errors> {
        todo!()
    }
}

impl FunctorTrait for DynamicCategory {
    fn arrow_mappings(&self) -> &HashMap<Rc<MorphismAlias<Self::SourceObject>>, Rc<MorphismAlias<Self::TargetObject>>> {
        todo!()
    }
}