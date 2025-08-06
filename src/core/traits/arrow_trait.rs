use crate::core::discrete_category::DiscreteCategory;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, NCategoryError};
use crate::core::unit::unit_category::UnitCategory;
use std::fmt::{Debug, Display};

pub trait ArrowTrait<'a> {
    type SourceObject: CategoryTrait<'a>;

    type TargetObject: CategoryTrait<'a>;

    type Identifier;

    fn cell_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::SourceObject;

    fn target_object(&self) -> &Self::TargetObject;

    fn is_identity(&self) -> bool;

    fn sub_arrow(
        &self,
    ) -> Result<
        Functor<
            'a,
            Self::Identifier,
            <Self::SourceObject as CategoryTrait<'a>>::Object,
            <Self::TargetObject as CategoryTrait<'a>>::Object,
        >,
        NCategoryError,
    >;
}

pub trait ArrowMappingTrait<'a> {
    type Identifier: Identifier;

    type SourceArrow: ArrowTrait<'a>;

    type TargetArrow: ArrowTrait<'a>;

    fn source_arrow(&self) -> &Self::SourceArrow;

    fn target_arrow(&self) -> &Self::TargetArrow;

    fn source_sub_arrow_mapping(
        &self,
    ) -> Result<
        &Functor<
            'a,
            Self::Identifier,
            <Self::SourceArrow as ArrowTrait<'a>>::SourceObject,
            <Self::SourceArrow as ArrowTrait<'a>>::TargetObject,
        >,
        NCategoryError,
    >;

    fn target_sub_arrow_mapping(
        &self,
    ) -> Result<
        &Functor<
            'a,
            Self::Identifier,
            <Self::TargetArrow as ArrowTrait<'a>>::SourceObject,
            <Self::TargetArrow as ArrowTrait<'a>>::TargetObject,
        >,
        NCategoryError,
    >;
}

pub type Functor<'a, Id, SourceCategory, TargetCategory> = Vec<
    &'a dyn ArrowMappingTrait<
        'a,
        Identifier = Id,
        SourceArrow = <SourceCategory as CategoryTrait<'a>>::Morphism,
        TargetArrow = <TargetCategory as CategoryTrait<'a>>::Morphism,
    >,
>;
