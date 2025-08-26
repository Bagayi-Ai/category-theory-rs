use crate::core::dynamic_category::dynamic_category::DynamicCategory;
use crate::core::errors::Errors;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::MorphismAlias;
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use std::rc::Rc;

pub type DynamicFunctor = DynamicCategory;

impl DynamicFunctor {
    pub fn new_functor(
        id: String,
        source: Rc<DynamicCategory>,
        target: Rc<DynamicCategory>,
        mappings: HashMap<Rc<MorphismAlias<DynamicCategory>>, Rc<MorphismAlias<DynamicCategory>>>,
    ) -> Result<Self, Errors> {
        todo!()
    }
}
impl ArrowTrait for DynamicCategory {
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

impl FunctorTrait for DynamicCategory {
    fn arrow_mappings(
        &self,
    ) -> &HashMap<Rc<MorphismAlias<Self::SourceObject>>, Rc<MorphismAlias<Self::TargetObject>>>
    {
        todo!()
    }
}
