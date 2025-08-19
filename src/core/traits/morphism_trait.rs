use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::rc::Rc;

pub trait MorphismTrait: ArrowTrait {
    fn functor(
        &self,
    ) -> Result<
        &Rc<
            impl FunctorTrait<SourceObject = Self::SourceObject, TargetObject = Self::TargetObject>,
        >,
        Errors,
    >;
}
