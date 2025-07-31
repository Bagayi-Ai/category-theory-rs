use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, NCategoryError, UnitCategory};
use crate::core::ncell::NCell;
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
    type Cell = Self;
    type BaseCategory = UnitCategory<T>;

    fn id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn add_object(&mut self, object: Self::Object) -> Result<Self::Identifier, NCategoryError> {
        let cell = Self::new_with_id(object.clone());
        if let Some(cells) = &mut self.cells {
            cells.insert(object.clone(), cell);
        } else {
            self.cells = Some(HashMap::new());
            self.cells.as_mut().unwrap().insert(object.clone(), cell);
        }
        Ok(object.clone())
    }

    fn add_object_with_id(&mut self, object_id: Self::Identifier, _object: Self::Object) -> Result<(), NCategoryError> {
        self.add_object(object_id.clone())?;
        Ok(())
    }

    fn add_cell(&mut self, _cell: Self::Cell) -> Result<Self::Identifier, NCategoryError> {
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

    fn get_identity_cell(&self, object_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError> {
        self.get_cell(object_id)
    }

    fn get_all_objects(&self) -> Result<HashSet<Self::Identifier>, NCategoryError> {
        if let Some(cells) = &self.cells {
            Ok(cells.keys().map(|item| item.clone()).collect())
        } else {
            Err(NCategoryError::NoObjectsInCategory)
        }
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::Cell>, NCategoryError> {
        self.get_all_objects()?.into_iter().map(
            |object_id| self.get_identity_cell(&object_id)).collect()
    }

    fn get_object_cells(&self, object_id: &Self::Identifier) -> Result<Vec<&Self::Cell>, NCategoryError> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_cell(object_id)?])
    }

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError> {
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

impl<T: Eq + Clone + Hash + Debug + Identifier> NCell for DiscreteCategory<T> {

    type Identifier = T;
    type Functor = UnitFunctor<T>;

    fn id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object_id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn target_object_id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn functor(&self) -> &<Self::Functor as NFunctor>::Identifier {
        todo!()
    }
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
        let object1_id = generate_identifier();
        let object1 = generate_object();
        let object2_id = generate_identifier();

        category.add_object_with_id(object1_id.clone(), object1).unwrap();
        assert!(category.get_object(&object1_id).is_ok());
        // check identity morphism
        let cell = category.get_object_cells(&object1_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object_id(), &object1_id);
        assert_eq!(cell.target_object_id(), &object1_id);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // check object 2 does not exist yet
        assert!(!category.get_object(&object2_id).is_ok());

        // check identity morphism
        let cell = category.get_object_cells(&object1_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object_id(), &object1_id);
        assert_eq!(cell.target_object_id(), &object1_id);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // check object 2 does not exist yet
        assert!(!category.get_object(&object2_id).is_ok());

        // add object 2
        let object2 = generate_object();
        category.add_object_with_id(object2_id.clone(), object2).unwrap();
        assert!(category.get_object(&object2_id).is_ok());

        // check identity morphism
        let cells = category.get_object_cells(&object2_id);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object_id(), &object2_id);
        assert_eq!(cell.target_object_id(), &object2_id);

        // add object 3 without id
        let object3 = generate_object();
        let object3_id = category.add_object(object3);
        assert!(object3_id.is_ok());
        let object3_id = object3_id.unwrap();

        // check object 3 exists
        assert!(category.get_object(&object3_id).is_ok());

        // check identity morphism
        let cells = category.get_object_cells(&object3_id);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object_id(), &object3_id);
        assert_eq!(cell.target_object_id(), &object3_id);
    }
}
