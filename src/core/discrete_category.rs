use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use crate::core::ncategory::{NCategory, NCategoryError, CellTrait, RecursiveCellMap};

pub struct DiscreteCategory<T> {
    objects: HashSet<T>
}

impl<T: Eq + Clone + Hash> DiscreteCategory<T> {
    pub fn new() -> Self {
        DiscreteCategory {
            objects: HashSet::new(),
        }
    }
}

pub struct DiscreteCategoryCell<T> {
    _object_id: T,
}

impl<T: Eq + Hash + Clone + Debug> CellTrait for DiscreteCategoryCell<T> {
    type BaseCategory = ();
    type CurrentCategoryCellId = T;

    fn id(&self) -> &Self::CurrentCategoryCellId {
        todo!()
    }

    fn base_cell_id(&self) -> Result<<Self::BaseCategory as NCategory>::CellId, NCategoryError> {
        todo!()
    }

    fn apply_cell_id(&self, _cell_id: &<Self::BaseCategory as NCategory>::CellId) -> Result<<Self::BaseCategory as NCategory>::CellId, NCategoryError> {
        todo!()
    }

    fn apply_to_object(&self, _object_id: &<Self::BaseCategory as NCategory>::ObjectId) -> Result<<Self::BaseCategory as NCategory>::ObjectId, NCategoryError> {
        todo!()
    }
}


impl<T: Eq + Clone + Hash + Debug> NCategory for DiscreteCategory<T> {
    type Object = T;
    type ObjectId = T;
    type CellId = T;
    type Cell = DiscreteCategoryCell<T>;
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
        panic!("No cells in DiscreteCategory")
    }

    fn is_zero_category(&self) -> bool {
        true
    }

    fn get_object(&self, id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::ObjectId>, NCategoryError> {
        Ok(self.objects.iter().collect())
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::CellId>, NCategoryError> {
        // In DiscreteCategory, there are no cells, so we return an empty set.
        Ok(HashSet::new())
    }

    fn get_object_cells(&self, object_id: &Self::ObjectId) -> Result<Vec<&Self::CellId>, NCategoryError> {
        // only cell in 0-category is the identity cell.
        if let Some(object) = self.objects.get(object_id) {
            Ok(vec![object])
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_cell(&self, _cell_id: &Self::CellId) -> Result<&Self::Cell, NCategoryError> {
        todo!()
    }

    fn commute(&self, _left: Vec<&Self::CellId>, _right: Vec<&Self::CellId>) -> Result<bool, NCategoryError> {
        Ok(false)
    }

    fn base_object(&self, _object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        Ok(&())
    }

    fn get_identity_cell(&self, _object_id: &Self::ObjectId) -> Result<&Self::CellId, NCategoryError> {
        todo!()
    }

    fn apply_cells_recursive(&self, _cell_id: &Self::CellId, _cell_id_to_map: &Self::CellId) -> Result<RecursiveCellMap<Self::CellId>, NCategoryError> {
        todo!()
    }
}

impl From<String> for DiscreteCategory<String> {
    fn from(object: String) -> Self {
        let mut category = DiscreteCategory::new();
        category.add_object(object).unwrap();
        category
    }
}

impl <T: Eq + Clone + Hash + Debug> From<Vec<T>> for DiscreteCategory<T>
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

    struct GenericCategory0TestHelper {
        category: DiscreteCategory<String>,
    }

    impl GenericCategory0TestHelper {
        pub fn new() -> Self {
            GenericCategory0TestHelper {
                category: DiscreteCategory::new(),
            }
        }
    }

    impl NCategoryTestHelper for GenericCategory0TestHelper {
        type Category = DiscreteCategory<String>;

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

        fn generate_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory>::CellId>, Vec<<Self::Category as NCategory>::CellId>) {
            todo!()
        }

        fn generate_non_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory>::CellId>, Vec<<Self::Category as NCategory>::CellId>) {
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
