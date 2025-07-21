use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use crate::core::ncategory::NCategory;

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

    fn source(&self, cell_id: &Self::CellId) -> &Self::Object {
        self.get_object(cell_id).unwrap()
    }

    fn target(&self, cell: &Self::CellId) -> &Self::Object {
        self.get_object(cell).unwrap()
    }

    fn add_object(&mut self, object: Self::Object) -> Self::ObjectId {
        self.objects.insert(object.clone());
        self.objects.get(&object).unwrap().clone()
    }

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object) {
        self.objects.insert(objectId.clone());
    }

    fn add_cell(&mut self, m: Self::Cell) {
        panic!("No cells in Category0")
    }

    fn get_object(&self, id: &Self::ObjectId) -> Option<&Self::Object> {
        self.objects.get(id)
    }

    fn get_object_cells(&self, objectId: &Self::ObjectId) -> Vec<&Self::Cell> {
        // only cell in 0-category is the identity cell.
        if self.objects.contains(objectId) {
            vec![self.objects.get(objectId).unwrap()]
        } else {
            vec![]
        }
    }

    fn get_cell(&self, cell_id: &Self::CellId) -> Option<&Self::Cell> {
        if self.objects.contains(cell_id) {
            Some(self.objects.get(cell_id).unwrap())
        } else {
            None
        }
    }

    fn commute(_left: &Self::Cell, _right: &Self::Cell) -> bool {
        false
    }

    fn base_object(&self, object_id: &Self::ObjectId) -> &Self::BaseCategory {
        &()
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
        type category = Category0<String>;

        fn get_category(&self) -> &Self::category {
            &self.category
        }

        fn get_mut_category(&mut self) -> &mut Self::category {
            &mut self.category
        }

        fn generate_object_id(&self) -> <Self::category as NCategory>::ObjectId {
            random_string(5)
        }

        fn generate_cell_id(&self) -> <Self::category as NCategory>::CellId {
            random_string(5)
        }

        fn generate_object(&mut self) -> <Self::category as NCategory>::Object {
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
