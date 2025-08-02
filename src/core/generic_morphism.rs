use crate::core::discrete_category::DiscreteCategory;
use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct GenericMorphism<'a, Category>
where
    Category: NCategory<'a>,
    <Category as NCategory<'a>>::Object: Clone,
{
    id: <Category as NCategory<'a>>::Identifier,
    source: <Category as NCategory<'a>>::Object,
    target: <Category as NCategory<'a>>::Object,
    name: String,
}

impl<'a, Category: NCategory<'a>> GenericMorphism<'a, Category>
where
    <Category as NCategory<'a>>::Object: Clone,
{
    pub fn new(
        id: <Category as NCategory<'a>>::Identifier,
        source: <Category as NCategory<'a>>::Object,
        target: <Category as NCategory<'a>>::Object,
        name: String,
    ) -> Self {
        GenericMorphism {
            id,
            source,
            target,
            name,
        }
    }

    pub fn id(&self) -> &<Category as NCategory<'a>>::Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'a, Category: NCategory<'a>> Hash for GenericMorphism<'a, Category>
where
    <Category as NCategory<'a>>::Object: Clone,
    <Category as NCategory<'a>>::BaseCategory: 'a,
{
    fn hash<H>(&self, _: &mut H)
    where
        H: Hasher,
    {
        todo!()
    }
}

impl<'a, Category: NCategory<'a> + Eq> Morphism<'a> for GenericMorphism<'a, Category>
where
    <Category as NCategory<'a>>::Object: Clone,
    <Category as NCategory<'a>>::BaseCategory: 'a,
    <Category as NCategory<'a>>::Identifier: 'a,
{
    type Category = Category;
    type Functor = GenericNFunctor<
        'a,
        <Category as NCategory<'a>>::Identifier,
        <Category as NCategory<'a>>::BaseCategory,
        <Category as NCategory<'a>>::BaseCategory,
    >;

    fn cell_id(&self) -> &<Category as NCategory<'a>>::Identifier {
        &self.id
    }

    fn source_object(&self) -> <Category as NCategory<'a>>::Object {
        self.source.clone()
    }

    fn target_object(&self) -> <Category as NCategory<'a>>::Object {
        self.target.clone()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }

    // fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier {
    //     todo!()
    // }
}
