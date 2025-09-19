use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::{Arc, LazyLock};
use crate::core::traits::functor_trait::FunctorTrait;

pub type Morphism<Object: CategoryTrait> = Arrow<Object, Object>;

// pub type Functor<SourceCategory, TargetCategory> = Arrow<SourceCategory, TargetCategory>;

pub struct Arrow<SourceObject: CategoryTrait, TargetObject: CategoryTrait> {
    id: ObjectId,
    source_object: Arc<SourceObject>,
    target_object: Arc<TargetObject>,
    is_identity: bool,
    functor: Option<Arc<Functor<SourceObject, TargetObject>>>,
    // place holder for empty mapping
    // in case the functor is None
    empty_map: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Clone
    for Arrow<SourceObject, TargetObject>
{
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Debug
    for Arrow<SourceObject, TargetObject>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arrow")
            .field("id", &self.id)
            .field("source_object", self.source_object.category_id())
            .field("target_object", self.target_object.category_id())
            .field("is_identity", &self.is_identity)
            .finish()
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Hash
    for Arrow<SourceObject, TargetObject>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        Arc::as_ptr(&self.source_object).hash(state);
        Arc::as_ptr(&self.target_object).hash(state);
        self.is_identity.hash(state);
        // todo hash mappings pointer as well
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> PartialEq
    for Arrow<SourceObject, TargetObject>
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && Arc::ptr_eq(&self.source_object, &other.source_object)
            && Arc::ptr_eq(&self.target_object, &other.target_object)
            && self.is_identity == other.is_identity
        // todo compare mappings pointer as well
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Eq
    for Arrow<SourceObject, TargetObject>
{
}

impl<Object: CategoryTrait> Arrow<Object, Object> {
    pub fn new_identity(object: Arc<Object>) -> Arc<Self> {
        Arc::new(Arrow {
            id: ObjectId::Str(String::generate()),
            source_object: object.clone(),
            target_object: object,
            functor: None,
            is_identity: true,
            empty_map: HashMap::new(),
        })
    }
}
impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Arrow<SourceObject, TargetObject> {
    pub fn new(
        id: String,
        source_object: Arc<SourceObject>,
        target_object: Arc<TargetObject>,
        functor: Option<Arc<Functor<SourceObject, TargetObject>>>,
    ) -> Self {
        Arrow {
            id: ObjectId::Str(id),
            source_object,
            target_object,
            functor,
            is_identity: false,
            empty_map: HashMap::new(),
        }
    }

    pub fn new_with_mappings(
        source_object: Arc<SourceObject>,
        target_object: Arc<TargetObject>,
        mappings: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
    ) -> Self {
        Arrow {
            id: ObjectId::Str(String::generate()),
            source_object: source_object.clone(),
            target_object: target_object.clone(),
            functor: Some(Arc::new(Functor::new(
                String::generate(),
                source_object.clone(),
                target_object.clone(),
                mappings.clone(),
            ))),
            is_identity: false,
            empty_map: HashMap::new(),
        }
    }

    pub fn get_functor(&self) -> Option<&Arc<Functor<SourceObject, TargetObject>>> {
        self.functor.as_ref()
    }
}

impl<SourceObject, TargetObject> ArrowTrait<SourceObject, TargetObject>
    for Arrow<SourceObject, TargetObject>
where
    SourceObject: CategoryTrait,
    TargetObject: CategoryTrait,
{
    fn source_object(&self) -> &Arc<SourceObject> {
        &self.source_object
    }

    fn target_object(&self) -> &Arc<TargetObject> {
        &self.target_object
    }

    fn new_instance(
        source: Arc<SourceObject>,
        target: Arc<TargetObject>,
        id: &str,
        mappings: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn new(
        id: String,
        source: Arc<SourceObject>,
        target: Arc<TargetObject>,
        mappings: HashMap<Arc<SourceObject::Morphism>, Arc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized,
    {
        Arrow {
            id: ObjectId::Str(id),
            source_object: source.clone(),
            target_object: target.clone(),
            functor: Some(Arc::new(Functor::new(
                String::generate(),
                source.clone(),
                target.clone(),
                mappings.clone(),
            ))),
            is_identity: false,
            empty_map: HashMap::new(),
        }
    }

    fn is_identity(&self) -> bool {
        self.is_identity
    }

    fn arrow_id(&self) -> &String {
        match &self.id {
            ObjectId::Str(s) => s,
            _ => panic!("Arrow ID is not a string"),
        }
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<Arc<Arrow<SourceObject, TargetObject>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Arrow<SourceObject, TargetObject>> {
        todo!()
    }

    fn functor(&self) -> Option<&Functor<SourceObject, TargetObject>> {
        self.functor.as_deref()
    }
}

#[async_trait]
impl<SourceObject, TargetObject> CategoryTrait for Arrow<SourceObject, TargetObject>
where
    SourceObject: CategoryTrait + Clone,
    TargetObject: CategoryTrait + Eq + Hash + Clone + Sync + Send,
    <SourceObject as CategoryTrait>::Morphism: Clone,
    <TargetObject as CategoryTrait>::Morphism: Clone,
{
    type Object = TargetObject;
    type Morphism = Morphism<Self::Object>;

    async fn new() -> Result<Self, Errors>
    where
        Self: Sized,
    {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        &self.id
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        todo!()
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        todo!()
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        todo!()
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        todo!()
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        todo!()
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        todo!()
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        todo!()
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Self::Morphism>>, Errors> {
        todo!()
    }
}
