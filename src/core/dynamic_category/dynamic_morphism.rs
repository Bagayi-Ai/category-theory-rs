use crate::core::dynamic_category::dynamic_category::{
    DynamicCategory, DynamicCategoryTypeAlias, DynamicType,
};
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use std::rc::Rc;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DynamicMorphism {
    id: String,
    source: Rc<DynamicCategory>,
    target: Rc<DynamicCategory>,
    functor: Option<Rc<DynamicCategory>>,
    identity: bool,
}

impl DynamicMorphism {
    pub fn new(
        id: String,
        source: Rc<DynamicCategory>,
        target: Rc<DynamicCategory>,
        functor: Rc<DynamicCategory>,
    ) -> Result<Self, Errors> {
        if *functor.dynamic_type() != DynamicType::Functor {
            return Err(Errors::InvalidDynamicType(
                "Expected Functor type".to_string(),
            ));
        }
        Ok(DynamicMorphism {
            id,
            source,
            target,
            functor: Some(functor),
            identity: false,
        })
    }

    pub fn new_identity_morphism(object: Rc<DynamicCategoryTypeAlias>) -> Rc<DynamicMorphism> {
        todo!()
    }
}

impl ArrowTrait for DynamicMorphism {
    type SourceObject = DynamicCategory;
    type TargetObject = DynamicCategory;

    fn source_object(&self) -> &Rc<Self::SourceObject> {
        todo!()
    }

    fn target_object(&self) -> &Rc<Self::TargetObject> {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn compose(&self, other: &impl ArrowTrait) -> Result<Self, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Self> {
        todo!()
    }
}

impl MorphismTrait for DynamicMorphism {
    fn functor(&self) -> Result<&Rc<DynamicCategory>, Errors> {
        todo!()
    }
}
