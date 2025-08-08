use crate::core::arrow_mapping::ArrowMapping;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::MorphismAlias;

pub trait ArrowMappingTrait<'a> {
    type Identifier: Identifier;

    type SourceArrow: ArrowTrait<'a>;

    type TargetArrow: ArrowTrait<'a>;

    fn source_arrow(&self) -> &Self::SourceArrow;

    fn target_arrow(&self) -> &Self::TargetArrow;
}

pub type ArrowMappingAlias<'a, Id, SourceCategory, TargetCategory> =
    ArrowMapping<'a, Id, MorphismAlias<'a, SourceCategory>, MorphismAlias<'a, TargetCategory>>;
