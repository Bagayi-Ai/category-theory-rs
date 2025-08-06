use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::fmt::Debug;
use std::hash::Hash;

pub trait MorphismTrait<'a>: Debug + Eq + Hash
where
    <Self as MorphismTrait<'a>>::Object: CategoryTrait<'a>,
{
    type Object;
    type Identifier;

    type Functor: FunctorTrait<
            'a,
            Identifier = Self::Identifier,
            SourceCategory = <Self::Object as CategoryTrait<'a>>::Object,
            TargetCategory = <Self::Object as CategoryTrait<'a>>::Object,
        >;

    fn cell_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::Object;

    fn target_object(&self) -> &Self::Object;

    fn is_identity(&self) -> bool;

    fn functor(&self) -> &Self::Functor;
}
