use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;

pub trait MorphismTrait<'a>: ArrowTrait<'a> {

    fn functor(
        &self,
    ) -> Result<
        &dyn FunctorTrait<
            'a,
            Identifier = Self::Identifier,
            SourceObject = Self::SourceObject,
            TargetObject = Self::TargetObject,
        >,
        Errors,
    >;
}
