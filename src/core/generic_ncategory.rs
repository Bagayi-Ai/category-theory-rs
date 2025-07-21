use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::generic_id::GenericObjectIdTrait;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cell<CellId, ObjectId> {
    id: CellId,
    from: ObjectId,
    to: ObjectId,
    name: String,
}


struct GenericNCategory<ObjectId: GenericObjectIdTrait, BaseCategory: NCategory> {
    objects: HashMap<ObjectId, BaseCategory>,
    object_mapping: HashMap<ObjectId, HashMap<ObjectId, HashSet<ObjectId>>>,
    cells: HashMap<ObjectId, Cell<ObjectId, ObjectId>>,
}

impl<ObjectId: GenericObjectIdTrait, BaseCategory: NCategory> NCategory for GenericNCategory<ObjectId, BaseCategory>
{
    type Object = BaseCategory;
    type ObjectId = ObjectId;
    type CellId = ObjectId;
    type Cell = Cell<ObjectId, ObjectId>;

    type BaseCategory = BaseCategory;

    fn source(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError> {
        if let Some(cell) = self.cells.get(cell_id) {
            Ok(&cell.from)
        } else {
            Err(NCategoryError::CellNotFound)
        }
    }

    fn target(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError> {
        if let Some(cell) = self.cells.get(cell_id) {
            Ok(&cell.to)
        } else {
            Err(NCategoryError::CellNotFound)
        }
    }

    fn add_object(&mut self, object: Self::Object) -> Result<Self::ObjectId, NCategoryError> {
        let object_id = ObjectId::new();
        self.add_object_with_id(object_id.clone(), object).unwrap();
        Ok(object_id)
    }

    fn add_object_with_id(&mut self, object_id: Self::ObjectId, object: Self::Object) -> Result<(), NCategoryError> {
        self.objects.insert(object_id.clone(), object);
        let identity_cell = Cell {
            id: object_id.clone(),
            from: object_id.clone(),
            to: object_id.clone(),
            name: "identity".to_string(),
        };
        self.add_cell(identity_cell).unwrap();
        Ok(())
    }

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::CellId, NCategoryError>{
        if self.cells.contains_key(&cell.id) {
            return Err(NCategoryError::CellAlreadyExists);
        }
        self.cells.insert(cell.id.clone(), cell.clone());
        self.object_mapping
            .entry(cell.from.clone())
            .or_default()
            .entry(cell.to.clone())
            .or_default()
            .insert(cell.id.clone());
        Ok(cell.id)
    }

    fn get_object(&self, id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_object_cells(&self, object_id: &Self::ObjectId) -> Result<Vec<&Self::CellId>, NCategoryError> {
        if let Some(cells) = self.object_mapping.get(object_id) {
            let mut cell_ids: Vec<&Self::CellId> = Vec::new();
            for (_to, cell_set) in cells {
                for cell_id in cell_set {
                    if let Some(cell) = self.cells.get(cell_id) {
                        if &cell.from == object_id {
                            cell_ids.push(&cell.id);
                        }
                    }
                }
            }
            Ok(cell_ids)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_cell(&self, _id: &Self::CellId) -> Result<&Self::Cell, NCategoryError> {
        todo!()
    }


    fn commute(_left: &Self::CellId, _right: &Self::CellId) -> Result<bool, NCategoryError> {
        // Placeholder implementation
        unimplemented!()
    }

    fn base_object(&self, _object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;
    use crate::core::category0::Category0;
    use crate::core::uuid_id::UuidCategoryObjectId;

    type Category0String = Category0<String>;

    struct GenericCategory1TestHelper {
        category: GenericNCategory<UuidCategoryObjectId, Category0String>,
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
        type Category = GenericNCategory<UuidCategoryObjectId, Category0String>;

        fn get_category(&self) -> &Self::Category {
            &self.category
        }

        fn get_mut_category(&mut self) -> &mut Self::Category {
            &mut self.category
        }

        fn generate_object_id(&self) -> <Self::Category as NCategory>::ObjectId {
            UuidCategoryObjectId::new()
        }

        fn generate_cell_id(&self) -> <Self::Category as NCategory>::CellId {
            UuidCategoryObjectId::new()
        }

        fn generate_cell(&mut self) -> <Self::Category as NCategory>::CellId {
            let object1 = self.generate_object();
            let object2 = self.generate_object();
            let object1_id = self.get_mut_category().add_object(object1).unwrap();
            let object2_id = self.get_mut_category().add_object(object2).unwrap();
            let cell_id = self.generate_cell_id();
            let cell = Cell {
                id: cell_id.clone(),
                from: object1_id,
                to: object2_id,
                name: "test_cell".to_string(),
            };
            self.get_mut_category().add_cell(cell).unwrap();
            cell_id
        }

        fn generate_commuting_cell(&self) -> (<Self::Category as NCategory>::CellId, <Self::Category as NCategory>::CellId) {
            todo!()
        }

        fn generate_object(&mut self) -> <Self::Category as NCategory>::Object {
            let random_string = random_string(5);
            let mut object = Category0::new();
            object.add_object(random_string).unwrap();
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