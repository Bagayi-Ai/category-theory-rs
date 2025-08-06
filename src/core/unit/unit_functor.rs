use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowMappingTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::unit::unit_category::UnitCategory;

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
    ) -> Vec<
        &dyn ArrowMappingTrait<
            'a,
            Identifier = Id,
            SourceArrow = <UnitCategory as CategoryTrait<'a>>::Morphism,
            TargetArrow = <UnitCategory as CategoryTrait<'a>>::Morphism,
        >,
    > {
        todo!()
    }
}
