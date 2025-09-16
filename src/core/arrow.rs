use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use dyn_clone::DynClone;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type Morphism<Object: CategoryTrait> = Arrow<Object, Object>;

pub type Functor<SourceCategory, TargetCategory> = Arrow<SourceCategory, TargetCategory>;

#[derive(Clone)]
pub struct Arrow<SourceObject: CategoryTrait, TargetObject: CategoryTrait> {
    id: String,
    source_object: Rc<SourceObject>,
    target_object: Rc<TargetObject>,
    // map arrows in source category to arrows in target category
    mappings: HashMap<
        Rc<SourceObject::Morphism>, // indirection avoids infinite size
        Rc<TargetObject::Morphism>,
    >,
    is_identity: bool,
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Debug
    for Arrow<SourceObject, TargetObject>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arrow")
            .field("id", &self.id)
            .field("source_object", &self.source_object)
            .field("target_object", &self.target_object)
            .field("is_identity", &self.is_identity)
            .finish()
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Hash
    for Arrow<SourceObject, TargetObject>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        Rc::as_ptr(&self.source_object).hash(state);
        Rc::as_ptr(&self.target_object).hash(state);
        self.is_identity.hash(state);
        // todo hash mappings pointer as well
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> PartialEq
    for Arrow<SourceObject, TargetObject>
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && Rc::ptr_eq(&self.source_object, &other.source_object)
            && Rc::ptr_eq(&self.target_object, &other.target_object)
            && self.is_identity == other.is_identity
        // todo compare mappings pointer as well
    }
}

impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Eq
    for Arrow<SourceObject, TargetObject>
{
}

impl<Object: CategoryTrait> Arrow<Object, Object> {
    pub fn new_identity(object: Rc<Object>) -> Rc<Self> {
        Rc::new(Arrow {
            id: String::generate(),
            source_object: object.clone(),
            target_object: object,
            mappings: HashMap::new(),
            is_identity: true,
        })
    }
}
impl<SourceObject: CategoryTrait, TargetObject: CategoryTrait> Arrow<SourceObject, TargetObject> {
    pub fn new(
        id: String,
        source_object: Rc<SourceObject>,
        target_object: Rc<TargetObject>,
        mappings: HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>>,
    ) -> Self {
        Arrow {
            id,
            source_object,
            target_object,
            mappings,
            is_identity: false,
        }
    }

    pub fn new_with_mappings(
        source_object: Rc<SourceObject>,
        target_object: Rc<TargetObject>,
        mappings: HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>>,
    ) -> Self {
        Arrow {
            id: String::generate(),
            source_object,
            target_object,
            mappings,
            is_identity: false,
        }
    }
}

impl<SourceObject, TargetObject> ArrowTrait<SourceObject, TargetObject>
    for Arrow<SourceObject, TargetObject>
where
    SourceObject: CategoryTrait,
    TargetObject: CategoryTrait,
{
    fn source_object(&self) -> &Rc<SourceObject> {
        &self.source_object
    }

    fn target_object(&self) -> &Rc<TargetObject> {
        &self.target_object
    }

    fn is_identity(&self) -> bool {
        self.is_identity
    }

    fn arrow_id(&self) -> &String {
        &self.id
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<Rc<Arrow<SourceObject, TargetObject>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Arrow<SourceObject, TargetObject>> {
        todo!()
    }

    fn arrow_mappings(&self) -> &HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>> {
        // This is a bit tricky because we need to convert the HashMap types
        // We can do this by creating a new HashMap and copying the values over
        // but this is not very efficient
        // A better way would be to use a wrapper type that implements the required trait
        // but for simplicity we will use the first approach here
        &self.mappings
    }

    fn validate_mappings(&self) -> Result<(), Errors> {
        /*
        Functor should validate that all objects in the source category
        have a corresponding object in the target category.

        And that all morphisms in the source category are mapped to morphisms in the target category,
        such that they commute with morphism in the target category.
        i.e        for each morphism f: A -> B in the source category,
        there exists a morphism f': F(A) -> F(B) in the target category such that
        F(B) ∘ F(f) = F(f') ∘ F(A)
         */

        // start with checking if all objects in the source category have a corresponding object in the target category
        let mapping = self.arrow_mappings();
        for source_object in self.source_object().get_all_objects()? {
            let identity_morphism = self
                .source_object()
                .get_identity_morphism(&**source_object)?;

            // a -> F(a)
            let mapped_identity_morphism =
                mapping
                    .get(identity_morphism)
                    .ok_or(Errors::InvalidFunctor(
                        "No functor found for identity morphism".to_string(),
                    ))?;

            // now get the hom-set for the source object
            let hom_set_x = self.source_object().get_hom_set_x(&**source_object)?;

            for morphism in hom_set_x {
                if morphism.is_identity() {
                    // just check its identity mapping
                    if !mapping.contains_key(morphism) {
                        return Err(Errors::InvalidFunctor(
                            "No functor found for identity morphism".to_string(),
                        ));
                    }
                } else {
                    // F(a) -> F(b)
                    let target_morphism = mapping.get(morphism).ok_or(Errors::InvalidFunctor(
                        "No functor found for morphism".to_string(),
                    ))?;
                    // now we check the commutation condition
                    // from the source object to the target object to the mapped target object
                    // a -> F(a) -> F(b)
                    let first_path = mapped_identity_morphism.compose(&**target_morphism)?;

                    let identity_of_target = self
                        .source_object()
                        .get_identity_morphism(&**morphism.target_object())?;
                    // b -> F(b)
                    let mapped_identity_target_morphism =
                        mapping
                            .get(identity_of_target)
                            .ok_or(Errors::InvalidFunctor(
                                "No functor found for identity morphism in target".to_string(),
                            ))?;
                    // // a -> b -> F(b)
                    // let second_path = morphism.compose(&**mapped_identity_target_morphism)?;
                    //
                    // first_path.validate_commutation(&*second_path)?;
                }
            }
        }
        Ok(())
    }
}

impl<SourceObject, TargetObject> CategoryTrait for Arrow<SourceObject, TargetObject>
where
    SourceObject: CategoryTrait + Clone,
    TargetObject: CategoryTrait + Eq + Hash + Clone,
    <SourceObject as CategoryTrait>::Morphism: Clone,
    <TargetObject as CategoryTrait>::Morphism: Clone,
{
    type Object = TargetObject;
    type Morphism = Morphism<Self::Object>;

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        self.target_object.category_id()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        todo!()
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Self::Morphism>,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        todo!()
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }
}
