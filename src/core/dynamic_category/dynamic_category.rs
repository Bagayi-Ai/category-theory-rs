use crate::core::dynamic_category::dynamic_value::DynamicValue;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::core::morphism::Morphism;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::dynamic_category::dynamic_morphism::DynamicMorphism;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicType {
    Category,
    Functor,
}

pub type DynamicCategoryTypeAlias =
    dyn CategoryTrait<Object = DynamicCategory, Morphism = DynamicMorphism>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DynamicCategory {
    id: DynamicValue,
    objects: HashMap<Rc<DynamicCategory>, HashSet<Rc<DynamicMorphism>>>,
    morphisms: HashMap<String, Rc<DynamicMorphism>>,
    dynamic_type: DynamicType,
}

impl Default for DynamicCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicCategory {
    pub fn new_with_id(id: DynamicValue) -> Self {
        DynamicCategory {
            id,
            objects: HashMap::new(),
            morphisms: HashMap::new(),
            dynamic_type: DynamicType::Category,
        }
    }

    pub fn new() -> Self {
        Self::new_with_id(DynamicValue::Str(uuid::Uuid::new_v4().to_string()))
    }

    pub fn new_functor(
        id: String,
        source: Rc<DynamicCategory>,
        target: Rc<DynamicCategory>,
        mappings: HashMap<Rc<MorphismAlias<DynamicCategory>>, Rc<MorphismAlias<DynamicCategory>>>,
    ) -> Result<Self, Errors> {
        let mut result = DynamicCategory::new_with_id(id.into());
        result.dynamic_type = DynamicType::Functor;

        // a functor is technically an object in the target category.


        Ok(result)
    }

    pub fn id(&self) -> &DynamicValue {
        &self.id
    }

    pub fn dynamic_type(&self) -> &DynamicType {
        &self.dynamic_type
    }

    pub fn expecting_category_type(&self) -> Result<(), Errors> {
        if self.dynamic_type != DynamicType::Category {
            return Err(Errors::InvalidOperation(
                "Expected a category type".to_string(),
            ));
        }
        Ok(())
    }
}

impl Hash for DynamicCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl CategoryTrait for DynamicCategory {
    type Object = DynamicCategory;
    type Morphism = DynamicMorphism;

    fn new() -> Self
    where
        Self: Sized,
    {
        DynamicCategory::new()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        self.expecting_category_type()?;
        if self.objects.contains_key(&object) {
            return Err(Errors::ObjectAlreadyExists);
        }
        let identity_cell = DynamicMorphism::new_identity_morphism(object.clone());
        self.objects
            .entry(object)
            .or_default()
            .insert(identity_cell.clone());
        self.add_morphism(identity_cell)?;
        Ok(())
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Self::Morphism>,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        self.expecting_category_type()?;
        if self.morphisms.contains_key(morphism.id()) {
            return Err(Errors::MorphismAlreadyExists);
        }
        // validate target object is part of the category
        if !self.objects.contains_key(morphism.target_object()) {
            return Err(Errors::ObjectNotFound);
        }

        // if its not identity morphism add it to the objects as part of the hom-set
        if !morphism.is_identity() {
            self.objects
                .get_mut(morphism.source_object())
                .ok_or(Errors::ObjectNotFound)?
                .insert(morphism.clone());
        }

        let cell = self
            .morphisms
            .entry(morphism.id().clone())
            .or_insert(morphism);
        Ok(cell)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.expecting_category_type()?;
        self.objects
            .get_key_value(object)
            .map(|(k, _)| k)
            .ok_or(Errors::ObjectNotFound)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        self.expecting_category_type()?;
        Ok(self.objects.keys().collect())
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        Ok(self.morphisms.values().collect())
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        let result = self
            .objects
            .get(source_object)
            .ok_or(Errors::ObjectNotFound)?
            .iter()
            .collect::<HashSet<_>>();
        Ok(result)
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        let result = self.objects.get(object).ok_or(Errors::ObjectNotFound)?;
        Ok(result.iter().collect())
    }
}

impl<T: Eq + Clone + Hash + Debug> From<T> for DynamicCategory
where
    T: Into<DynamicValue>,
{
    fn from(value: T) -> Self {
        DynamicCategory::new_with_id(value.into())
    }
}

impl<T: Eq + Clone + Hash + Debug> From<Vec<T>> for DynamicCategory
where
    T: Into<DynamicValue>,
{
    fn from(objects: Vec<T>) -> Self {
        let mut category = DynamicCategory::new();
        for object in objects {
            let object = DynamicCategory::new_with_id(object.into());
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}

impl From<Vec<Rc<DynamicCategory>>> for DynamicCategory {
    fn from(value: Vec<Rc<DynamicCategory>>) -> Self {
        let mut category = DynamicCategory::new();
        for object in value {
            category.add_object(object).unwrap();
        }
        category
    }
}

