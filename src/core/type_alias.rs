use crate::core::arrow_mapping::ArrowMapping;
use crate::core::traits::category_trait::CategoryTrait;

pub type MorphismAlias<'a, Category> = <Category as CategoryTrait<'a>>::Morphism;

pub type ArrowMappingAlias<'a, Id, SourceCategory, TargetCategory> =
    ArrowMapping<'a, Id, MorphismAlias<'a, SourceCategory>, MorphismAlias<'a, TargetCategory>>;
