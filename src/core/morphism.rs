use crate::core::errors::Errors;
use crate::core::errors::Errors::InvalidArrowNoFunctorFound;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::morphism_trait::MorphismTrait;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Morphism<Id, Object>
where
    Id: Identifier,
    Object: CategoryTrait,
{
    id: Id,
    source: Rc<Object>,
    target: Rc<Object>,
    functor: Option<Rc<Functor<Id, Object, Object>>>,
    identity: bool,
}

impl<Id: Identifier, Object: CategoryTrait> Morphism<Id, Object> {
    pub fn new(
        id: Id,
        source: Rc<Object>,
        target: Rc<Object>,
        functor: Rc<Functor<Id, Object, Object>>,
    ) -> Self {
        Morphism {
            id,
            source,
            target,
            functor: Some(functor),
            identity: false,
        }
    }

    pub fn new_with_mappings(
        source: Rc<Object>,
        target: Rc<Object>,
        mappings: HashMap<Rc<MorphismAlias<Object>>, Rc<MorphismAlias<Object>>>,
    ) -> Self {
        let id = Id::generate();
        let functor = Functor::new(id.clone(), source.clone(), target.clone(), mappings);
        Morphism {
            id,
            source,
            target,
            functor: Some(functor.into()),
            identity: false,
        }
    }

    pub fn new_identity_morphism(object: Rc<Object>) -> Rc<Self> {
        let id = Id::generate();
        Rc::new(Morphism {
            id,
            source: object.clone(),
            target: object,
            functor: None,
            identity: true,
        })
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

impl<Id: Identifier, Object: CategoryTrait> Hash for Morphism<Id, Object> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state)
    }
}

impl<Id: Identifier, Object: CategoryTrait> ArrowTrait for Morphism<Id, Object> {
    type SourceObject = Object;
    type TargetObject = Object;

    fn source_object(&self) -> &Rc<Self::SourceObject> {
        &self.source
    }

    fn target_object(&self) -> &Rc<Self::TargetObject> {
        &self.target
    }

    fn is_identity(&self) -> bool {
        self.identity
    }

    fn compose(&self, other: &impl ArrowTrait) -> Result<Morphism<Id, Object>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Morphism<Id, Object>> {
        todo!()
    }
}

impl<'a, Id: Identifier, Category: CategoryTrait> MorphismTrait for Morphism<Id, Category> {
    fn functor(&self) -> Result<&Rc<Functor<Id, Self::SourceObject, Self::TargetObject>>, Errors> {
        if self.identity {
            return Err(Errors::NoFunctorForIdentityArrow);
        }
        self.functor.as_ref().ok_or(InvalidArrowNoFunctorFound)
    }
}
