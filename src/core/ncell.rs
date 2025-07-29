use crate::core::identifier::{Identifier, ObjectId};
use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::nfunctor::{NFunctor, UnitFunctor};

pub trait NCell<'a> : 'a
where
    Self::Category: NCategory<'a>,
    <Self::Category as NCategory<'a>>::BaseCategory: NCategory<'a, Identifier = <Self::Category as NCategory<'a>>::Identifier>,
{
    type Category: NCategory<'a>;
    type Functor: NFunctor<'a, Category = <Self::Category as NCategory<'a>>::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory<'a>>::Identifier;

    fn source_object(&self) -> &<Self::Category as NCategory<'a>>::Object;

    fn source_object_id(&'a self) -> &<<Self::Category as NCategory<'a>>::Object as ObjectId>::Id {
        let source_object = self.source_object();
        source_object.object_id()
    }

    fn target_object(&self) -> &<Self::Category as NCategory<'a>>::Object;

    // Each cell should have a functor that maps its source base category to its target base category.
    fn functor(&self) -> &Self::Functor;

}

impl <'a, T: ObjectId + 'a> NCell<'a> for UnitCategory<T> {
    type Category = UnitCategory<T>;
    type Functor = UnitFunctor<T>;

    fn id(&self) -> &<Self::Category as NCategory<'a>>::Identifier {
        todo!()
    }

    fn source_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn target_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}