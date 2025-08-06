use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct Arrow<'a, Id, SourceObject, TargetObject>
where
    Id: Identifier,
    SourceObject: CategoryTrait<'a>,
    TargetObject: CategoryTrait<'a>,
{
    id: Id,
    source: &'a SourceObject,
    target: &'a TargetObject,
}

impl<'a, Id: Identifier, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>>
    Arrow<'a, Id, SourceObject, TargetObject>
{
    pub fn new(id: Id, source: &'a SourceObject, target: &'a TargetObject) -> Self {
        Arrow { id, source, target }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

impl<'a, Id: Identifier, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>> Hash
    for Arrow<'a, Id, SourceObject, TargetObject>
{
    fn hash<H>(&self, _: &mut H)
    where
        H: Hasher,
    {
        todo!()
    }
}

impl<'a, Id: Identifier + 'a, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>>
    ArrowTrait<'a> for Arrow<'a, Id, SourceObject, TargetObject>
{
    type SourceObject = SourceObject;
    type TargetObject = TargetObject;
    type Identifier = Id;

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
