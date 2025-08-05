use crate::core::identifier::Identifier;
use crate::core::narrow::{NArrow, SubNArrow};
use crate::core::ncategory::NCategory;
use crate::core::unit::unit_category::UnitCategory;

pub struct UnitArrow<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> NArrow<'a> for UnitArrow<T> {
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

impl<'a, T: Identifier + 'a> SubNArrow<'a> for UnitArrow<T> {
    type Identifier = T;
    type SourceObject = UnitCategory;
    type TargetObject = UnitCategory;

    fn sub_arrow(
        &self,
    ) -> &dyn NArrow<
        'a,
        Identifier = Self::Identifier,
        SourceObject = <Self::SourceObject as NCategory<'a>>::Object,
        TargetObject = <Self::TargetObject as NCategory<'a>>::Object,
    > {
        todo!()
    }
}
