use std::fmt::Debug;
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::nfunctor::{NFunctor, UnitFunctor};

pub trait Morphism<'a> : Debug
{
    type Category: NCategory<'a>;

    // type Functor: NFunctor<Identifier = Self::Identifier>;

    fn cell_id(&self) -> &<Self::Category as NCategory<'a>>::Identifier;

    fn source_object(&self) -> <Self::Category as NCategory<'a>>::Object;

    fn target_object(&self) -> <Self::Category as NCategory<'a>>::Object;

    // fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}



impl <'a, T: Identifier> Morphism<'a> for UnitMorphism<T> {
    type Category = UnitCategory<T>;

    fn cell_id(&self) -> &<Self::Category as NCategory<'a>>::Identifier {
        todo!()
    }

    fn source_object(&self) -> <Self::Category as NCategory<'a>>::Object {
        todo!()
    }

    fn target_object(&self) -> <Self::Category as NCategory<'a>>::Object {
        todo!()
    }
}