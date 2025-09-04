use crate::core::errors::Errors;
use crate::core::errors::Errors::InvalidArrowNoFunctorFound;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Morphism<Object: CategoryTrait> {
    id: String,
    source: Rc<Object>,
    target: Rc<Object>,
    functor: Option<Rc<Functor<Object, Object>>>,
    identity: bool,
}

impl<Object: CategoryTrait> Morphism<Object> {
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
        // let id = String::generate();
        // let functor = Functor::new(id.clone(), source.clone(), target.clone(), mappings);
        // let functor = Rc::new(functor);
        // Morphism {
        //     id,
        //     source,
        //     target,
        //     functor: Some(functor),
        //     identity: false,
        // }
        todo!()
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

impl<Object: CategoryTrait> ArrowTrait<Object, Object> for Morphism<Object> {
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
        other: &impl ArrowTrait<Object, Object>,
    ) -> Result<Rc<Morphism<Object>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Morphism<Object>> {
        todo!()
    }
}

impl<Object: CategoryTrait> MorphismTrait<Object> for Morphism<Object>
where
    <Object as CategoryTrait>::Object: CategoryTrait,
{
    fn functor(&self) -> Result<&Rc<impl FunctorTrait<Object, Object>>, Errors> {
        if self.identity {
            return Err(Errors::NoFunctorForIdentityArrow);
        }
        self.functor.as_ref().ok_or(InvalidArrowNoFunctorFound)
    }
}
