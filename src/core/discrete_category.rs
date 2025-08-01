use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, NCategoryError, UnitCategory};
use crate::core::morphism::Morphism;
use crate::core::nfunctor::{NFunctor, UnitFunctor};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiscreteCategory<T: Hash + Eq + Clone> {
    category_id: T,
    // TODO: Find a way of avoiding storing identity cells
    // as we could derive them from the objects.
    cells: Option<HashMap<T ,Self>>

}

impl<T: Hash + Eq + Clone> Hash for DiscreteCategory<T> {
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
        DiscreteCategory {
            category_id: T::generate(),
            cells: Some(HashMap::new()),
        }
    }

    pub fn new_with_id(category_id: T) -> Self {
        DiscreteCategory {
            category_id,
            cells: None
        }
    }
}

impl<'a, T: Eq + Clone + Hash + Debug + Identifier<Id = T> + 'a> NCategory<'a> for DiscreteCategory<T>
{
    type Identifier = T;
    type Object = T;
    type Morphism = Self;
    type BaseCategory = UnitCategory<T>;

    fn category_id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn add_object(&mut self, object: Self::Object) -> Result<(), NCategoryError> {
        let cell = Self::new_with_id(object.clone());
        if let Some(cells) = &mut self.cells {
            cells.insert(object.clone(), cell);
        } else {
            self.cells = Some(HashMap::new());
            self.cells.as_mut().unwrap().insert(object.clone(), cell);
        }
        Ok(())
    }

    fn add_moprhism(&mut self, _cell: Self::Morphism) -> Result<Self::Identifier, NCategoryError> {
        Err(NCategoryError::OnlyIdentityCellDiscreteCategory)
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError> {
        if let Some(cells) = &self.cells {
            if let Some((key, _value)) = cells.get_key_value(object_id) {
                return Ok(key.clone());
            }
        }
        Err(NCategoryError::ObjectNotFound)
    }

    fn get_identity_morphism(&self, object_id: Self::Object) -> Result<&Self::Morphism, NCategoryError> {
        self.get_cell(&object_id)
    }

    fn get_all_objects(&self) -> Result<HashSet<Self::Identifier>, NCategoryError> {
        if let Some(cells) = &self.cells {
            Ok(cells.keys().map(|item| item.clone()).collect())
        } else {
            Err(NCategoryError::NoObjectsInCategory)
        }
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError> {
        self.get_all_objects()?.into_iter().map(
            |object_id| self.get_identity_morphism(object_id)).collect()
    }

    fn get_object_morphisms(&self, object_id: Self::Object) -> Result<Vec<&Self::Morphism>, NCategoryError> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_morphism(object_id)?])
    }

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(cell_id) {
                return Ok(cell);
            }
        }
        Err(NCategoryError::CellNotFound)
    }

    fn base_object(&self, _object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}

impl<'a, T: Eq + Clone + Hash + Debug + Identifier<Id = T> + 'a> Morphism<'a> for DiscreteCategory<T> {

    type Category = Self;
    // type Functor = UnitFunctor<T>;

    fn cell_id(&self) -> &<Self::Category as NCategory<'a>>::Identifier {
        todo!()
    }

    fn source_object(&self) -> <Self::Category as NCategory<'a>>::Object {
        self.category_id.clone()
    }

    fn target_object(&self) -> <Self::Category as NCategory<'a>>::Object {
        self.category_id.clone()
    }


    // fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier {
    //     todo!()
    // }
}

impl <T: Eq + Clone + Hash + Debug + Identifier<Id = T>> From<T> for DiscreteCategory<T>
{
    fn from(object: T) -> Self {
        let mut category = DiscreteCategory::new();
        category.add_object(object).unwrap();
        category
    }
}

impl <T: Eq + Clone + Hash + Debug + Identifier<Id = T>> From<Vec<T>> for DiscreteCategory<T>
{
    fn from(objects: Vec<T>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            category.add_object(object).unwrap();
        }
        category
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;

    fn generate_object() ->  String {
        random_string(5)
    }

    fn generate_identifier() -> String {
        String::generate()
    }

    #[test]
    pub fn test_base_scenarios() {
        let mut category = DiscreteCategory::new();
        // add object 1
        let object1 = generate_object();

        category.add_object(object1.clone()).unwrap();
        assert!(category.get_object(&object1).is_ok());
        // check identity morphism
        let cell = category.get_object_morphisms(object1.clone());
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object(), object1);
        assert_eq!(cell.target_object(), object1);

        // check identity morphism
        let cell = category.get_object_morphisms(object1.clone());
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object(), object1);
        assert_eq!(cell.target_object(), object1);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // add object 2
        let object2 = generate_object();
        assert!(category.add_object(object2.clone()).is_ok());
        assert!(category.get_object(&object2).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(object2.clone());
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object(), object2);
        assert_eq!(cell.target_object(), object2);

        // add object 3 without id
        let object3 = generate_object();
        assert!(category.add_object(object3.clone()).is_ok());

        // check object 3 exists
        assert!(category.get_object(&object3).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(object3.clone());
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object(), object3);
        assert_eq!(cell.target_object(), object3);
    }
}
