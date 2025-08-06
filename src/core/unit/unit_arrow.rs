use crate::core::arrow::{Arrow, SubArrow};
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::unit::unit_category::UnitCategory;

pub struct UnitArrow<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> Arrow<'a> for UnitArrow<T> {
    type SourceObject = UnitCategory;
    type TargetObject = UnitCategory;
    type Identifier = T;

    fn arrow_id(&self) -> &Self::Identifier {
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
}

impl<'a, T: Identifier + 'a> SubArrow<'a> for UnitArrow<T> {
    type Identifier = T;
    type SourceObject = UnitCategory;
    type TargetObject = UnitCategory;

    fn sub_arrow(
        &self,
    ) -> &dyn Arrow<
        'a,
        Identifier = Self::Identifier,
        SourceObject = <Self::SourceObject as CategoryTrait<'a>>::Object,
        TargetObject = <Self::TargetObject as CategoryTrait<'a>>::Object,
    > {
        todo!()
    }
}
