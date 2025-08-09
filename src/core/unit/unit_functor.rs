use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::unit::unit_morphism::UnitMorphism;
use std::collections::HashMap;

pub struct UnitFunctor {}

impl<'a, Id: Identifier + 'a> FunctorTrait<'a> for UnitMorphism<Id> {
    fn functor_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<&MorphismAlias<'a, Self::SourceObject>, &MorphismAlias<'a, Self::TargetObject>>
    {
        todo!()
    }
}
