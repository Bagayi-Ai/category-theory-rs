use crate::core::dynamic_category::dynamic_morphism::DynamicMorphism;
use crate::core::dynamic_category::dynamic_value::DynamicValue;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicType {
    Category,
    Morphism,
    Functor,
}

pub type DynamicCategoryTypeAlias =
    dyn CategoryTrait<Object = DynamicCategory, Morphism = DynamicMorphism>;

#[derive(Debug)]
pub struct DynamicCategory {
    id: DynamicValue,
    objects: HashMap<DynamicValue, Rc<DynamicCategory>>,
    object_mapping: HashMap<DynamicValue, HashSet<Rc<DynamicMorphism>>>,
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
            object_mapping: HashMap::new(),
            morphisms: HashMap::new(),
            dynamic_type: DynamicType::Category,
        }
    }

    pub fn new() -> Self {
        Self::new_with_id(DynamicValue::Str(uuid::Uuid::new_v4().to_string()))
    }

    // pub fn new_arrow(source: &Rc<Self>, target: &Rc<Self>) -> Result<Self, Errors> {
    //     let mut category = Self::new_with_id(DynamicValue::Str(uuid::Uuid::new_v4().to_string()));
    //     category.dynamic_type = DynamicType::Arrow;
    //     category.objects.insert(source.id().clone(), source.clone());
    //     category.objects.insert(target.id().clone(), target.clone());
    //     Ok(category)
    // }

    pub fn id(&self) -> &DynamicValue {
        &self.id
    }

    pub fn dynamic_type(&self) -> &DynamicType {
        &self.dynamic_type
    }
}

impl PartialEq for DynamicCategory {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for DynamicCategory {}

impl Hash for DynamicCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl CategoryTrait for DynamicCategory {
    type Object = DynamicCategory;
    type Morphism = DynamicMorphism;

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        if self.dynamic_type != DynamicType::Category {
            return Err(Errors::InvalidOperation(
                "Cannot add object to an arrow category".to_string(),
            ));
        }

        if self.objects.contains_key(object.id()) {
            return Err(Errors::ObjectAlreadyExists);
        }
        self.objects.insert(object.id().clone(), object.clone());
        let identity_cell = DynamicMorphism::new_identity_morphism(object.clone());
        self.object_mapping
            .entry(object.id().clone())
            .or_default()
            .insert(identity_cell.clone());
        self.add_morphism(identity_cell)?;
        Ok(())
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
        object_id: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        todo!()
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
        // let mut category = DynamicCategory::new();
        // for object in objects {
        //     let object = DynamicCategory::new_with_id(object.into());
        //     category.add_object(Rc::new(object)).unwrap();
        // }
        // category
        todo!()
    }
}

impl From<Vec<Rc<DynamicCategory>>> for DynamicCategory {
    fn from(value: Vec<Rc<DynamicCategory>>) -> Self {
        // let mut category = DynamicCategory::new();
        // for object in value {
        //     category.add_object(object).unwrap();
        // }
        // category
        todo!()
    }
}

impl From<Vec<Rc<DynamicCategoryTypeAlias>>> for DynamicCategory {
    fn from(value: Vec<Rc<DynamicCategoryTypeAlias>>) -> Self {
        todo!()
    }
}
