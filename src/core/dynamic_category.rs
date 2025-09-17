use crate::core::arrow::{Arrow, Functor, Morphism};
use crate::core::base_category::BaseCategory;
use crate::core::epic_monic_category::EpicMonicCategory;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use crate::core::traits::factorization_system_trait::FactorizationSystemTrait;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicType {
    Category,
    Functor,
    EpicMonicCategory,
}

pub enum DynamicCategoryEnum {
    Category(
        Box<dyn CategoryTrait<Object = DynamicCategory, Morphism = Morphism<DynamicCategory>>>,
    ),
    EpicMonicCategory(
        Box<
            dyn FactorizationSystemTrait<
                    Object = DynamicCategory,
                    Morphism = Morphism<DynamicCategory>,
                >,
        >,
    ),
}

impl Clone for DynamicCategoryEnum {
    fn clone(&self) -> Self {
        match self {
            DynamicCategoryEnum::Category(cat) => {
                DynamicCategoryEnum::Category(dyn_clone::clone_box(&**cat))
            }
            DynamicCategoryEnum::EpicMonicCategory(cat) => {
                DynamicCategoryEnum::EpicMonicCategory(dyn_clone::clone_box(&**cat))
            }
        }
    }
}

#[derive(Clone)]
pub struct DynamicCategory {
    inner_category: DynamicCategoryEnum,
    dynamic_type: DynamicType,
    functor: Option<Rc<Arrow<DynamicCategory, DynamicCategory>>>,
}

impl Debug for DynamicCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
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
    pub fn new_with_id(id: ObjectId) -> Self {
        DynamicCategory {
            inner_category: DynamicCategoryEnum::Category(Box::new(BaseCategory::new_with_id(
                id.clone(),
            ))),
            dynamic_type: DynamicType::Category,
            functor: None,
        }
    }

    pub fn new() -> Self {
        Self::new_with_id(ObjectId::Str(uuid::Uuid::new_v4().to_string()))
    }

    pub fn new_epic_monic_category(id: ObjectId) -> Result<Self, Errors> {
        let epic_monic_category = EpicMonicCategory::<DynamicCategory>::new();
        let mut result = DynamicCategory::new_with_id(id);
        result.dynamic_type = DynamicType::EpicMonicCategory;
        result.inner_category =
            DynamicCategoryEnum::EpicMonicCategory(Box::new(epic_monic_category));
        Ok(result)
    }

    pub fn functor_to_category(
        functor: Rc<Functor<DynamicCategory, DynamicCategory>>,
    ) -> Result<Self, Errors> {
        let mut result = DynamicCategory::new_with_id(functor.arrow_id().clone().into());
        result.dynamic_type = DynamicType::Functor;
        result.functor = Some(functor);
        Ok(result)
    }

    pub fn id(&self) -> &ObjectId {
        &self.inner_category().category_id()
    }

    pub fn dynamic_type(&self) -> &DynamicType {
        &self.dynamic_type
    }

    pub fn inner_category(
        &self,
    ) -> &dyn CategoryTrait<Object = DynamicCategory, Morphism = Morphism<DynamicCategory>> {
        match &self.inner_category {
            DynamicCategoryEnum::Category(cat) => cat.as_ref(),
            DynamicCategoryEnum::EpicMonicCategory(cat) => cat.as_ref(),
        }
    }

    pub fn inner_category_mut(
        &mut self,
    ) -> &mut dyn CategoryTrait<Object = DynamicCategory, Morphism = Morphism<DynamicCategory>>
    {
        match &mut self.inner_category {
            DynamicCategoryEnum::Category(cat) => cat.as_mut(),
            DynamicCategoryEnum::EpicMonicCategory(cat) => cat.as_mut(),
        }
    }

    pub fn inner_factorization_system(
        &self,
    ) -> Option<
        &dyn FactorizationSystemTrait<
            Object = DynamicCategory,
            Morphism = Morphism<DynamicCategory>,
        >,
    > {
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
    type Object = DynamicCategory;

    type Morphism = Morphism<DynamicCategory>;

    fn new() -> Self
    where
        Self: Sized,
    {
        DynamicCategory::new()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        DynamicCategory::new_with_id(id.clone())
    }

    fn category_id(&self) -> &ObjectId {
        self.id()
    }

    fn update_category_id(&mut self, new_id: ObjectId) {
        self.inner_category_mut().update_category_id(new_id);
    }

    fn add_object(&mut self, object: Rc<DynamicCategory>) -> Result<Rc<Self::Morphism>, Errors> {
        self.inner_category_mut().add_object(object)
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Morphism<Self::Object>>,
    ) -> Result<&Rc<Morphism<Self::Object>>, Errors> {
        self.inner_category_mut().add_morphism(morphism)
    }

    fn get_object(&self, object: &DynamicCategory) -> Result<&Rc<DynamicCategory>, Errors> {
        self.inner_category().get_object(object)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<DynamicCategory>>, Errors> {
        self.inner_category().get_all_objects()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        self.inner_category().get_all_morphisms()
    }

    fn get_hom_set_x(
        &self,
        source_object: &DynamicCategory,
    ) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        self.inner_category().get_hom_set_x(source_object)
    }

    fn get_object_morphisms(
        &self,
        object: &DynamicCategory,
    ) -> Result<Vec<&Rc<Morphism<Self::Object>>>, Errors> {
        self.inner_category().get_object_morphisms(object)
    }
}

impl FactorizationSystemTrait for DynamicCategory {
    fn morphism_factors(
        &self,
        morphism: &Morphism<Self::Object>,
    ) -> Result<&(Rc<Morphism<Self::Object>>, Rc<Morphism<Self::Object>>), Errors> {
        if let Some(factorization_system) = &self.inner_factorization_system() {
            factorization_system.morphism_factors(morphism)
        } else {
            Err(Errors::InvalidOperation(
                "This category does not support factorization systems".to_string(),
            ))
        }
    }
}

impl From<ObjectId> for DynamicCategory {
    fn from(value: ObjectId) -> Self {
        DynamicCategory::new_with_id(value)
    }
}

impl From<String> for DynamicCategory {
    fn from(s: String) -> Self {
        DynamicCategory::new_with_id(ObjectId::Str(s))
    }
}

impl From<&str> for DynamicCategory {
    fn from(s: &str) -> Self {
        DynamicCategory::new_with_id(ObjectId::Str(s.to_string()))
    }
}

impl From<i32> for DynamicCategory {
    fn from(n: i32) -> Self {
        DynamicCategory::new_with_id(ObjectId::Int(n))
    }
}

impl<T: Eq + Clone + Hash + Debug> From<Vec<T>> for DynamicCategory
where
    T: Into<ObjectId>,
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

impl From<Rc<DynamicCategory>> for DynamicCategory {
    fn from(rc: Rc<DynamicCategory>) -> Self {
        (*rc).clone()
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

// impl<C> From<C> for DynamicCategory
// where
//     BaseCategory: From<C>,
// {
//     fn from(value: C) -> Self {
//         // First convert to the inner Category to obtain its id
//         let inner: BaseCategory = value.into();
//         let mut category = DynamicCategory::new_with_id(inner.id().clone());
//         category.inner_category = DynamicCategoryEnum::Category(Box::new(inner));
//         category
//     }
// }
