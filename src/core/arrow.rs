use crate::core::errors::Errors;
use crate::core::errors::Errors::{InvalidArrowNoFunctorFound, NoFunctorForIdentityArrow};
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use crate::core::functor::Functor;

pub struct Arrow<'a, Id, SourceObject, TargetObject>
where
    Id: Identifier,
    SourceObject: CategoryTrait<'a>,
    TargetObject: CategoryTrait<'a>,
{
    id: Id,
    source: &'a SourceObject,
    target: &'a TargetObject,
    functor: Option<&'a Arrow<'a, Id, SourceObject, TargetObject>>,
    identity: bool,
    mappings: Option<HashMap<
        &'a MorphismAlias<'a, SourceObject>,
        &'a MorphismAlias<'a, TargetObject>,
    >>
}

impl<'a, Id: Identifier, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>>
    Arrow<'a, Id, SourceObject, TargetObject>
{
    pub fn new(
        id: Id,
        source: &'a SourceObject,
        target: &'a TargetObject,
        functor: &'a Arrow<'a, Id, SourceObject, TargetObject>,
    ) -> Self {
        Arrow {
            id,
            source,
            target,
            functor: functor.into(),
            identity: false,
            mappings: None
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
            mappings: None,
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
        &impl FunctorTrait<'a>,
        Errors,
    > {
        if self.identity {
            return Err(Errors::NoFunctorForIdentityArrow);
        }
        self.functor.ok_or(InvalidArrowNoFunctorFound)
    }
}


impl<'a, Id: Identifier, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>>
Functor<'a, Id, SourceObject, TargetObject> {
    pub fn new_functor(
        id: Id,
        source: &'a SourceObject,
        target: &'a TargetObject,
        mapping: HashMap<
            &'a MorphismAlias<'a, SourceObject>,
            &'a MorphismAlias<'a, TargetObject>,
        >,
    ) -> Self {
        Functor {
            id,
            source,
            target,
            functor: None,
            identity: false,
            mappings: Some(mapping),
        }
    }

}

impl<'a, Id: Identifier, SourceObject: CategoryTrait<'a>, TargetObject: CategoryTrait<'a>>
FunctorTrait<'a> for Functor<'a, Id, SourceObject, TargetObject>
{
    fn functor_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn arrow_mappings(&self) -> &HashMap<&MorphismAlias<'a, Self::SourceObject>, &MorphismAlias<'a, Self::TargetObject>> {
        todo!()
    }
}

