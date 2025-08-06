use crate::core::arrow::Arrow;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::{ArrowMappingTrait, Functor, ArrowTrait};
use crate::core::traits::category_trait::{CategoryTrait, NCategoryError};
use std::marker::PhantomData;

pub(crate) struct ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    id: &'a Id,
    source_arrow: &'a SourceArrow,
    target_arrow: &'a TargetArrow,
}

impl<'a, Id, SourceArrow, TargetArrow> ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    pub fn new(id: &'a Id, source_arrow: &'a SourceArrow, target_arrow: &'a TargetArrow) -> Self {
        ArrowMapping {
            id,
            source_arrow,
            target_arrow,
        }
    }
}

impl<'a, Id, SourceArrow, TargetArrow> ArrowMappingTrait<'a>
    for ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    type Identifier = Id;
    type SourceArrow = SourceArrow;
    type TargetArrow = TargetArrow;

    fn source_arrow(&self) -> &Self::SourceArrow {
        todo!()
    }

    fn target_arrow(&self) -> &Self::TargetArrow {
        todo!()
    }

    fn source_sub_arrow_mapping(
        &self,
    ) -> Result<
        &Functor<
            'a,
            Self::Identifier,
            <Self::SourceArrow as ArrowTrait<'a>>::SourceObject,
            <Self::SourceArrow as ArrowTrait<'a>>::TargetObject,
        >,
        NCategoryError,
    > {
        todo!()
    }

    fn target_sub_arrow_mapping(
        &self,
    ) -> Result<
        &Functor<
            'a,
            Self::Identifier,
            <Self::TargetArrow as ArrowTrait<'a>>::SourceObject,
            <Self::TargetArrow as ArrowTrait<'a>>::TargetObject,
        >,
        NCategoryError,
    > {
        todo!()
    }
}
