use crate::core::identifier::Identifier;
use crate::core::narrow::{NArrow, SubNArrow};
use crate::core::ncategory::NCategory;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
{
    id: Id,
    source: &'a SourceCategory,
    target: &'a TargetCategory,
}

impl<'a, Id, SourceCategory, TargetCategory> GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier + 'a,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
{
    pub fn new(id: Id, source: &'a SourceCategory, target: &'a TargetCategory) -> Self {
        GenericNArrow { id, source, target }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

impl<'a, Id, SourceCategory, TargetCategory> NArrow<'a>
    for GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier + 'a,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
{
    type SourceObject = SourceCategory;
    type TargetObject = TargetCategory;
    type Identifier = Id;

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

impl<'a, Id, SourceCategory, TargetCategory> SubNArrow<'a>
    for GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier + 'a,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
{
    type Identifier = Id;

    type SourceObject = SourceCategory;

    type TargetObject = TargetCategory;

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
