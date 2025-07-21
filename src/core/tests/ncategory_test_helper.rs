use rand::{distributions::Alphanumeric, Rng};
use crate::core::ncategory::{NCategory};

pub trait NCategoryTestHelper {
    type category: NCategory;
    fn get_category(&self) -> &Self::category;
    fn get_mut_category(&mut self) -> &mut Self::category;
    fn generate_object_id(&self) -> <Self::category as NCategory>::ObjectId;
    fn generate_cell_id(&self) -> <Self::category as NCategory>::CellId;
    fn generate_object(&self) -> <Self::category as NCategory>::Object;
}


pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn basic_object_cell_test(mut category_test_helper: impl NCategoryTestHelper){
    // add object 1
    let object1_id = category_test_helper.generate_object_id();
    let object1 = category_test_helper.generate_object();
    let object2_id = category_test_helper.generate_object_id();;
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object1_id.clone(), object1);
        assert!(category.get_object(&object1_id).is_some());

        // check identity morphism
        let cell = category.get_object_cells(&object1_id);
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(category.source(cell), &object1_id);
        assert_eq!(category.target(cell), &object1_id);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // check object 2 does not exist yet
        assert!(!category.get_object(&object2_id).is_some());
    }

    // add object 2
    let object2 = category_test_helper.generate_object();
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object2_id.clone(), object2);
        assert!(category.get_object(&object2_id).is_some());

        // check identity morphism
        let cell = category.get_object_cells(&object2_id);
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(category.source(cell), &object2_id);
        assert_eq!(category.target(cell), &object2_id);
    }

    // add object 3 without id
    let object3_id = {
        let object3 = category_test_helper.generate_object();
        let category = category_test_helper.get_mut_category();
        let object_id = category.add_object(object3);
        object_id.clone()
    };
    // check object 3 exists
    {
        let category = category_test_helper.get_category();
        assert!(category.get_object(&object3_id).is_some());

        // check identity morphism
        let cell = category.get_object_cells(&object3_id);
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(category.source(cell), &object3_id);
        assert_eq!(category.target(cell), &object3_id);
    }
}
