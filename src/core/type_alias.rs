use crate::core::arrow_mapping::ArrowMapping;
use crate::core::traits::category_trait::CategoryTrait;

pub type ChildObjectAlias<'a, Object> = <Object as CategoryTrait<'a>>::Object;

pub type MorphismAlias<'a, Category> = <Category as CategoryTrait<'a>>::Morphism;

pub type ArrowMappingAlias<'a, Id, SourceCategory, TargetCategory> =
    ArrowMapping<'a, Id, MorphismAlias<'a, SourceCategory>, MorphismAlias<'a, TargetCategory>>;
