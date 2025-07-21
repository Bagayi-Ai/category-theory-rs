use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use crate::core::ncategory::{NCategory};
use crate::core::generic_id::GenericObjectIdTrait;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cell<ObjectId> {
    from: ObjectId,
    to: ObjectId,
    name: String,
}


struct GenericNCategory<ObjectId: GenericObjectIdTrait, BaseCategory: NCategory> {
    objects: HashMap<ObjectId, BaseCategory>,
    object_mapping: HashMap<ObjectId, HashSet<ObjectId>>,
    cells: HashMap<ObjectId, Cell<ObjectId>>,
}

impl<ObjectId: GenericObjectIdTrait, BaseCategory: NCategory> NCategory for GenericNCategory<ObjectId, BaseCategory>
{
    type Object = BaseCategory;
    type ObjectId = ObjectId;
    type CellId = ObjectId;
    type Cell = Cell<ObjectId>;

    type BaseCategory = BaseCategory;

    fn source(&self, cell_id: &Self::CellId) -> &Self::ObjectId {
        if let Some(cell) = self.cells.get(cell_id) {
            &cell.from
        } else {
            panic!("Cell not found")
        }
    }

    fn target(&self, cell_id: &Self::CellId) -> &Self::ObjectId {
        if let Some(cell) = self.cells.get(cell_id) {
            &cell.to
        } else {
            panic!("Cell not found")
        }
    }

    fn add_object(&mut self, object: Self::Object) -> Self::ObjectId {
        let object_id = ObjectId::new();
        self.add_object_with_id(object_id.clone(), object);
        object_id
    }

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object) {
        self.objects.insert(objectId.clone(), object);
        let identity_cell = Cell {
            from: objectId.clone(),
            to: objectId.clone(),
            name: "identity".to_string(),
        };
        self.cells.insert(objectId.clone(), identity_cell);
        self.object_mapping.insert(objectId.clone(), vec![objectId.clone()].into_iter().collect());
    }

    fn add_cell(&mut self, m: Self::Cell) {
        todo!()
    }

    fn get_object(&self, id: &Self::ObjectId) -> Option<&Self::Object> {
        self.objects.get(id)
    }

    fn get_object_cells(&self, objectId: &Self::ObjectId) -> Vec<&Self::CellId> {
        if let Some(cells) = self.object_mapping.get(objectId) {
            cells.iter().collect()
        } else {
            vec![]
        }
    }

    fn get_cell(&self, id: &Self::CellId) -> Option<&Self::Cell> {
        todo!()
    }


    fn commute(left: &Self::CellId, right: &Self::CellId) -> bool {
        // Placeholder implementation
        unimplemented!()
    }

    fn base_object(&self, object_id: &Self::ObjectId) -> &Self::BaseCategory {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;
    use crate::core::category0::Category0;
    use crate::core::uuid_id::UuidCategoryObjectId;

    struct GenericCategory1TestHelper {
        category: GenericNCategory<UuidCategoryObjectId, Category0<Cell<String>>>,
    }

    impl GenericCategory1TestHelper {
        pub fn new() -> Self {
            GenericCategory1TestHelper {
                category: GenericNCategory {
                    objects: HashMap::new(),
                    object_mapping: HashMap::new(),
                    cells: HashMap::new(),
                },
            }
        }
    }

    impl NCategoryTestHelper for GenericCategory1TestHelper {
        type category = GenericNCategory<UuidCategoryObjectId, Category0<Cell<String>>>;

        fn get_category(&self) -> &Self::category {
            &self.category
        }

        fn get_mut_category(&mut self) -> &mut Self::category {
            &mut self.category
        }

        fn generate_object_id(&self) -> <Self::category as NCategory>::ObjectId {
            UuidCategoryObjectId::new()
        }

        fn generate_cell_id(&self) -> <Self::category as NCategory>::CellId {
            UuidCategoryObjectId::new()
        }

        fn generate_object(&mut self) -> <Self::category as NCategory>::Object {
            let random_string = random_string(5);
            let random_cell = Cell {
                from: random_string.clone(),
                to: random_string.clone(),
                name: random_string.clone(),
            };
            let mut object = Category0::new();
            object.add_object(random_cell);
            object
        }

        fn expected_category_level(&self) -> isize {
            1
        }

    }
    #[test]
    pub fn test_base_scenarios() {
        let category_test_helper = GenericCategory1TestHelper::new();
        basic_object_cell_test(category_test_helper);
    }
}