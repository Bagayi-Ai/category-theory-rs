use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NCategoryError {
    CellAlreadyExists,
    CellNotFound,
    InvalidCellComposition,
    InvalidCellCommutation,
    ObjectNotFound,
    InvalidObjectMapping,
    InvalidCellMapping,
}

pub enum RecursiveCellMap<CellId> {
    Leaf,
    Node(
        HashMap<
            CellId, // source cell id
            (
                CellId, // target cell id
                // children maps strating all over from the target object of the cell
                Option<Vec<RecursiveCellMap<CellId>>>,
            )
        >
    ),
}

pub trait CellTrait {
    type BaseCategory: NCategory;

    type CurrentCategoryCellId;

    fn id(&self) -> &Self::CurrentCategoryCellId;

    fn base_cell_id(&self)
        -> Result<<Self::BaseCategory as NCategory>::CellId, NCategoryError>;

    fn apply_cell_id(&self, cell_id: &<Self::BaseCategory as NCategory>::CellId)
        -> Result<<Self::BaseCategory as NCategory>::CellId, NCategoryError>;

    fn apply_to_object(&self, object_id: &<Self::BaseCategory as NCategory>::ObjectId)
        -> Result<<Self::BaseCategory as NCategory>::ObjectId, NCategoryError>;
}

pub trait NCategory {
    type Object;
    type ObjectId: Eq + Hash + Clone + Debug;
    type CellId: Eq + Hash + Clone;
    type Cell: CellTrait<CurrentCategoryCellId = Self::CellId, BaseCategory = Self::BaseCategory>;
    type BaseCategory: NCategory;

    fn source(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError>;

    fn target(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError>;

    fn add_object(&mut self, object: Self::Object) -> Result<Self::ObjectId, NCategoryError>;

    fn add_object_with_id(&mut self, object_id: Self::ObjectId, object: Self::Object) -> Result<(), NCategoryError>;

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::CellId, NCategoryError>;

    fn get_object(&self, object_id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError>;

    fn get_identity_cell(
        &self,
        object_id: &Self::ObjectId,
    ) -> Result<&Self::CellId, NCategoryError>;

    fn get_all_objects(&self) -> Result<HashSet<&Self::ObjectId>, NCategoryError>;

    fn get_all_cells(&self) -> Result<HashSet<&Self::CellId>, NCategoryError>;

    fn get_object_cells(
        &self,
        object_id: &Self::ObjectId,
    ) -> Result<Vec<&Self::CellId>, NCategoryError>;

    fn get_object_targets(
        &self,
        object_id: &Self::ObjectId,
    ) -> Result<Vec<&Self::ObjectId>, NCategoryError> {
        self.get_object_cells(object_id)
            .unwrap()
            .iter()
            .map(|cell_id| self.target(cell_id))
            .collect()
    }

    fn get_cell(&self, cell_id: &Self::CellId) -> Result<&Self::Cell, NCategoryError>;

    fn is_zero_category(&self) -> bool {
        false
    }

    fn apply_cells_recursive(
        &self,
        cell_id: &Self::CellId,
        cell_id_to_map: &Self::CellId
    ) -> Result<
        RecursiveCellMap<Self::CellId>,
        NCategoryError
    >;

    fn commute(&self, left_cell_id: Vec<&Self::CellId>, right_cell_id: Vec<&Self::CellId>) -> Result<bool, NCategoryError> {
        let (left_cells, _right_cells) = self.validate_cell_commutation(left_cell_id, right_cell_id)?;

        // now we apply the cell to the objects in the base category
        // start with applying the left cells
        let source_object = self.source(&left_cells[0].id())?;
        let source_object_base_category = self.base_object(source_object)?;
        let source_base_objects = source_object_base_category.get_all_objects()?;
        let mut result_object_mapping = HashMap::new();
        let mut result_object_cells_mapping = HashMap::new();

        for object in source_base_objects {
            // add object to the resulting mapping if its not already there
            for cell in &left_cells {
                // apply the cell to the object
                let mapped_object = cell.apply_to_object(object)?;

                // check if the result is in the target object
                let target_object = self.target(&cell.id())?;
                let target_object_base_category = self.base_object(target_object)?;
                if target_object_base_category.get_object(&mapped_object).is_ok() {
                    // add the result mapping to the result object mapping
                    result_object_mapping.insert(object.clone(), mapped_object);
                }
                else {
                    return Err(NCategoryError::InvalidObjectMapping);
                }

                // now map the cells of the object
                let object_cells = source_object_base_category.get_object_cells(object)?;
                for object_cell in object_cells {
                    // apply the cell to the object cell
                    let mapped_cell = cell.apply_cell_id(object_cell)?;

                    // check if the result is in the target object
                    if target_object_base_category.get_cell(&mapped_cell).is_ok() {
                        // add the result mapping to the result object cells mapping
                        result_object_cells_mapping.insert(object_cell, mapped_cell);
                    } else {
                        return Err(NCategoryError::InvalidCellMapping);
                    }
                }
            }
        }
        Ok(true)
    }

    fn base_object(&self, object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError>;

    fn category_level() -> isize
    where
        Self: Sized,
    {
        1 + <Self::BaseCategory as NCategory>::category_level()
    }
    fn validate_cell_commutation(
        &self,
        left_cell_id: Vec<&Self::CellId>,
        right_cell_id: Vec<&Self::CellId>,
    ) -> Result<(Vec<&Self::Cell>, Vec<&Self::Cell>), NCategoryError>
    {
        if left_cell_id.is_empty() || right_cell_id.is_empty() {
            return Err(NCategoryError::InvalidCellCommutation);
        }

        let left_source = self.source(left_cell_id[0])?;
        let right_source = self.source(right_cell_id[0])?;

        if left_source != right_source {
            return Err(NCategoryError::InvalidCellCommutation);
        }

        let left_target = self.target(left_cell_id[0])?;
        let right_target = self.target(right_cell_id[0])?;

        if left_target != right_target {
            return Err(NCategoryError::InvalidCellCommutation);
        }

        // validate cells composition
        let left_cells = self.validate_cell_composition(left_cell_id)?;
        let right_cells = self.validate_cell_composition(right_cell_id)?;

        Ok((left_cells, right_cells))
    }

    fn validate_cell_composition(&self, cell_ids: Vec<&Self::CellId>) -> Result<Vec<&Self::Cell>, NCategoryError>{
        if cell_ids.is_empty() {
            return Err(NCategoryError::InvalidCellComposition);
        }

        let mut result_cells : Vec<&Self::Cell> = Vec::new();

        let mut cell_target = self.source(cell_ids[0])?;
        for cell_id in &cell_ids[1..] {
            if cell_target != self.source(cell_id)? {
                return Err(NCategoryError::InvalidCellComposition);
            }
            cell_target = self.target(cell_id)?;
            let cell = self.get_cell(cell_id)?;
            result_cells.push(cell);
        }

        Ok(result_cells)
    }
}



impl NCategory for () {
    type Object = ();
    type ObjectId = ();
    type CellId = ();
    type Cell = ();
    type BaseCategory = ();

    fn source(&self, _cell_id: &Self::CellId) -> Result<&Self::Object, NCategoryError> { Ok(self) }
    fn target(&self, _cell_id: &Self::CellId) -> Result<&Self::Object, NCategoryError> { Ok(self) }
    fn add_object(&mut self, _object: Self::Object) -> Result<Self::ObjectId, NCategoryError> { Ok(*self) }
    fn add_object_with_id(&mut self, _object_id: Self::ObjectId, _object: Self::Object)-> Result<(), NCategoryError>{Ok(())}
    fn add_cell(&mut self, _cell: Self::Cell) -> Result<Self::CellId, NCategoryError> {Ok(())}
    fn get_object(&self, _id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError> { Ok(self) }

    fn get_identity_cell(&self, object_id: &Self::ObjectId) -> Result<&Self::CellId, NCategoryError> {
        // it should be cell with the same id as the object id
        self.get_cell(object_id)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::ObjectId>, NCategoryError> {
        Ok(HashSet::new())
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::CellId>, NCategoryError> {
        Ok(HashSet::new())
    }

    fn get_object_cells(&self, _object_id: &Self::ObjectId) -> Result<Vec<&Self::Cell>, NCategoryError> { Ok(vec![self]) }
    fn get_cell(&self, _cell_id: &Self::CellId) -> Result<&Self::Cell, NCategoryError> { Ok(self) }

    fn apply_cells_recursive(&self, _cell_id: &Self::CellId, _cell_id_to_map: &Self::CellId) -> Result<RecursiveCellMap<Self::CellId>, NCategoryError> {
        todo!()
    }

    fn commute(&self, _left: Vec<&Self::Cell>, _right: Vec<&Self::Cell>) -> Result<bool, NCategoryError> { Ok(true) }

    fn base_object(&self, _object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        Ok(self)
    }

    fn category_level() -> isize { -1 }
}

impl CellTrait for () {
    type BaseCategory = ();
    type CurrentCategoryCellId = ();

    fn id(&self) -> &Self::CurrentCategoryCellId { &() }

    fn base_cell_id(&self) -> Result<<Self::BaseCategory as NCategory>::CellId, NCategoryError> {
        Ok(())
    }

    fn apply_cell_id(&self, _cell_id: &<Self::BaseCategory as NCategory>::CellId) -> Result<<Self::BaseCategory as NCategory>::CellId, NCategoryError> {
        Ok(())
    }

    fn apply_to_object(&self, _object_id: &<Self::BaseCategory as NCategory>::ObjectId) -> Result<<Self::BaseCategory as NCategory>::ObjectId, NCategoryError> {
        Ok(())
    }
}