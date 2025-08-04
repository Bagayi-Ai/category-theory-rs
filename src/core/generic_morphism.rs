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
    source: &'a <Category as NCategory<'a>>::Object,
    target: &'a <Category as NCategory<'a>>::Object,
    name: String,
}

impl<'a, Category: NCategory<'a>> GenericMorphism<'a, Category>
where
    <Category as NCategory<'a>>::Object: Clone,
{
    pub fn new(
        id: <Category as NCategory<'a>>::Identifier,
        source: &'a <Category as NCategory<'a>>::Object,
        target: &'a <Category as NCategory<'a>>::Object,
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
    <Category as NCategory<'a>>::Object: 'a,
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
    <Category as NCategory<'a>>::Object: Clone + 'a + NCategory<'a>,
    <Category as NCategory<'a>>::Identifier: 'a,
{
    type Object = <Category as NCategory<'a>>::Object;
    type Identifier = <Category as NCategory<'a>>::Identifier;

    type Functor = GenericNFunctor<
        'a,
        Self::Identifier,
        <Self::Object as NCategory<'a>>::Object,
        <Self::Object as NCategory<'a>>::Object,
    >;

    fn cell_id(&self) -> &Self::Identifier {
        &self.id
    }

    fn source_object(&self) -> &Self::Object {
        self.source
    }

    fn target_object(&self) -> &Self::Object {
        self.target
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
