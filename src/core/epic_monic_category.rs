use crate::core::errors::Errors;
use crate::core::morphism::Morphism;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::factorization_system_trait::FactorizationSystemTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::utils;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq,
{
    category: InnerCategory,
    morphism_factors: HashMap<
        Rc<Morphism<InnerCategory::Object>>,
        (
            Rc<Morphism<InnerCategory::Object>>,
            Rc<Morphism<InnerCategory::Object>>,
        ),
    >,
}

impl<InnerCategory> Default for EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<InnerCategory> EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + 'static,
{
    pub fn new() -> Self {
        EpicMonicCategory {
            category: InnerCategory::new(),
            morphism_factors: HashMap::new(),
        }
    }

    pub fn category(&self) -> &InnerCategory {
        &self.category
    }

    fn factorize(
        &mut self,
        morphism: &Morphism<InnerCategory::Object>,
    ) -> Result<
        (
            Morphism<InnerCategory::Object>,
            Morphism<InnerCategory::Object>,
        ),
        Errors,
    > {
        // we factorize to image of f and from image of f to target object
        let source_object = morphism.source_object();
        let target_object = morphism.target_object();
        let mappings = morphism.functor()?.arrow_mappings();

        let mut all_target_objects = target_object.get_all_objects()?.clone();

        let mut image_object = InnerCategory::Object::new();
        let mut image_mapping = HashMap::new();
        // possibility the target object is the image of source object
        let mut target_as_image = false;

        // epic can also be just the target object if all objects are in the image
        for (source_morphism, target_morphism) in mappings {
            if source_morphism.is_identity() {
                if !target_morphism.is_identity() {
                    return Err(Errors::InvalidArrowNoFunctorFound);
                }
                // remove it from all_target_objects if it exist
                if target_as_image && all_target_objects.contains(&target_morphism.target_object())
                {
                    all_target_objects.remove(&target_morphism.target_object());
                } else {
                    target_as_image = false;
                }

                // if object has already been added to image_object
                if let Ok(object) = image_object.get_object(&**target_morphism.target_object()) {
                    let target_morphism = image_object.get_identity_morphism(&**object)?;
                    image_mapping.insert((*source_morphism).clone(), target_morphism.clone());
                } else {
                    image_object.add_object(target_morphism.target_object().clone())?;
                    image_mapping.insert((*source_morphism).clone(), (*target_morphism).clone());
                }
            } else {
                panic!("to implement later")
            }
        }
        // let image_object = if target_as_image && !all_target_objects.is_empty() {
        //     target_object.clone()
        // } else {
        //     let result = Rc::new(image_object);
        //     self.category.add_object(result.clone())?;
        //     result
        // };

        // add the image object to the category
        let image_object = Rc::new(image_object);
        self.category.add_object(image_object.clone())?;
        let epic_morphism =
            Morphism::new_with_mappings(source_object.clone(), image_object.clone(), image_mapping);

        // now mapping from image to target object
        let monic_mapping = {
            let mut monic_mapping = HashMap::new();
            for obj in image_object.get_all_objects()? {
                let source_morphism = image_object.get_identity_morphism(&**obj)?;
                let target_morphism = target_object.get_identity_morphism(&**obj)?;
                monic_mapping.insert(source_morphism.clone(), target_morphism.clone());
            }
            monic_mapping
        };

        let monic_morphism =
            Morphism::new_with_mappings(image_object.clone(), target_object.clone(), monic_mapping);
        Ok((epic_morphism, monic_morphism))
    }
}

impl<InnerCategory> CategoryTrait for EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone + 'static,
    <InnerCategory as CategoryTrait>::Object: Clone,
{
    type Object = InnerCategory::Object;

    fn new() -> Self {
        todo!()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        self.category.category_id()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        self.category.add_object(object)
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Morphism<InnerCategory::Object>>,
    ) -> Result<&Rc<Morphism<InnerCategory::Object>>, Errors> {
        // here we need to factor it to epic and monic morphisms
        let (epic, monic) = self.factorize(&morphism)?;
        // self.hash_map.insert(morphism.id().clone(), (epic, monic));
        self.morphism_factors
            .insert(morphism.clone(), (epic.into(), monic.into()));
        self.category.add_morphism(morphism)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.category.get_object(object)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        self.category.get_all_objects()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Morphism<InnerCategory::Object>>>, Errors> {
        self.category.get_all_morphisms()
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Morphism<InnerCategory::Object>>>, Errors> {
        self.category.get_hom_set_x(source_object)
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Object,
    ) -> Result<Vec<&Rc<Morphism<InnerCategory::Object>>>, Errors> {
        self.category.get_object_morphisms(object_id)
    }
}

impl<InnerCategory> FactorizationSystemTrait for EpicMonicCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone + 'static,
    <InnerCategory as CategoryTrait>::Object: Clone,
{
    fn morphism_factors(
        &self,
        morphism: &Morphism<InnerCategory::Object>,
    ) -> Result<
        &(
            Rc<Morphism<InnerCategory::Object>>,
            Rc<Morphism<InnerCategory::Object>>,
        ),
        Errors,
    > {
        self.morphism_factors
            .get(morphism)
            .ok_or(Errors::InvalidFactorization)
    }
}

impl<Object: CategoryTrait + Hash + Eq> Hash for EpicMonicCategory<Object> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.category.hash(state);
    }
}
