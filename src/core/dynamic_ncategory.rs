use crate::core::ncategory::NCategory;

#[derive(Clone, Debug)]
struct CellDynamic<T> {
    from: T,
    to: T,
    name: String,
    children: Vec<CellDynamic<T>>,
}

struct DynamicNCategory<T> {
    _phantom: std::marker::PhantomData<T>,
}


impl <T: Clone + PartialEq> NCategory for DynamicNCategory<T>
where
    T: Clone + PartialEq,
{
    type Object = T;
    type ObjectId = ();
    type CellId = ();
    type Cell = CellDynamic<Self::ObjectId>;
    type Higher = Self;

    fn source(m: &Self::Cell) -> &Self::ObjectId {
        &m.from
    }

    fn target(m: &Self::Cell) -> &Self::ObjectId {
        &m.to
    }

    fn add_object(&mut self, o: Self::Object) -> &Self::ObjectId {
        todo!()
    }

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object) -> Self::ObjectId {
        todo!()
    }

    fn add_cell(&mut self, m: Self::Cell) {
        todo!()
    }

    fn get_object(&self, id: &Self::ObjectId) -> Option<&Self::Object> {
        todo!()
    }

    fn get_cell(&self, id: &Self::CellId) -> Option<&Self::Cell> {
        todo!()
    }

    fn commute(left: &Self::CellId, right: &Self::CellId) -> bool {
        todo!()
    }
}