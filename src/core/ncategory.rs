use std::hash::Hash;
use std::fmt::Debug;

pub trait NCategory {
    type Object;
    type ObjectId: Eq + Hash + Clone + Debug;
    type CellId: Eq + Hash + Clone;
    type Cell;
    type BaseCategory: NCategory;

    fn source(&self, cell_id: &Self::CellId) -> &Self::ObjectId;

    fn target(&self, cell_id: &Self::CellId) -> &Self::ObjectId;

    fn add_object(&mut self, object: Self::Object) -> &Self::ObjectId;

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object);

    fn add_cell(&mut self, cell: Self::Cell);

    fn get_object(&self, objectId: &Self::ObjectId) -> Option<&Self::Object>;

    fn get_object_cells(
        &self,
        objectId: &Self::ObjectId,
    ) -> Vec<&Self::CellId>;

    fn get_cell(&self, cell_id: &Self::CellId) -> Option<&Self::Cell>;

    fn commute(left_cell_id: &Self::CellId, right_cell_id: &Self::CellId) -> bool;

    fn base_object(&self, object_id: &Self::ObjectId) -> &Self::BaseCategory;
}

impl NCategory for () {
    type Object = ();
    type ObjectId = ();
    type CellId = ();
    type Cell = ();
    type BaseCategory = ();

    fn source(&self, _cell_id: &Self::CellId) -> &Self::Object { self }
    fn target(&self, _cell_id: &Self::CellId) -> &Self::Object { self }
    fn add_object(&mut self, _object: Self::Object) -> &Self::ObjectId { self }
    fn add_object_with_id(&mut self, _object_id: Self::ObjectId, _object: Self::Object) {}
    fn add_cell(&mut self, _cell: Self::Cell) {}
    fn get_object(&self, _id: &Self::ObjectId) -> Option<&Self::Object> { Some(self) }
    fn get_object_cells(&self, _object_id: &Self::ObjectId) -> Vec<&Self::Cell> { vec![self] }
    fn get_cell(&self, _cell_id: &Self::CellId) -> Option<&Self::Cell> { Some(self) }
    fn commute(_left: &Self::Cell, _right: &Self::Cell) -> bool { true }
    
    fn base_object(&self, object_id: &Self::ObjectId) -> &Self::BaseCategory {
        self
    }
}