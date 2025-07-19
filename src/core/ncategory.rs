use std::hash::Hash;

pub trait NCategory {
    type Object;
    type ObjectId: Eq + Hash + Clone;
    type CellId: Eq + Hash + Clone;
    type Cell;
    type Higher: NCategory;

    fn source(cell: &Self::Cell) -> &Self::ObjectId;

    fn target(cell: &Self::Cell) -> &Self::ObjectId;

    fn add_object(&mut self, object: Self::Object) -> &Self::ObjectId;

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object);

    fn add_cell(&mut self, cell: Self::Cell);

    fn get_object(&self, objectId: &Self::ObjectId) -> Option<&Self::Object>;

    fn get_cell(&self, cellId: &Self::CellId) -> Option<&Self::Cell>;

    fn commute(left: &Self::CellId, right: &Self::CellId) -> bool;

}