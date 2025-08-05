use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;

pub trait Arrow<'a>: 'a {
    type SourceObject: NCategory<'a>;

    type TargetObject: NCategory<'a>;

    type Identifier: Identifier;

    fn arrow_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::SourceObject;

    fn target_object(&self) -> &Self::TargetObject;

    fn is_identity(&self) -> bool;
}

pub trait SubArrow<'a>: 'a {
    type Identifier: Identifier;
    type SourceObject: NCategory<'a>;
    type TargetObject: NCategory<'a>;

    fn sub_arrow(
        &self,
    ) -> &dyn Arrow<
        'a,
        Identifier = Self::Identifier,
        SourceObject = <Self::SourceObject as NCategory<'a>>::Object,
        TargetObject = <Self::TargetObject as NCategory<'a>>::Object,
    >;
}
