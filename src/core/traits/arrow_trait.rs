use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::fmt::Debug;
use std::hash::Hash;

pub type DynArrowTraitType<'a, ID, SC, TC> = dyn ArrowTrait<
        'a,
        SourceObject = <SC as CategoryTrait<'a>>::Object,
        TargetObject = <TC as CategoryTrait<'a>>::Object,
        Identifier = ID,
        Functor = dyn FunctorTrait<
            'a,
            Identifier = ID,
            SourceCategory = <SC as CategoryTrait<'a>>::Object,
            TargetCategory = <TC as CategoryTrait<'a>>::Object,
        >,
    >;

pub trait ArrowTrait<'a>: Debug {
    type SourceObject: CategoryTrait<'a>;

    type TargetObject: CategoryTrait<'a>;

    type Identifier;

    type Functor: FunctorTrait<
            'a,
            Identifier = Self::Identifier,
            SourceCategory = Self::SourceObject,
            TargetCategory = Self::TargetObject,
        >;

    fn cell_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::SourceObject;

    fn target_object(&self) -> &Self::TargetObject;

    fn is_identity(&self) -> bool;

    fn functor(&self) -> &Self::Functor;
}
