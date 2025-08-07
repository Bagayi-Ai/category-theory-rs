use crate::core::identifier::Identifier;
use crate::core::traits::arrow_mapping_trait::ArrowMappingTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::category_trait::MorphismAlias;

pub trait FunctorTrait<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    fn functor_id(&self) -> &Id;

    fn source_category(&self) -> &'a SourceCategory;

    fn target_category(&self) -> &'a TargetCategory;

    fn arrow_mappings(
        &self,
    ) -> Vec<
        &dyn ArrowMappingTrait<
            'a,
            Identifier = Id,
            SourceArrow = MorphismAlias<'a, SourceCategory>,
            TargetArrow = MorphismAlias<'a, TargetCategory>,
        >,
    >;
}
