use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;

pub trait Arrow<'a>: 'a {
    type SourceObject: CategoryTrait<'a>;

    type TargetObject: CategoryTrait<'a>;

    type Identifier: Identifier;

    fn arrow_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::SourceObject;

    fn target_object(&self) -> &Self::TargetObject;

    fn is_identity(&self) -> bool;
}

pub trait SubArrow<'a>: 'a {
    type Identifier: Identifier;
    type SourceObject: CategoryTrait<'a>;
    type TargetObject: CategoryTrait<'a>;

    fn sub_arrow(
        &self,
    ) -> &dyn Arrow<
        'a,
        Identifier = Self::Identifier,
        SourceObject = <Self::SourceObject as CategoryTrait<'a>>::Object,
        TargetObject = <Self::TargetObject as CategoryTrait<'a>>::Object,
    >;
}
