use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct GenericMorphism<'a, Category>
where
    Category: CategoryTrait<'a>,
    <Category as CategoryTrait<'a>>::Object: Clone,
{
    id: <Category as CategoryTrait<'a>>::Identifier,
    source: &'a <Category as CategoryTrait<'a>>::Object,
    target: &'a <Category as CategoryTrait<'a>>::Object,
    name: String,
}

impl<'a, Category: CategoryTrait<'a>> GenericMorphism<'a, Category>
where
    <Category as CategoryTrait<'a>>::Object: Clone,
{
    pub fn new(
        id: <Category as CategoryTrait<'a>>::Identifier,
        source: &'a <Category as CategoryTrait<'a>>::Object,
        target: &'a <Category as CategoryTrait<'a>>::Object,
        name: String,
    ) -> Self {
        GenericMorphism {
            id,
            source,
            target,
            name,
        }
    }

    pub fn id(&self) -> &<Category as CategoryTrait<'a>>::Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'a, Category: CategoryTrait<'a>> Hash for GenericMorphism<'a, Category>
where
    <Category as CategoryTrait<'a>>::Object: Clone,
    <Category as CategoryTrait<'a>>::Object: 'a,
{
    fn hash<H>(&self, _: &mut H)
    where
        H: Hasher,
    {
        todo!()
    }
}

impl<'a, Category: CategoryTrait<'a> + Eq> ArrowTrait<'a> for GenericMorphism<'a, Category>
where
    <Category as CategoryTrait<'a>>::Object: Clone + 'a + CategoryTrait<'a>,
    <Category as CategoryTrait<'a>>::Identifier: 'a,
{
    type SourceObject = <Category as CategoryTrait<'a>>::Object;
    type TargetObject = <Category as CategoryTrait<'a>>::Object;
    type Identifier = <Category as CategoryTrait<'a>>::Identifier;

    type Functor = GenericNFunctor<'a, Self::Identifier, Self::SourceObject, Self::TargetObject>;

    fn cell_id(&self) -> &Self::Identifier {
        &self.id
    }

    fn source_object(&self) -> &Self::SourceObject {
        self.source
    }

    fn target_object(&self) -> &Self::TargetObject {
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
