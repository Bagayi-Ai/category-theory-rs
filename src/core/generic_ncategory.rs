use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::identifier::Identifier;
use crate::core::ncell::NCell;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericNCell<Id: Identifier, Category: NCategory<Identifier = Id>>
{
    id: Id,
    from: Id,
    to: Id,
    name: String,
    _phantom: std::marker::PhantomData<Category>,
}

impl <Id: Identifier, Category: NCategory<Identifier = Id>> GenericNCell<Id, Category>
{
    pub fn new(id: Id, from: Id, to: Id, name: String) -> Self {
        GenericNCell {
            id,
            from,
            to,
            name,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl <Id, Category: NCategory> NCell for GenericNCell<Id, Category>
where
    Id: Identifier,
    Category: NCategory<Identifier = Id>,
{
    type Category = Category;
    type BaseCell = GenericNCell<Id, <Category as NCategory>::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_category(&self) -> &Self::Category {
        todo!()
    }

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        &self.from
    }

    fn target_category(&self) -> &Self::Category {
        todo!()
    }

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        &self.to
    }

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn base_cell_id(&self) -> &<<Self::BaseCell as NCell>::Category as NCategory>::Identifier {
        todo!()
    }

    fn base_cell(&self) -> Self::BaseCell {
        todo!()
    }
}


pub struct GenericNCategory<Id: Identifier, BaseCategory: NCategory<Identifier = Id>>
{
    objects: HashMap<Id, BaseCategory>,
    object_mapping: HashMap<Id, HashMap<Id, HashSet<Id>>>,
    cells: HashMap<Id, GenericNCell<Id, Self>>,
}


impl <Id: Identifier, Category: NCategory<Identifier = Id>> GenericNCategory<Id, Category>
{
    pub fn new() -> Self {
        GenericNCategory {
            objects: HashMap::new(),
            object_mapping: HashMap::new(),
            cells: HashMap::new(),
        }
    }
}


impl <Id: Identifier, BaseCategory: NCategory<Identifier = Id>> NCategory for GenericNCategory<Id, BaseCategory>{
    type Identifier = Id;
    type Object = BaseCategory;
    type Cell = GenericNCell<Id, Self>;
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
        let identity_cell: GenericNCell<Self::Identifier, Self> = GenericNCell::new(
            object_id.clone(),
            object_id.clone(),
            object_id.clone(),
            "identity".to_string(),
        );
        self.add_cell(identity_cell)?;
        Ok(())
    }

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::Identifier, NCategoryError> {
        if self.cells.contains_key(&cell.id) {
            return Err(NCategoryError::CellAlreadyExists);
        }
        self.object_mapping
            .entry(cell.from.clone())
            .or_default()
            .entry(cell.to.clone())
            .or_default()
            .insert(cell.id.clone());
        let cell_id = cell.id.clone();
        self.cells.insert(cell.id.clone(), cell);
        Ok(cell_id)
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
        if let Some(cells) = self.object_mapping.get(object_id) {
            let mut cell_ids: Vec<&Self::Identifier> = Vec::new();
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
