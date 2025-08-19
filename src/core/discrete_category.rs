use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::unit::unit_morphism::UnitMorphism;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type DiscreteCategoryString = DiscreteCategory<String>;
pub type DiscreteCategoryUsize = DiscreteCategory<usize>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiscreteCategory<T: Identifier> {
    category_id: T,
    // TODO: Find a way of avoiding storing identity cells
    // as we could derive them from the objects.
    cells: Option<HashMap<T, Rc<Self>>>,
    rc_reference: Option<Rc<Self>>,
}

impl<T: Identifier + Display> Display for DiscreteCategory<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.category_id)
    }
}

impl<T: Identifier> Hash for DiscreteCategory<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category_id.hash(state);
        if let Some(cells) = &self.cells {
            for key in cells.keys() {
                key.hash(state);
            }
        }
    }
}

impl<T: Eq + Clone + Debug + Hash + Identifier> DiscreteCategory<T> {
    pub fn new() -> Self {
        let mut result = DiscreteCategory {
            category_id: T::generate(),
            cells: Some(HashMap::new()),
            rc_reference: None,
        };
        result.rc_reference = Some(Rc::new(result.clone()));
        result
    }

    pub fn new_with_id(category_id: T) -> Self {
        let mut result = DiscreteCategory {
            category_id,
            cells: None,
            rc_reference: None,
        };
        result.rc_reference = Some(Rc::new(result.clone()));
        result
    }

    pub fn clone_with_new_id(&self) -> Self {
        Self {
            category_id: T::generate(),
            cells: self.cells.clone(),
            rc_reference: self.rc_reference.clone(),
        }
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + ToString + Display> CategoryTrait
    for DiscreteCategory<T>
{
    type Identifier = T;
    type Object = Self;
    type Morphism = Self;

    fn new() -> Self {
        DiscreteCategory::new()
    }

    fn category_id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        if let Some(cells) = &mut self.cells {
            if cells.contains_key(&object.category_id) {
                return Err(Errors::ObjectAlreadyExists);
            }
            cells.insert(object.category_id.clone(), object);
            Ok(())
        } else {
            self.cells = Some(HashMap::new());
            self.cells
                .as_mut()
                .unwrap()
                .insert(object.category_id.clone(), object);
            Ok(())
        }
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Self::Morphism>,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        Err(Errors::CannotAddMorphismToDiscreteCategory)
    }

    fn get_identity_morphism(&self, object: &Self::Object) -> Result<&Rc<Self::Morphism>, Errors> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(&object.category_id) {
                return Ok(cell);
            }
        }
        Err(Errors::ObjectNotFound)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(&object.category_id) {
                return Ok(cell);
            }
        }
        Err(Errors::ObjectNotFound)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        let result = if let Some(cells) = &self.cells {
            cells.values().collect()
        } else {
            HashSet::new()
        };
        Ok(result)
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        let result = if let Some(cells) = &self.cells {
            cells.values().collect()
        } else {
            HashSet::new()
        };
        Ok(result)
    }

    fn get_hom_set(
        &self,
        source_object: &Self::Object,
        target_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        // only one morphism in discrete category, the identity morphism.
        Ok(HashSet::from([self.get_identity_morphism(source_object)?]))
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_morphism(object)?])
    }

    fn nested_level() -> usize {
        1
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> ArrowTrait for DiscreteCategory<T> {
    type Identifier = T;
    type SourceObject = Self;
    type TargetObject = Self;

    fn arrow_id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn source_object(&self) -> &Rc<Self::SourceObject> {
        self.rc_reference.as_ref().unwrap()
    }

    fn target_object(&self) -> &Rc<Self::TargetObject> {
        self.rc_reference.as_ref().unwrap()
    }

    fn is_identity(&self) -> bool {
        true
    }

    fn arrows(&self) -> Vec<&DiscreteCategory<T>> {
        vec![self]
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<
            SourceObject = Self::SourceObject,
            TargetObject = Self::TargetObject,
            Identifier = Self::Identifier,
        >,
    ) -> Result<DiscreteCategory<T>, Errors> {
        todo!()
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> MorphismTrait for DiscreteCategory<T> {
    fn functor(&self) -> Result<&Rc<DiscreteCategory<T>>, Errors> {
        todo!()
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> FunctorTrait for DiscreteCategory<T> {
    fn functor_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<Rc<MorphismAlias<Self::SourceObject>>, Rc<MorphismAlias<Self::TargetObject>>>
    {
        todo!()
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> From<T> for DiscreteCategory<T> {
    fn from(object: T) -> Self {
        DiscreteCategory::new_with_id(object)
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> From<Vec<T>> for DiscreteCategory<T> {
    fn from(objects: Vec<T>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            let object = DiscreteCategory::new_with_id(object);
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}
impl From<&str> for DiscreteCategory<String> {
    fn from(object: &str) -> Self {
        DiscreteCategory::new_with_id(object.to_string())
    }
}

impl From<Vec<&str>> for DiscreteCategory<String> {
    fn from(objects: Vec<&str>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            let object = DiscreteCategory::new_with_id(object.to_string());
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;

    fn generate_morphism() -> Rc<DiscreteCategory<String>> {
        Rc::new(DiscreteCategory::new_with_id(random_string(5)))
    }

    fn generate_identifier() -> String {
        String::generate()
    }

    #[test]
    pub fn test_base_scenarios() {
        let mut category = DiscreteCategory::new();
        // add object 1
        let object1 = generate_morphism();

        category.add_object(object1.clone()).unwrap();
        // check identity morphism
        let cell = category.get_object_morphisms(&object1);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object(), object1.source_object());
        assert_eq!(cell.target_object(), object1.target_object());

        // check identity morphism
        let cell = category.get_object_morphisms(&object1);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object(), object1.source_object());
        assert_eq!(cell.target_object(), object1.target_object());

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // add object 2
        let object2 = generate_morphism();
        assert!(category.add_object(object2.clone()).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&object2);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object(), object2.source_object());
        assert_eq!(cell.target_object(), object2.target_object());

        // add object 3 without id
        let object3 = generate_morphism();
        assert!(category.add_object(object3.clone()).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&object3);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object(), object3.source_object());
        assert_eq!(cell.target_object(), object3.target_object());
    }
}
