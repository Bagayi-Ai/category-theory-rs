use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;

pub trait NArrow<'a>: 'a {
    type SourceObject: NCategory<'a>;

    type TargetObject: NCategory<'a>;

    type Identifier: Identifier;

    fn arrow_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::SourceObject;

    fn target_object(&self) -> &Self::TargetObject;

    fn is_identity(&self) -> bool;
}

pub trait SubNArrow<'a>: 'a {
    type Identifier: Identifier;
    type SourceObject: NCategory<'a>;
    type TargetObject: NCategory<'a>;

    fn sub_arrow(
        &self,
    ) -> &dyn NArrow<
        'a,
        Identifier = Self::Identifier,
        SourceObject = <Self::SourceObject as NCategory<'a>>::Object,
        TargetObject = <Self::TargetObject as NCategory<'a>>::Object,
    >;
}
