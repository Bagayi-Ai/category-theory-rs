use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use crate::core::ncategory::NCategory;

struct Category0<T> {
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
    type CellId = ();
    type Cell = ();
    type Higher = Self;

    fn source(cell: &Self::Cell) -> &Self::Object {
        panic!("No cells in Category0")
    }

    fn target(_m: &Self::Cell) -> &Self::Object {
        panic!("No cells in Category0")
    }

    fn add_object(&mut self, object: Self::Object) -> &Self::ObjectId {
        self.objects.insert(object.clone());
        self.objects.get(&object).unwrap()
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

    fn get_cell(&self, id: &Self::CellId) -> Option<&Self::Cell> {
        panic!("No cells in Category0")
    }

    fn commute(_left: &Self::Cell, _right: &Self::Cell) -> bool {
        true
    }
}


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
            ()
        }

        fn generate_object(&self) -> <Self::category as NCategory>::Object {
            random_string(5)
        }

    }
    #[test]
    pub fn test_base_scenarios() {
        let mut category_test_helper = GenericCategory0TestHelper::new();
        basic_object_cell_test(category_test_helper);
    }
}