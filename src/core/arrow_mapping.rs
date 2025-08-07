use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_mapping_trait::ArrowMappingTrait;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

pub(crate) struct ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    id: Id,
    source_arrow: &'a SourceArrow,
    target_arrow: &'a TargetArrow,
}

impl<'a, Id, SourceArrow, TargetArrow> ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    pub fn new(id: Id, source_arrow: &'a SourceArrow, target_arrow: &'a TargetArrow) -> Self {
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
        &dyn FunctorTrait<
            'a,
            Self::Identifier,
            <Self::SourceArrow as ArrowTrait<'a>>::SourceObject,
            <Self::SourceArrow as ArrowTrait<'a>>::TargetObject,
        >,
        Errors,
    > {
        todo!()
    }

    fn target_sub_arrow_mapping(
        &self,
    ) -> Result<
        &dyn FunctorTrait<
            'a,
            Self::Identifier,
            <Self::TargetArrow as ArrowTrait<'a>>::SourceObject,
            <Self::TargetArrow as ArrowTrait<'a>>::TargetObject,
        >,
        Errors,
    > {
        todo!()
    }
}

impl<'a, Id, SourceArrow, TargetArrow> Debug for ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a, Id, SourceArrow, TargetArrow> Clone for ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<'a, Id, SourceArrow, TargetArrow> PartialEq for ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a, Id, SourceArrow, TargetArrow> Hash for ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<'a, Id, SourceArrow, TargetArrow> Eq for ArrowMapping<'a, Id, SourceArrow, TargetArrow>
where
    Id: Identifier,
    SourceArrow: ArrowTrait<'a>,
    TargetArrow: ArrowTrait<'a>,
{
}
