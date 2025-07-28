use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::identifier::Identifier;
use crate::core::generic_ncell::GenericNCell;
use crate::core::ncell::NCell;

#[derive(Debug, Clone)]
pub struct GenericNCategory<'a, Id: Identifier, BaseCategory: NCategory<'a, Identifier = Id>>
{
    objects: HashMap<Id, BaseCategory>,
    object_mapping: HashMap<&'a Id, HashMap<&'a Id, HashSet<&'a Id>>>,
    cells: HashMap<Id, GenericNCell<'a, Id, Self>>,
}


impl <'a, Id: Identifier, Category: NCategory<'a, Identifier = Id>> GenericNCategory<'a, Id, Category>
{
    pub fn new() -> Self {
        GenericNCategory {
            objects: HashMap::new(),
            object_mapping: HashMap::new(),
            cells: HashMap::new(),
        }
    }
}


impl <'a, Id: Identifier, BaseCategory: NCategory<'a, Identifier = Id>> NCategory<'a> for GenericNCategory<'a, Id, BaseCategory>{
    type Identifier = Id;
    type Object = BaseCategory;
    type Cell = GenericNCell<'a, Id, Self>;
    type BaseCategory = BaseCategory;

    fn id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&'a mut self, object: Self::Object) -> Result<&Self::Identifier, NCategoryError> {
        let object_id: Self::Identifier = Identifier::generate();
        self.add_object_with_id(object_id.clone(), object)
    }

    fn add_object_with_id(&'a mut self, object_id: Self::Identifier, object: Self::Object) -> Result<&Self::Identifier, NCategoryError> {
        self.objects.insert(object_id.clone(), object);
        // Retrieve the object reference and extract necessary data
        let (id_ref, obj_ref): (_, &Self::Object) = {
            let object_ref = self.objects.get(&object_id).ok_or(NCategoryError::ObjectNotFound)?;
            (object_ref.id(), object_ref)
        };
        let identity_cell: GenericNCell<'a, Self::Identifier, Self> = GenericNCell::new(
            id_ref,
            obj_ref,
            obj_ref,
            "identity".to_string(),
        );
        self.cells.insert(identity_cell.id().clone(), identity_cell);
        self.object_mapping
            .entry(id_ref)
            .or_default()
            .entry(id_ref)
            .or_default()
            .insert(id_ref);
        Ok(id_ref)
    }

    fn add_cell(&'a mut self, cell: Self::Cell) -> Result<&Self::Identifier, NCategoryError> {
        if self.cells.contains_key(&cell.id()) {
            return Err(NCategoryError::CellAlreadyExists);
        }
        let cell_id = cell.id().clone();
        self.cells.insert(cell.id().clone(), cell);
        // get the cell
        let cell = self.cells.get(&cell_id).ok_or(NCategoryError::CellNotFound)?;
        self.object_mapping
            .entry(cell.source().id())
            .or_default()
            .entry(cell.target().id())
            .or_default()
            .insert(cell.id());
        Ok(cell.id())
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<&Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(object_id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_identity_cell(&self, object_id: &Self::Identifier) -> Result<&Self::Identifier, NCategoryError> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        todo!()
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        Ok(self.cells.keys().collect())
    }

    fn get_object_cells(&self, object_id: &Self::Identifier) -> Result<Vec<&Self::Identifier>, NCategoryError> {
        if let Some(mapping_taget) = self.object_mapping.get(object_id) {
            let mut cell_ids: Vec<&Self::Identifier> = Vec::new();
            for (_to, cell_set) in mapping_taget {
                for cell_id in cell_set {
                    if let Ok(cell) = self.get_cell(cell_id) {
                        if &cell.source().id() == &object_id {
                            cell_ids.push(&cell.id());
                        }
                    }
                    else {
                        return Err(NCategoryError::InvalidCategory);
                    }
                }
            }
            Ok(cell_ids)
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
