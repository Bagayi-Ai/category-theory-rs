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
        ArrowMappingsTrait<
            'a,
            <Self::SourceObject as CategoryTrait<'a>>::Object,
            <Self::SourceObject as CategoryTrait<'a>>::Object,
        >,
        NCategoryError,
    >;
}

pub trait ArrowMappingTrait<'a> {
    type SourceArrow: ArrowTrait<'a>;

    type TargetArrow: ArrowTrait<'a>;

    fn source_arrow(&self) -> &Self::SourceArrow;

    fn target_arrow(&self) -> &Self::TargetArrow;
}

pub type ArrowMappingsTrait<'a, SourceCategory, TargetCategory> = Vec<
    &'a dyn ArrowMappingTrait<
        'a,
        SourceArrow = <SourceCategory as CategoryTrait<'a>>::Morphism,
        TargetArrow = <TargetCategory as CategoryTrait<'a>>::Morphism,
    >,
>;
