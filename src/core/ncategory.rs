use std::hash::Hash;
use std::fmt::Debug;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NCategoryError {
    CellAlreadyExists,
    CellNotFound,
    ObjectNotFound,
}

pub trait NCategory {
    type Object;
    type ObjectId: Eq + Hash + Clone + Debug;
    type CellId: Eq + Hash + Clone;
    type Cell;
    type BaseCategory: NCategory;

    fn source(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError>;

    fn target(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError>;

    fn add_object(&mut self, object: Self::Object) -> Result<Self::ObjectId, NCategoryError>;

    fn add_object_with_id(&mut self, object_id: Self::ObjectId, object: Self::Object) -> Result<(), NCategoryError>;

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::CellId, NCategoryError>;

    fn get_object(&self, object_id: &Self::ObjectId) -> Result<&Self::Object, NCategoryError>;

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

    fn commute(left_cell_id: &Self::CellId, right_cell_id: &Self::CellId) -> Result<bool, NCategoryError>;

    fn base_object(&self, object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError>;

    fn category_level() -> isize
    where
        Self: Sized,
    {
        1 + <Self::BaseCategory as NCategory>::category_level()
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
    fn get_object_cells(&self, _object_id: &Self::ObjectId) -> Result<Vec<&Self::Cell>, NCategoryError> { Ok(vec![self]) }
    fn get_cell(&self, _cell_id: &Self::CellId) -> Result<&Self::Cell, NCategoryError> { Ok(self) }
    fn commute(_left: &Self::Cell, _right: &Self::Cell) -> Result<bool, NCategoryError> { Ok(true) }
    
    fn base_object(&self, _object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        Ok(self)
    }

    fn category_level() -> isize { -1 }
}