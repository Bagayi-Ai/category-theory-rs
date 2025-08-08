use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::unit::unit_category::UnitCategory;
use std::collections::HashMap;

pub struct UnitFunctor {}

impl<'a, Id: Identifier> FunctorTrait<'a, Id, UnitCategory, UnitCategory> for UnitFunctor {
    fn functor_id(&self) -> &Id {
        todo!()
    }

    fn source_category(&self) -> &'a UnitCategory {
        todo!()
    }

    fn target_category(&self) -> &'a UnitCategory {
        todo!()
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<&MorphismAlias<'a, UnitCategory>, &MorphismAlias<'a, UnitCategory>> {
        todo!()
    }
}
