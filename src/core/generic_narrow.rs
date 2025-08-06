use crate::core::arrow::{Arrow, SubArrow};
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    id: Id,
    source: &'a SourceCategory,
    target: &'a TargetCategory,
}

impl<'a, Id, SourceCategory, TargetCategory> GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier + 'a,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    pub fn new(id: Id, source: &'a SourceCategory, target: &'a TargetCategory) -> Self {
        GenericNArrow { id, source, target }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

impl<'a, Id, SourceCategory, TargetCategory> Arrow<'a>
    for GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier + 'a,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
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

impl<'a, Id, SourceCategory, TargetCategory> SubArrow<'a>
    for GenericNArrow<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier + 'a,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    type Identifier = Id;

    type SourceObject = SourceCategory;

    type TargetObject = TargetCategory;

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
