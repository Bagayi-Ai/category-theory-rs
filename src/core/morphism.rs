use std::hash::Hash;
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::nfunctor::{NFunctor, UnitFunctor};
use std::fmt::Debug;

pub trait Morphism<'a>: Debug + Eq + Hash {
    type Category: NCategory<'a>;

    type Functor: NFunctor<
            'a,
            Identifier = <Self::Category as NCategory<'a>>::Identifier,
            SourceCategory = <Self::Category as NCategory<'a>>::BaseCategory,
            TargetCategory = <Self::Category as NCategory<'a>>::BaseCategory,
        >;

    fn cell_id(&self) -> &<Self::Category as NCategory<'a>>::Identifier;

    fn source_object(&self) -> <Self::Category as NCategory<'a>>::Object;

    fn target_object(&self) -> <Self::Category as NCategory<'a>>::Object;

    fn functor(&self) -> &Self::Functor;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> Morphism<'a> for UnitMorphism<T> {
    type Category = UnitCategory<T>;

    type Functor = UnitFunctor<'a, T, Self::Category, Self::Category>;

    fn cell_id(&self) -> &<Self::Category as NCategory<'a>>::Identifier {
        todo!()
    }

    fn source_object(&self) -> <Self::Category as NCategory<'a>>::Object {
        todo!()
    }

    fn target_object(&self) -> <Self::Category as NCategory<'a>>::Object {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}
