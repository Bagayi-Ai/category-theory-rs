use crate::core::arrow::{Arrow, Functor, Morphism};
use crate::core::discrete_category;
use crate::core::discrete_category::DiscreteCategory;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use dyn_clone::DynClone;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct BaseCategory<Object: CategoryTrait> {
    id: ObjectId,
    objects: HashMap<ObjectId, Rc<Object>>,
    object_mappings: HashMap<ObjectId, HashSet<String>>,
    morphism: HashMap<String, Rc<Morphism<Object>>>,
}

impl<Object: CategoryTrait> Debug for BaseCategory<Object> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Object: CategoryTrait + Clone> Hash for BaseCategory<Object> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl<Object: CategoryTrait + Clone> Default for BaseCategory<Object> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Object: CategoryTrait> BaseCategory<Object> {
    pub fn new() -> Self {
        Self::new_with_id(ObjectId::generate())
    }

    pub fn new_with_id(id: ObjectId) -> Self {
        BaseCategory {
            id,
            objects: HashMap::new(),
            object_mappings: HashMap::new(),
            morphism: HashMap::new(),
        }
    }

    pub fn id(&self) -> &ObjectId {
        &self.id
    }
}

impl<Object: CategoryTrait + Hash + Eq + DynClone + std::clone::Clone> CategoryTrait
    for BaseCategory<Object>
{
    type Object = Object;

    type Morphism = Morphism<Self::Object>;

    fn new() -> Self {
        BaseCategory::new()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        BaseCategory::new_with_id(id.clone())
    }

    fn category_id(&self) -> &ObjectId {
        &self.id
    }

    fn update_category_id(&mut self, new_id: ObjectId) {
        self.id = new_id;
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<Rc<Self::Morphism>, Errors> {
        if self.objects.contains_key(object.category_id()) {
            return Err(Errors::ObjectAlreadyExists);
        }
        let object_id = object.category_id();
        self.objects.insert(object_id.clone(), object.clone());
        let identity_cell = Morphism::new_identity(object.clone());
        self.object_mappings
            .entry(object_id.clone())
            .or_default()
            .insert(identity_cell.arrow_id().to_string());
        self.add_morphism(identity_cell.clone())?;
        Ok(identity_cell)
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Morphism<Self::Object>>,
    ) -> Result<&Rc<Morphism<Self::Object>>, Errors> {
        if self.morphism.contains_key(morphism.arrow_id()) {
            return Err(Errors::MorphismAlreadyExists);
        }
        // validate target object is part of the category
        if !self
            .objects
            .contains_key(&morphism.target_object().category_id())
        {
            return Err(Errors::ObjectNotFound);
        }

        // if its not identity morphism add it to the objects as part of the hom-set
        if !morphism.is_identity() {
            self.object_mappings
                .get_mut(&morphism.source_object().category_id())
                .ok_or(Errors::ObjectNotFound)?
                .insert(morphism.arrow_id().to_string());
        }

        let cell = self
            .morphism
            .entry(morphism.arrow_id().clone())
            .or_insert(morphism);
        Ok(cell)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.objects
            .get(object.category_id())
            .ok_or(Errors::ObjectNotFound)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        Ok(self.objects.values().collect())
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let result: HashSet<&Rc<Morphism<Self::Object>>> = HashSet::new();
        // for (_id, cell) in &self.cells {
        //     result.insert(cell);
        // }
        Ok(result)
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        let result = self
            .object_mappings
            .get(source_object.category_id().into())
            .ok_or(Errors::ObjectNotFound)?
            .iter()
            .map(|item| self.morphism.get(item).ok_or(Errors::MorphismNotFound))
            .collect::<Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors>>()?;
        Ok(result)
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Morphism<Self::Object>>>, Errors> {
        let result = self
            .object_mappings
            .get(object.category_id().into())
            .ok_or(Errors::ObjectNotFound)?
            .iter()
            .map(|item| self.morphism.get(item).ok_or(Errors::MorphismNotFound))
            .collect::<Result<Vec<&Rc<Morphism<Self::Object>>>, Errors>>()?;
        Ok(result)
    }
}

impl<T: Eq + Clone + Hash + Debug> From<Vec<T>> for BaseCategory<DiscreteCategory>
where
    T: Into<ObjectId>,
{
    fn from(objects: Vec<T>) -> Self {
        let mut category = BaseCategory::new();
        for object in objects {
            let object = DiscreteCategory::new_with_id(object.into());
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}

impl From<String> for BaseCategory<DiscreteCategory> {
    fn from(object: String) -> Self {
        let discrete_category = DiscreteCategory::new_with_id(object.into());
        let mut category = BaseCategory::new();
        category.add_object(Rc::new(discrete_category)).unwrap();
        category
    }
}
