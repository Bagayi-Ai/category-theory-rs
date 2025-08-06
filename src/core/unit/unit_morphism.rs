use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::{Functor, ArrowTrait};
use crate::core::traits::category_trait::{CategoryTrait, NCategoryError};
use crate::core::unit::unit_category::UnitCategory;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> ArrowTrait<'a> for UnitMorphism<T> {
    type SourceObject = UnitCategory;
    type TargetObject = Self::SourceObject;
    type Identifier = T;

    fn cell_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object(&self) -> &Self::SourceObject {
        todo!()
    }

    fn target_object(&self) -> &Self::TargetObject {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn sub_arrow(
        &self,
    ) -> Result<
        Functor<
            'a,
            Self::Identifier,
            <Self::SourceObject as CategoryTrait<'a>>::Object,
            <Self::SourceObject as CategoryTrait<'a>>::Object,
        >,
        NCategoryError,
    > {
        todo!()
    }
}
