use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::nfunctor::{NFunctor, UnitFunctor};

pub trait NCell
{
    type Identifier: Identifier;

    type Functor: NFunctor<Identifier = Self::Identifier>;

    fn cell_id(&self) -> &Self::Identifier;

    fn category_id(&self) -> &Self::Identifier;

    fn source_object_id(&self) -> &Self::Identifier;

    fn target_object_id(&self) -> &Self::Identifier;

    fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier;
}

impl <T: Identifier> NCell for UnitCategory<T> {

    type Identifier = T;

    type Functor = UnitFunctor<T>;

    fn cell_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn target_object_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier {
        todo!()
    }
}