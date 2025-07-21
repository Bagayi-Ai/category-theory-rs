use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use crate::core::ncategory::{NCategory, NCategoryError};

pub struct Category0<T> {
    objects: HashSet<T>
}

impl<T: Eq + Clone + Hash> Category0<T> {
    pub fn new() -> Self {
        Category0 {
            objects: HashSet::new(),
        }
    }
}

impl<T: Eq + Clone + Hash + Debug> NCategory for Category0<T> {
    type Object = T;
    type ObjectId = T;
    type CellId = T;
    type Cell = T;
    type BaseCategory = ();

    fn source(&self, cell_id: &Self::CellId) -> Result<&Self::Object, NCategoryError> {
        if let Ok(object) = self.get_object(cell_id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn target(&self, cell: &Self::CellId) -> Result<&Self::Object, NCategoryError> {
        if let Ok(object) = self.get_object(cell) {
            Ok(object)
        }
        else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn add_object(&mut self, object: Self::Object) -> Result<Self::ObjectId, NCategoryError> {
        self.objects.insert(object.clone());
        Ok(self.objects.get(&object).unwrap().clone())
    }

    fn add_object_with_id(&mut self, object_id: Self::ObjectId, _object: Self::Object) -> Result<(), NCategoryError> {
        self.objects.insert(object_id.clone());
        Ok(())
    }

    fn add_cell(&mut self, _cell: Self::Cell) -> Result<Self::CellId, NCategoryError> {
        panic!("No cells in Category0")
    }

    fn get_object(&self, id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_object_cells(&self, object_id: &Self::ObjectId) -> Result<Vec<&Self::Cell>, NCategoryError> {
        // only cell in 0-category is the identity cell.
        if let Some(object) = self.objects.get(object_id) {
            Ok(vec![object])
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_cell(&self, cell_id: &Self::CellId) -> Result<&Self::Cell, NCategoryError> {
        if let Some(cell) = self.objects.get(cell_id) {
            Ok(cell)
        } else {
            Err(NCategoryError::CellNotFound)
        }
    }

    fn commute(_left: &Self::Cell, _right: &Self::Cell) -> Result<bool, NCategoryError> {
        Ok(false)
    }

    fn base_object(&self, _object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        Ok(&())
    }
}

impl From<String> for Category0<String> {
    fn from(object: String) -> Self {
        let mut category = Category0::new();
        category.add_object(object).unwrap();
        category
    }
}

impl <T: Eq + Clone + Hash + Debug> From<Vec<T>> for Category0<T>
{
    fn from(objects: Vec<T>) -> Self {
        let mut category = Category0::new();
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

    struct GenericCategory0TestHelper {
        category: Category0<String>,
    }

    impl GenericCategory0TestHelper {
        pub fn new() -> Self {
            GenericCategory0TestHelper {
                category: Category0::new(),
            }
        }
    }

    impl NCategoryTestHelper for GenericCategory0TestHelper {
        type Category = Category0<String>;

        fn get_category(&self) -> &Self::Category {
            &self.category
        }

        fn get_mut_category(&mut self) -> &mut Self::Category {
            &mut self.category
        }

        fn generate_object_id(&self) -> <Self::Category as NCategory>::ObjectId {
            random_string(5)
        }

        fn generate_cell_id(&self) -> <Self::Category as NCategory>::CellId {
            random_string(5)
        }

        fn generate_cell(&mut self) -> <Self::Category as NCategory>::CellId {
            todo!()
        }

        fn generate_commuting_cell(&self) -> (<Self::Category as NCategory>::CellId, <Self::Category as NCategory>::CellId) {
            todo!()
        }

        fn generate_object(&mut self) -> <Self::Category as NCategory>::Object {
            random_string(5)
        }

        fn expected_category_level(&self) -> isize {
            0
        }

    }
    #[test]
    pub fn test_base_scenarios() {
        let category_test_helper = GenericCategory0TestHelper::new();
        basic_object_cell_test(category_test_helper);
    }
}
