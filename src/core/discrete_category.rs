use crate::core::arrow::{Arrow, Functor, Morphism};
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiscreteCategory {
    category_id: ObjectId,
    // TODO: Find a way of avoiding storing identity cells
    // as we could derive them from the objects.
    cells: Option<HashMap<ObjectId, Rc<Morphism<Self>>>>,
}

impl Hash for DiscreteCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category_id.hash(state);
        if let Some(cells) = &self.cells {
            for key in cells.keys() {
                key.hash(state);
            }
        }
    }
}

impl Default for DiscreteCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscreteCategory {
    pub fn new() -> Self {
        DiscreteCategory {
            category_id: ObjectId::Str(String::generate()),
            cells: Some(HashMap::new()),
        }
    }

    pub fn new_with_id(category_id: ObjectId) -> Self {
        DiscreteCategory {
            category_id,
            cells: None,
        }
    }

    pub fn clone_with_new_id(&self) -> Self {
        Self {
            category_id: ObjectId::Str(String::generate()),
            cells: self.cells.clone(),
        }
    }
    pub fn category_id(&self) -> &ObjectId {
        &self.category_id
    }
}

impl CategoryTrait for DiscreteCategory {
    type Object = Self;

    type Morphism = Morphism<Self::Object>;

    fn new() -> Self {
        DiscreteCategory::new()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        DiscreteCategory::new_with_id(id.clone())
    }

    fn category_id(&self) -> &ObjectId {
        &self.category_id
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        let identity_morphism = Morphism::new_identity(object.clone());
        if let Some(cells) = &mut self.cells {
            if cells.contains_key(&object.category_id()) {
                return Err(Errors::ObjectAlreadyExists);
            }
            cells.insert(object.category_id().clone(), identity_morphism);
            Ok(())
        } else {
            self.cells = Some(HashMap::new());
            self.cells
                .as_mut()
                .unwrap()
                .insert(object.category_id().clone(), identity_morphism);
            Ok(())
        }
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Morphism<Self::Object>>,
    ) -> Result<&Rc<Morphism<Self::Object>>, Errors> {
        Err(Errors::CannotAddMorphismToDiscreteCategory)
    }

    fn get_identity_morphism(
        &self,
        object: &Self::Object,
    ) -> Result<&Rc<Morphism<Self::Object>>, Errors> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(&object.category_id()) {
                return Ok(cell);
            }
        }
        Err(Errors::ObjectNotFound)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        // if let Some(cells) = &self.cells {
        //     if let Some(cell) = cells.get(&object.category_id()) {
        //         return Ok(cell.source_object());
        //     }
        // }
        // Err(Errors::ObjectNotFound)
        todo!()
    }

    // fn get_all_objects(&self) -> Result<HashSet<&Rc<DiscreteCategory>>, Errors> {
    //     let result = if let Some(cells) = &self.cells {
    //         cells.values().map(|item| item.source_object()).collect()
    //     } else {
    //         HashSet::new()
    //     };
    //     Ok(result)
    // }
    // Rust
    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        // let result = if let Some(cells) = &self.cells {
        //     cells.values().map(|item| item.source_object()).collect()
        // } else {
        //     Vec::new()
        // };
        // Ok(result)
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        let result = if let Some(cells) = &self.cells {
            cells.values().collect()
        } else {
            HashSet::new()
        };
        Ok(result)
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        // only one morphism in discrete category, the identity morphism.
        Ok(HashSet::from([self.get_identity_morphism(source_object)?]))
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Morphism<Self::Object>>>, Errors> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_morphism(object)?])
    }

    fn nested_level() -> usize {
        1
    }
}

impl From<ObjectId> for DiscreteCategory {
    fn from(object: ObjectId) -> Self {
        DiscreteCategory::new_with_id(object)
    }
}

impl From<Vec<ObjectId>> for DiscreteCategory {
    fn from(objects: Vec<ObjectId>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            let object = DiscreteCategory::new_with_id(object);
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}

impl From<i32> for DiscreteCategory {
    fn from(object: i32) -> Self {
        DiscreteCategory::new_with_id(ObjectId::Int(object))
    }
}

impl From<Vec<i32>> for DiscreteCategory {
    fn from(objects: Vec<i32>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            let object = DiscreteCategory::new_with_id(object.into());
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}
impl From<&str> for DiscreteCategory {
    fn from(object: &str) -> Self {
        DiscreteCategory::new_with_id(object.into())
    }
}

impl From<Vec<&str>> for DiscreteCategory {
    fn from(objects: Vec<&str>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            let object = DiscreteCategory::new_with_id(object.into());
            category.add_object(Rc::new(object)).unwrap();
        }
        category
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;

    fn generate_morphism() -> Rc<DiscreteCategory> {
        Rc::new(DiscreteCategory::new_with_id(ObjectId::Str(random_string(
            5,
        ))))
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
        let cell = category.get_object_morphisms(&*object1);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert!(cell.source_object().equal_to(&*object1));
        assert!(cell.target_object().equal_to(&*object1));

        // check identity morphism
        let cell = category.get_object_morphisms(&*object1);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert!(cell.source_object().equal_to(&*object1));
        assert!(cell.target_object().equal_to(&*object1));

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // add object 2
        let object2 = generate_morphism();
        assert!(category.add_object(object2.clone()).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&*object2);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert!(cell.source_object().equal_to(&*object2));
        assert!(cell.target_object().equal_to(&*object2));

        // add object 3 without id
        let object3 = generate_morphism();
        assert!(category.add_object(object3.clone()).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&*object3);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert!(cell.source_object().equal_to(&*object3));
        assert!(cell.target_object().equal_to(&*object3));
    }
}
