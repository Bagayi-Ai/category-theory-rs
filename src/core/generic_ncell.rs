use std::fmt::Debug;
use std::hash::Hash;
use crate::core::discrete_category::DiscreteCategory;
use crate::core::identifier::{Identifier};
use crate::core::ncategory::NCategory;
use crate::core::ncell::NCell;
use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::nfunctor::NFunctor;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericNCell<Id: Identifier>
{
    id: Id,
    source_id: Id,
    target_id: Id,
    name: String,
}

impl <Id: Identifier> GenericNCell<Id>
{
    pub fn new(id: Id, source_id: Id, target_id: Id, name: String) -> Self {
        GenericNCell {
            id,
            source_id,
            target_id,
            name,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}


impl <Id: Identifier> NCell for GenericNCell<Id>
{
    type Identifier = Id;
    type Functor = GenericNFunctor<Self::Identifier>;

    fn cell_id(&self) -> &Self::Identifier {
        &self.id
    }

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object_id(&self) -> &Self::Identifier {
        &self.source_id
    }

    fn target_object_id(&self) -> &Self::Identifier {
        &self.target_id
    }

    fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier {
        todo!()
    }
}