use crate::core::identifier::Identifier;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::ncategory::{NCategory};
use crate::core::nfunctor::{NFunctor};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Morphism<'a>: Debug + Eq + Hash
where
    <Self as Morphism<'a>>::Object: NCategory<'a>
{
    type Object;
    type Identifier;

    type Functor: NFunctor<
            'a,
            Identifier = Self::Identifier,
            SourceCategory = <Self::Object as NCategory<'a>>::Object,
            TargetCategory = <Self::Object as NCategory<'a>>::Object,
        >;

    fn cell_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::Object;

    fn target_object(&self) -> &Self::Object;

    fn is_identity(&self) -> bool;

    fn functor(&self) -> &Self::Functor;
}