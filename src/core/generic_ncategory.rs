use std::hash::Hash;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

use crate::core::ncategory::{NCategory, NCategoryError, CellTrait, RecursiveCellMap};
use crate::core::generic_id::GenericObjectIdTrait;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cell<CellId, ObjectId, BaseCategory: NCategory>
where
    BaseCategory: NCategory<CellId = ObjectId>
{
    id: CellId,
    from: ObjectId,
    to: ObjectId,
    name: String,
    _phantom: std::marker::PhantomData<BaseCategory>,
}

impl <CellId, ObjectId, BaseCategory: NCategory> CellTrait for Cell<CellId, ObjectId, BaseCategory>
where
    BaseCategory: NCategory<CellId = ObjectId>
{
    type BaseCategory = BaseCategory;
    type CurrentCategoryCellId = ObjectId;

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


#[derive(Clone)]
struct GenericNCategory<ObjectId: GenericObjectIdTrait, BaseCategory: NCategory>
where
    BaseCategory: NCategory<CellId = ObjectId>
{
    objects: HashMap<ObjectId, BaseCategory>,
    object_mapping: HashMap<ObjectId, HashMap<ObjectId, HashSet<ObjectId>>>,
    cells: HashMap<ObjectId, Cell<ObjectId, ObjectId, BaseCategory>>,
}

impl<ObjectId: GenericObjectIdTrait, BaseCategory: NCategory<BaseCategory = BaseCategory>> NCategory for GenericNCategory<ObjectId, BaseCategory>
where
    BaseCategory: NCategory<CellId = ObjectId>,
{
    type Object = BaseCategory;
    type ObjectId = ObjectId;
    type CellId = ObjectId;
    type Cell = Cell<ObjectId, ObjectId, BaseCategory>;

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

    fn apply_cells_recursive(
        &self,
        cell_id: &Self::CellId,
        cell_id_to_map: &Self::CellId
    ) -> Result<
        RecursiveCellMap<Self::CellId>,
        NCategoryError
    > {

        if self.is_zero_category() {
            return Ok(RecursiveCellMap::Leaf);
        }

        let mut object_map: HashMap<
            Self::CellId,
            (Self::CellId, Option<Vec<RecursiveCellMap<Self::CellId>>>)> = HashMap::new();


        let source_object_id = self.source(cell_id)?;

        // we traverse both source and target object all the way to 0-category
        // then start building up recursively the mapping between the objects from 0-category
        // all the way to the current category

        let source_base_category = self.base_object(source_object_id)?;
        let source_objects = source_base_category.get_all_objects()?;

        let cell = self.get_cell(cell_id)?;
        let base_cell = cell.base_cell_id()?;


        let mut cell_maps = Vec::new();
        for object_id in &source_objects {
            let cell_ids_to_map = source_base_category.get_object_cells(object_id)?;

            for cell_id_to_map in cell_ids_to_map {
                let cell_map = source_base_category.apply_cells_recursive(&base_cell, cell_id_to_map)?;
                cell_maps.push(cell_map);
            }
        }

        // map the cell id to target id
        let mapped_cell_id = cell.apply_cell_id(cell_id_to_map)?;
        // confirm the cell exists
        self.get_cell(&mapped_cell_id)?;
        object_map.insert(cell_id.clone(), (mapped_cell_id.clone(), Some(cell_maps)));

        Ok(RecursiveCellMap::Node(object_map))
    }

    fn add_object_with_id(&mut self, object_id: Self::ObjectId, object: Self::Object) -> Result<(), NCategoryError> {
        self.objects.insert(object_id.clone(), object);
        let identity_cell = Cell {
            id: object_id.clone(),
            from: object_id.clone(),
            to: object_id.clone(),
            name: "identity".to_string(),
            _phantom: std::marker::PhantomData,
        };
        self.add_cell(identity_cell).unwrap();
        Ok(())
    }

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::CellId, NCategoryError>{
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

    fn get_object(&self, id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_identity_cell(&self, _object_id: &Self::ObjectId) -> Result<&Self::CellId, NCategoryError> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::ObjectId>, NCategoryError> {
        Ok(self.objects.keys().collect())
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::CellId>, NCategoryError> {
        Ok(self.cells.keys().collect())
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

    fn base_object(&self, _object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}
