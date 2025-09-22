use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, LazyLock};

pub trait ArrowTrait<SourceObject: CategoryTrait, TargetObject: CategoryTrait>:
    Eq + Hash + Send + Sync
{
    fn source_object(&self) -> &Arc<SourceObject>;

    fn target_object(&self) -> &Arc<TargetObject>;

    fn new_instance(
        source: Arc<SourceObject>,
        target: Arc<TargetObject>,
        id: &str,
        mappings: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized;

    fn new(
        id: String,
        source: Arc<SourceObject>,
        target: Arc<TargetObject>,
        mappings: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized;

    fn new_with_mappings(
        source_object: Arc<SourceObject>,
        target_object: Arc<TargetObject>,
        mappings: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::new(String::generate(), source_object, target_object, mappings)
    }

    fn is_identity(&self) -> bool;

    fn arrow_id(&self) -> &String;

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<Arc<impl ArrowTrait<SourceObject, TargetObject>>, Errors>;

    // for handling composition of arrows
    // for single arrow just return itself
    fn arrows(&self) -> Vec<&impl ArrowTrait<SourceObject, TargetObject>>;

    fn functor(&self) -> Option<&impl FunctorTrait<SourceObject, TargetObject>>;

    fn arrow_mappings(
        &self,
    ) -> Option<&HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>> {
        self.functor().map(|f| f.morphisms_mappings())
    }

    fn validate_composition(&self) -> Result<(), Errors> {
        todo!()
    }

    fn validate_commutation(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<(), Errors> {
        todo!()
    }

    fn is_isomorphism(&self) -> bool {
        todo!()
    }
}
