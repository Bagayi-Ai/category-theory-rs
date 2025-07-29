use crate::core::identifier::{Identifier};
use crate::core::ncategory::NCategory;
use crate::core::ncell::NCell;
use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::nfunctor::NFunctor;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericNCell<Id: Identifier, Category: NCategory<Identifier= Id>>
{
    id: Id,
    source_id: Id,
    target_id: Id,
    name: String,
    _phantom: std::marker::PhantomData<Category>,
}

impl <Id: Identifier, Category: NCategory<Identifier= Id>> GenericNCell<Id, Category>
{
    pub fn new(id: Id, source_id: Id, target_id: Id, name: String) -> Self {
        GenericNCell {
            id,
            source_id,
            target_id,
            name,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn source(&self) -> &Category::Object {
        self.source()
    }

    pub fn target(&self) -> &Category::Object {
        self.target()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}


impl <Id, Category: NCategory> NCell for GenericNCell<Id, Category>
where
    Id: Identifier,
    Category: NCategory<Identifier = Id>,
{
    type Category = Category;
    type Functor = GenericNFunctor<Category::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn functor(&self) -> &<Self::Category as NCategory>::Identifier{
        todo!()
    }
}

