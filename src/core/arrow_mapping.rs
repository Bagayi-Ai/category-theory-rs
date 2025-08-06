use crate::core::arrow::Arrow;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::{ArrowMappingTrait, ArrowTrait};
use crate::core::traits::category_trait::{CategoryTrait, NCategoryError};
use std::marker::PhantomData;

pub(crate) struct ArrowMapping<'a, SourceArrow, TargetArrow>
where
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    source_arrow: &'a SourceArrow,
    target_arrow: &'a TargetArrow,
}

impl<'a, SourceArrow, TargetArrow> ArrowMapping<'a, SourceArrow, TargetArrow>
where
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    pub fn new(source_arrow: &'a SourceArrow, target_arrow: &'a TargetArrow) -> Self {
        ArrowMapping {
            source_arrow,
            target_arrow,
        }
    }
}

impl<'a, SourceArrow, TargetArrow> ArrowMappingTrait<'a>
    for ArrowMapping<'a, SourceArrow, TargetArrow>
where
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    type SourceArrow = SourceArrow;
    type TargetArrow = TargetArrow;

    fn source_arrow(&self) -> &Self::SourceArrow {
        todo!()
    }

    fn target_arrow(&self) -> &Self::TargetArrow {
        todo!()
    }
}
