use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::identifier::{Identifier, ObjectId};

pub trait NFunctor<'a>
where
    Self::Category: NCategory<'a>,
    // <Self::Category as NCategory>::BaseCategory: NCategory<Identifier = <Self::Category as NCategory>::Identifier>,
{
    type Category: NCategory<'a>;

    fn id(&self) -> &<Self::Category as NCategory<'a>>::Identifier;

    fn source_category(&self) -> &Self::Category;

    fn target_category(&self) -> &Self::Category;

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitFunctor<T: ObjectId> {
    _phantom: std::marker::PhantomData<T>,
}
impl <'a, T: ObjectId + 'a> NFunctor<'a> for UnitFunctor<T> {
    type Category = UnitCategory<T>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_category(&self) -> &Self::Category {
        todo!()
    }

    fn target_category(&self) -> &Self::Category {
        todo!()
    }
}