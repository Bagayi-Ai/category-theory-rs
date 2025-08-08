use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::unit::unit_category::UnitCategory;
use std::collections::HashMap;
use crate::core::unit::unit_morphism::UnitMorphism;

impl<'a, T: Identifier + 'a> FunctorTrait<'a> for UnitMorphism<T>{
    fn functor_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn arrow_mappings(&self) -> &HashMap<&MorphismAlias<'a, Self::SourceObject>, &MorphismAlias<'a, Self::TargetObject>> {
        todo!()
    }
}