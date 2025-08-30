use crate::core::category::Category;
use crate::core::dynamic_value::DynamicValue;
use crate::core::epic_monic_category::EpicMonicCategory;
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::factorization_system_trait::FactorizationSystemTrait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicType {
    Category,
    Functor,
    EpicMonicCategory,
}

pub type DynamicMorphism = Morphism<DynamicValue, DynamicCategory>;

pub type DynamicFunctor = Functor<DynamicValue, DynamicCategory, DynamicCategory>;

pub type DynamicCategoryTypeAlias =
    dyn CategoryTrait<Id = DynamicValue, Object = DynamicCategory, Morphism = DynamicMorphism>;

pub type DynamicFactorizationSystem = dyn FactorizationSystemTrait<
        Id = DynamicValue,
        Object = DynamicCategory,
        Morphism = DynamicMorphism,
    >;

#[derive(Debug)]
pub enum DynamicCategoryEnum {
    Category(Box<DynamicCategoryTypeAlias>),
    EpicMonicCategory(Box<DynamicFactorizationSystem>),
}

#[derive(Debug)]
pub struct DynamicCategory {
    inner_category: DynamicCategoryEnum,
    dynamic_type: DynamicType,
    functor: Option<Rc<DynamicFunctor>>,
}

impl Default for DynamicCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for DynamicCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for DynamicCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for DynamicCategory {}

impl DynamicCategory {
    pub fn new_with_id(id: DynamicValue) -> Self {
        DynamicCategory {
            inner_category: DynamicCategoryEnum::Category(Box::new(Category::new_with_id(
                id.clone(),
            ))),
            dynamic_type: DynamicType::Category,
            functor: None,
        }
    }

    pub fn new() -> Self {
        Self::new_with_id(DynamicValue::Str(uuid::Uuid::new_v4().to_string()))
    }

    pub fn new_epic_monic_category(id: DynamicValue) -> Result<Self, Errors> {
        let epic_monic_category = EpicMonicCategory::new();
        let mut result = DynamicCategory::new_with_id(id);
        result.dynamic_type = DynamicType::EpicMonicCategory;
        result.inner_category =
            DynamicCategoryEnum::EpicMonicCategory(Box::new(epic_monic_category));
        Ok(result)
    }

    pub fn functor_to_category(functor: Rc<DynamicFunctor>) -> Result<Self, Errors> {
        let mut result = DynamicCategory::new_with_id(functor.id().clone().into());
        result.dynamic_type = DynamicType::Functor;
        result.functor = Some(functor);
        Ok(result)
    }

    pub fn from_category_impl<C>(c: C) -> Self
    where
        C: CategoryTrait<Object = DynamicCategory, Morphism = DynamicMorphism, Id = DynamicValue>
            + 'static,
    {
        DynamicCategory {
            inner_category: DynamicCategoryEnum::Category(Box::new(c)),
            dynamic_type: DynamicType::Category,
            functor: None,
        }
    }

    pub fn id(&self) -> &DynamicValue {
        &self.inner_category().category_id()
    }

    pub fn dynamic_type(&self) -> &DynamicType {
        &self.dynamic_type
    }

    pub fn inner_category(&self) -> &DynamicCategoryTypeAlias {
        match &self.inner_category {
            DynamicCategoryEnum::Category(cat) => cat.as_ref(),
            DynamicCategoryEnum::EpicMonicCategory(cat) => cat.as_ref(),
        }
    }

    pub fn inner_category_mut(&mut self) -> &mut DynamicCategoryTypeAlias {
        match &mut self.inner_category {
            DynamicCategoryEnum::Category(cat) => cat.as_mut(),
            DynamicCategoryEnum::EpicMonicCategory(cat) => cat.as_mut(),
        }
    }

    pub fn inner_factorization_system(&self) -> Option<&DynamicFactorizationSystem> {
        match &self.inner_category {
            DynamicCategoryEnum::EpicMonicCategory(cat) => Some(cat.as_ref()),
            _ => None,
        }
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

impl CategoryTrait for DynamicCategory {
    type Id = DynamicValue;
    type Object = DynamicCategory;
    type Morphism = DynamicMorphism;

    fn new() -> Self
    where
        Self: Sized,
    {
        DynamicCategory::new()
    }

    fn category_id(&self) -> &Self::Id {
        self.id()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        self.inner_category_mut().add_object(object)
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Self::Morphism>,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        self.inner_category_mut().add_morphism(morphism)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.inner_category().get_object(object)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        self.inner_category().get_all_objects()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        self.inner_category().get_all_morphisms()
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        self.inner_category().get_hom_set_x(source_object)
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        self.inner_category().get_object_morphisms(object)
    }
}

impl FactorizationSystemTrait for DynamicCategory {
    fn morphism_factors(
        &self,
        morphism: &Self::Morphism,
    ) -> Result<&(Rc<Self::Morphism>, Rc<Self::Morphism>), Errors> {
        if let Some(factorization_system) = &self.inner_factorization_system() {
            factorization_system.morphism_factors(morphism)
        } else {
            Err(Errors::InvalidOperation(
                "This category does not support factorization systems".to_string(),
            ))
        }
    }
}

impl From<DynamicValue> for DynamicCategory {
    fn from(value: DynamicValue) -> Self {
        DynamicCategory::new_with_id(value)
    }
}

impl From<String> for DynamicCategory {
    fn from(s: String) -> Self {
        DynamicCategory::new_with_id(DynamicValue::Str(s))
    }
}

impl From<&str> for DynamicCategory {
    fn from(s: &str) -> Self {
        DynamicCategory::new_with_id(DynamicValue::Str(s.to_string()))
    }
}

impl From<i32> for DynamicCategory {
    fn from(n: i32) -> Self {
        DynamicCategory::new_with_id(DynamicValue::Int(n))
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

impl<C> From<C> for DynamicCategory
where
    Category<DynamicValue, DynamicCategory>: From<C>,
{
    fn from(value: C) -> Self {
        // First convert to the inner Category to obtain its id
        let inner: Category<DynamicValue, DynamicCategory> = value.into();
        let mut category = DynamicCategory::new_with_id(inner.id().clone());
        category.inner_category = DynamicCategoryEnum::Category(Box::new(inner));
        category
    }
}
