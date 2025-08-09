use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait, ChildObjectAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::unit::unit_category::UnitCategory;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> ArrowTrait<'a> for UnitMorphism<T> {
    type SourceObject = UnitCategory;
    type TargetObject = Self::SourceObject;
    type Identifier = T;

    fn arrow_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object(&self) -> &Self::SourceObject {
        todo!()
    }

    fn target_object(&self) -> &Self::TargetObject {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn arrows(&self) -> Vec<&UnitMorphism<T>> {
        todo!()
    }
}

impl<'a, T: Identifier + 'a> MorphismTrait<'a> for UnitMorphism<T> {
    fn functor(&self) -> Result<&UnitMorphism<T>, Errors> {
        todo!()
    }
}
