use rand::{distributions::Alphanumeric, Rng};
use crate::core::ncategory::{NCategory};

pub trait NCategoryTestHelper {
    type Category: NCategory;
    fn get_category(&self) -> &Self::Category;
    fn get_mut_category(&mut self) -> &mut Self::Category;
    fn generate_object_id(&self) -> <Self::Category as NCategory>::ObjectId;
    fn generate_cell_id(&self) -> <Self::Category as NCategory>::CellId;
    fn generate_cell(&mut self) -> <Self::Category as NCategory>::CellId;

    fn generate_commuting_cell(
        &mut self
    ) -> (Vec<<Self::Category as NCategory>::CellId>, Vec<<Self::Category as NCategory>::CellId>);

    fn generate_non_commuting_cell(
        &mut self
    ) -> (Vec<<Self::Category as NCategory>::CellId>, Vec<<Self::Category as NCategory>::CellId>);

    fn generate_object(&mut self) -> <Self::Category as NCategory>::Object;

    fn expected_category_level(&self) -> isize;
}


pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn basic_object_cell_test<CategoryTestHelper: NCategoryTestHelper>(mut category_test_helper: CategoryTestHelper){
    // add object 1
    let object1_id = category_test_helper.generate_object_id();
    let object1 = category_test_helper.generate_object();
    let object2_id = category_test_helper.generate_object_id();
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object1_id.clone(), object1).unwrap();
        assert!(category.get_object(&object1_id).is_ok());

        // check identity morphism
        let cell = category.get_object_cells(&object1_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(category.source(cell).unwrap(), &object1_id);
        assert_eq!(category.target(cell).unwrap(), &object1_id);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // check object 2 does not exist yet
        assert!(!category.get_object(&object2_id).is_ok());
    }

    // add object 2
    let object2 = category_test_helper.generate_object();
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object2_id.clone(), object2).unwrap();
        assert!(category.get_object(&object2_id).is_ok());

        // check identity morphism
        let cell = category.get_object_cells(&object2_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(category.source(cell).unwrap(), &object2_id);
        assert_eq!(category.target(cell).unwrap(), &object2_id);
    }

    // add object 3 without id
    let object3_id = {
        let object3 = category_test_helper.generate_object();
        let category = category_test_helper.get_mut_category();
        let object_id = category.add_object(object3);
        object_id.clone()
    };
    assert!(object3_id.is_ok());
    let object3_id = object3_id.unwrap();
    // check object 3 exists
    {
        let category = category_test_helper.get_category();
        assert!(category.get_object(&object3_id).is_ok());

        // check identity morphism
        let cell = category.get_object_cells(&object3_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(category.source(cell).unwrap(), &object3_id);
        assert_eq!(category.target(cell).unwrap(), &object3_id);
    }

    let level = <<CategoryTestHelper as NCategoryTestHelper>::Category as NCategory>::category_level();
    assert_eq!(level, category_test_helper.expected_category_level());

    if level == 0 {
        // end the test here
        return;
    }


    {
        // now we add two objects and a cell between them
        let cell_id = category_test_helper.generate_cell();
        let category = category_test_helper.get_category();
        let source_id = category.source(&cell_id);
        let target_id = category.target(&cell_id);
        assert!(category.get_object(source_id.unwrap()).is_ok());
        assert!(category.get_object(target_id.unwrap()).is_ok());
    }

    {
        // now we test for the commuting cells
        let (commuting_cell1, commuting_cell2) = category_test_helper.generate_commuting_cell();
        let commute_result = category_test_helper.get_category().commute(
            commuting_cell1.iter().collect(),
            commuting_cell2.iter().collect()
        );
        assert!(commute_result.is_ok());
        let commute_result = commute_result.unwrap();
        assert!(commute_result);
    }

    {
        // now we test for the non-commuting cells
        let (commuting_cell1, commuting_cell2) = category_test_helper.generate_non_commuting_cell();
        let commute_result = category_test_helper.get_category().commute(
            commuting_cell1.iter().collect(),
            commuting_cell2.iter().collect()
        );
        assert!(commute_result.is_ok());
        let commute_result = commute_result.unwrap();
        assert!(!commute_result);
    }

}
