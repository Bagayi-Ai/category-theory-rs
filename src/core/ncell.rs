use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::nfunctor::{NFunctor, UnitFunctor};

pub trait NCell
where
    Self::Category: NCategory,
    <Self::Category as NCategory>::BaseCategory: NCategory<Identifier = <Self::Category as NCategory>::Identifier>,
{
    type Category: NCategory;

    type Functor: NFunctor<Category = <Self::Category as NCategory>::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn functor(&self) -> &<<Self::Functor as NFunctor>::Category as NCategory>::Identifier;
}

impl <T: Identifier> NCell for UnitCategory<T> {
    type Category = UnitCategory<T>;

    type Functor = UnitFunctor<T>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn functor(&self) -> &<<Self::Functor as NFunctor>::Category as NCategory>::Identifier {
        todo!()
    }
}