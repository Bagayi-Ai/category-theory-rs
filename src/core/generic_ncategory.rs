use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};
use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::identifier::Identifier;
use crate::core::ncell::NCell;
use crate::core::nfunctor::NFunctor;
use crate::core::generic_ncell::GenericNCell;


pub struct GenericNCategory<'a, Id: Identifier, BaseCategory: NCategory<'a, Identifier = Id>>
{
    id: Id,
    objects: HashMap<Id, &'a BaseCategory>,
    object_mapping: HashMap<Id, HashMap<Id, HashSet<Id>>>,
    cells: HashMap<Id, GenericNCell<Id>>,
}


impl <'a, Id: Identifier, Category: NCategory<'a, Identifier = Id>> GenericNCategory<'a, Id, Category>
{
    pub fn new() -> Self {
        GenericNCategory {
            id: Id::generate(),
            objects: HashMap::new(),
            object_mapping: HashMap::new(),
            cells: HashMap::new(),
        }
    }
}


impl <'a, Id: Identifier<Id = Id>, BaseCategory: NCategory<'a, Identifier = Id> + 'a> NCategory<'a> for GenericNCategory<'a, Id, BaseCategory>{
    type Identifier = Id;
    type Object = &'a BaseCategory;
    type Cell = GenericNCell<Id>;
    type BaseCategory = BaseCategory;

    fn id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Self::Object) -> Result<Self::Identifier, NCategoryError> {
        let object_id: Self::Identifier = Identifier::generate();
        self.add_object_with_id(object_id.clone(), object).unwrap();
        Ok(object_id)
    }

    fn add_object_with_id(&mut self, object_id: Self::Identifier, object: Self::Object) -> Result<(), NCategoryError> {
        self.objects.insert(object_id.clone(), object);
        // let identity_cell = GenericNCell {
        //     id: object_id.clone(),
        //     from: object_id.clone(),
        //     to: object_id.clone(),
        //     name: "identity".to_string(),
        //     _phantom: std::marker::PhantomData,
        // };
        let identity_cell: GenericNCell<Self::Identifier> = GenericNCell::new(
            object_id.clone(),
            object_id.clone(),
            object_id.clone(),
            "identity".to_string(),
        );
        self.add_cell(identity_cell)?;
        Ok(())
    }

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::Identifier, NCategoryError> {
        if self.cells.contains_key(&cell.id()) {
            return Err(NCategoryError::CellAlreadyExists);
        }
        self.object_mapping
            .entry(cell.source_object_id().clone())
            .or_default()
            .entry(cell.target_object_id().clone())
            .or_default()
            .insert(cell.id().clone());
        let cell_id = cell.id().clone();
        self.cells.insert(cell.id().clone(), cell);
        Ok(cell_id)
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(object_id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_identity_cell(&self, object_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError> {
        // it's basically the cell with the same id as the object
        self.get_cell(object_id)
    }

    fn get_all_objects(&self) -> Result<HashSet<Self::Object>, NCategoryError> {
        todo!()
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::Cell>, NCategoryError> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let mut result: HashSet<&Self::Cell> = HashSet::new();
        for (_id, cell) in &self.cells {
            result.insert(cell);
        }
        Ok(result)
    }

    fn get_object_cells(&self, object_id: &Self::Identifier) -> Result<Vec<&Self::Cell>, NCategoryError> {
        if let Some(cells) = self.object_mapping.get(object_id) {
            let mut result: Vec<&Self::Cell> = Vec::new();
            for (_to, cell_set) in cells {
                for cell_id in cell_set {
                    if let Some(cell) = self.cells.get(cell_id) {
                        if cell.source_object_id() == object_id {
                            result.push(&cell);
                        }
                    }
                }
            }
            Ok(result)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError> {
        if let Some(cell) = self.cells.get(cell_id) {
            Ok(cell)
        } else {
            Err(NCategoryError::CellNotFound)
        }
    }

    fn base_object(&self, object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}
