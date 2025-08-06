use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::functor_trait::FunctorTrait;

pub trait ArrowMappingTrait<'a> {
    type Identifier: Identifier;

    type SourceArrow: ArrowTrait<'a>;

    type TargetArrow: ArrowTrait<'a>;

    fn source_arrow(&self) -> &Self::SourceArrow;

    fn target_arrow(&self) -> &Self::TargetArrow;

    fn source_sub_arrow_mapping(
        &self,
    ) -> Result<
        &dyn FunctorTrait<
            'a,
            Self::Identifier,
            <Self::SourceArrow as ArrowTrait<'a>>::SourceObject,
            <Self::SourceArrow as ArrowTrait<'a>>::TargetObject,
        >,
        Errors,
    >;

    fn target_sub_arrow_mapping(
        &self,
    ) -> Result<
        &dyn FunctorTrait<
            'a,
            Self::Identifier,
            <Self::TargetArrow as ArrowTrait<'a>>::SourceObject,
            <Self::TargetArrow as ArrowTrait<'a>>::TargetObject,
        >,
        Errors,
    >;
}
