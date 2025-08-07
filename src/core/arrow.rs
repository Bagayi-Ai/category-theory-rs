use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::hash::{Hash, Hasher};

pub struct Arrow<'a, Id, SourceObject, TargetObject>
where
    Id: Identifier,
    SourceObject: CategoryTrait<'a>,
    TargetObject: CategoryTrait<'a>,
{
    id: Id,
    source: &'a SourceObject,
    target: &'a TargetObject,
    functor: Option<&'a dyn FunctorTrait<'a, Id, SourceObject, TargetObject>>,
    identity: bool,
}

impl<'a, Id: Identifier, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>>
    Arrow<'a, Id, SourceObject, TargetObject>
{
    pub fn new(
        id: Id,
        source: &'a SourceObject,
        target: &'a TargetObject,
        functor: &'a dyn FunctorTrait<'a, Id, SourceObject, TargetObject>,
    ) -> Self {
        Arrow {
            id,
            source,
            target,
            functor: functor.into(),
            identity: false,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

impl<'a, Id: Identifier, Object: CategoryTrait<'a, Identifier = Id>> Arrow<'a, Id, Object, Object> {
    pub fn new_identity_arrow(object: &'a Object) -> Self {
        Arrow {
            id: object.category_id().clone(),
            source: object,
            target: object,
            functor: None,
            identity: true,
        }
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

    fn arrow_id(&self) -> &Self::Identifier {
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

    fn functor(
        &self,
    ) -> Result<
        &Functor<
            'a,
            Id,
            <Self::SourceObject as CategoryTrait<'a>>::Object,
            <Self::TargetObject as CategoryTrait<'a>>::Object,
        >,
        Errors,
    > {
        todo!()
    }
}
