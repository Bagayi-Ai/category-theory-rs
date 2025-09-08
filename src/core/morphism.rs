use crate::core::errors::Errors;
use crate::core::errors::Errors::InvalidArrowNoFunctorFound;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Morphism<Object: CategoryTrait> {
    id: String,
    source: Rc<Object>,
    target: Rc<Object>,
    functor: Option<Rc<dyn FunctorTrait<Object, Object>>>,
    identity: bool,
}

impl <Object: CategoryTrait> PartialEq for Morphism<Object> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        && Rc::ptr_eq(&self.source, &other.source)
        && Rc::ptr_eq(&self.target, &other.target)
        && self.identity == other.identity
        && match (&self.functor, &other.functor) {
            (Some(f1), Some(f2)) => Rc::ptr_eq(f1, f2),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<Object: CategoryTrait> Eq for Morphism<Object> {}

impl<Object: CategoryTrait> Hash for Morphism<Object> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        Rc::as_ptr(&self.source).hash(state);
        Rc::as_ptr(&self.target).hash(state);
        self.identity.hash(state);
        if let Some(functor) = &self.functor {
            Rc::as_ptr(functor).hash(state);
        } else {
            0.hash(state);
        }
    }
}

impl<Object: CategoryTrait> Debug for Morphism<Object> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Object: CategoryTrait> Morphism<Object>
{
    pub fn new(
        id: String,
        source: Rc<Object>,
        target: Rc<Object>,
        functor: Rc<Functor<Object, Object>>,
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
        mappings: HashMap<
            Rc<Morphism<CategorySubObjectAlias<Object>>>,
            Rc<Morphism<CategorySubObjectAlias<Object>>>,
        >,
    ) -> Self {
        let id = String::generate();
        let functor = Functor::new(id.clone(), source.clone(), target.clone(), mappings);
        let functor = Rc::new(functor);
        Morphism {
            id,
            source,
            target,
            functor: Some(functor),
            identity: false,
        }
    }

    pub fn new_identity_morphism(object: Rc<Object>) -> Rc<Self> {
        let id = String::generate();
        Rc::new(Morphism {
            id,
            source: object.clone(),
            target: object,
            functor: None,
            identity: true,
        })
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

impl<Object: CategoryTrait> ArrowTrait<Object, Object> for Morphism<Object>
{
    fn source_object(&self) -> &Rc<Object> {
        &self.source
    }

    fn target_object(&self) -> &Rc<Object> {
        &self.target
    }

    fn is_identity(&self) -> bool {
        self.identity
    }

    fn compose(
        &self,
        other: &dyn ArrowTrait<Object, Object>,
    ) -> Result<Rc<dyn ArrowTrait<Object, Object>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&dyn ArrowTrait<Object, Object>> {
        todo!()
    }
}

impl<Object: CategoryTrait> MorphismTrait<Object> for Morphism<Object>
where
    <Object as CategoryTrait>::Object: CategoryTrait,
{
    fn functor(&self) -> Result<&Rc<dyn  FunctorTrait<Object, Object>>, Errors> {
        if self.identity {
            return Err(Errors::NoFunctorForIdentityArrow);
        }
        // self.functor.as_ref().ok_or(InvalidArrowNoFunctorFound)
        self.functor
            .as_ref()
            .map(|rc| unsafe { &*(rc as *const Rc<dyn FunctorTrait<Object, Object>> as *const Rc<dyn FunctorTrait<Object, Object>>) })
            .ok_or(InvalidArrowNoFunctorFound)
    }
}
