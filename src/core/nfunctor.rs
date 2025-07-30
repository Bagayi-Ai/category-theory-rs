use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::identifier::{Identifier};

pub trait NFunctor
{
    type Identifier: Identifier;

    fn id(&self) -> &Self::Identifier;

    fn source_category_id(&self) -> &Self::Identifier;

    fn target_category_id(&self) -> &Self::Identifier;

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitFunctor<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}
impl <T: Identifier> NFunctor for UnitFunctor<T> {

    type Identifier = T;

    fn id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn target_category_id(&self) -> &Self::Identifier {
        todo!()
    }
}