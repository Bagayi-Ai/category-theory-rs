use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::category_trait::MorphismAlias;
use std::collections::HashMap;

pub trait FunctorTrait<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait<'a> + ?Sized,
    TargetCategory: CategoryTrait<'a> + ?Sized,
{
    fn functor_id(&self) -> &Id;

    fn source_category(&self) -> &'a SourceCategory;

    fn target_category(&self) -> &'a TargetCategory;

    fn arrow_mappings(
        &self,
    ) -> &HashMap<&MorphismAlias<'a, SourceCategory>, &MorphismAlias<'a, TargetCategory>>;
}
