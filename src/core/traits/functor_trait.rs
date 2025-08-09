use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::category_trait::MorphismAlias;
use std::collections::HashMap;

pub trait FunctorTrait<'a>: ArrowTrait<'a> {
    fn functor_id(&self) -> &Self::Identifier;

    fn arrow_mappings(
        &self,
    ) -> &HashMap<&MorphismAlias<'a, Self::SourceObject>, &MorphismAlias<'a, Self::TargetObject>>;
}
