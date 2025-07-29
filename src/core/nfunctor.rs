use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::identifier::{Identifier};

pub trait NFunctor
where
    Self::Category: NCategory,
    // <Self::Category as NCategory>::BaseCategory: NCategory<Identifier = <Self::Category as NCategory>::Identifier>,
{
    type Category: NCategory;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier;

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitFunctor<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}
impl <T: Identifier> NFunctor for UnitFunctor<T> {
    type Category = UnitCategory<T>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }
}