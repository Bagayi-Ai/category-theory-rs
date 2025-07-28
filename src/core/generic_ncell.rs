use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;
use crate::core::ncell::NCell;
use crate::core::generic_nfunctor::GenericNFunctor;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericNCell<'a, Id: Identifier, Category: NCategory<'a, Identifier= Id>>
{
    id: &'a Id,
    source: &'a Category::Object,
    target: &'a Category::Object,
    name: String,
}

impl <'a, Id: Identifier, Category: NCategory<'a, Identifier= Id>> GenericNCell<'a, Id, Category>
{
    pub fn new(id: &'a Id, source: &'a Category::Object, target: &'a Category::Object, name: String) -> Self {
        GenericNCell {
            id,
            source,
            target,
            name,
        }
    }

    pub fn id(&self) -> &Id {
        self.id
    }

    pub fn source(&self) -> &Category::Object {
        self.source
    }

    pub fn target(&self) -> &Category::Object {
        self.target
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}


impl <'a, Id, Category: NCategory<'a>> NCell<'a> for GenericNCell<'a, Id, Category>
where
    Id: Identifier,
    Category: NCategory<'a, Identifier = Id>,
{
    type Category = Category;
    type Functor = GenericNFunctor<'a, Category::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory<'a>>::Identifier {
        todo!()
    }

    fn source_object(&self) -> &<Self::Category as NCategory<'a>>::Object {
        todo!()
    }

    fn target_object(&self) -> &<Self::Category as NCategory<'a>>::Object {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}

