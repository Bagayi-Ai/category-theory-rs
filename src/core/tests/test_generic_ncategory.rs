use std::cell::Cell;
use crate::core::tests::ncategory_test_helper::*;
use crate::core::discrete_category::{DiscreteCategory};
use crate::core::generic_ncategory::*;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::cell_tree::CellTree;
use crate::core::ncell::NCell;
use crate::core::generic_ncell::GenericNCell;
use crate::core::identifier::Identifier;

type DiscreteCategoryString = DiscreteCategory<String>;

pub struct GenericCategory1TestHelper<'a> {
    category: GenericNCategory<'a, String, DiscreteCategoryString>,
}

impl <'a> GenericCategory1TestHelper<'a> {
    pub fn new() -> Self {
        GenericCategory1TestHelper {
            category: GenericNCategory::new(),
        }
    }
}

impl <'a> NCategoryTestHelper<'a> for GenericCategory1TestHelper<'a> {
    type Category = GenericNCategory<'a, String, DiscreteCategoryString>;

    type CategoryObject = DiscreteCategoryString;

    fn get_category(&self) -> &Self::Category {
        &self.category
    }

    fn get_mut_category(&mut self) -> &mut Self::Category {
        &mut self.category
    }

    fn generate_cell(&mut self) -> <Self::Category as NCategory<'a>>::Identifier {
        // let object1 = self.generate_object();
        // let object2 = self.generate_object();
        // let object1_id = self.get_mut_category().add_object(&object1).unwrap();
        // let object2_id = self.get_mut_category().add_object(&object2).unwrap();
        // let cell_id = self.generate_identifier();
        // let cell = GenericNCell::new(
        //     cell_id.clone(),
        //     object1_id,
        //     object2_id,
        //     "test_cell".to_string());
        // self.get_mut_category().add_cell(cell).unwrap();
        // cell_id
        todo!()
    }

    fn generate_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory<'a>>::Identifier>, Vec<<Self::Category as NCategory<'a>>::Identifier>) {
        // have 3 DiscreteCategory objects A, B, C
        // A will contain {a,b,c} as objects
        // B will contain {1, 2, 3, 4, 5} as objects
        // C will contain {x, y, z} as objects
        // let object_a = DiscreteCategory::from(
        //     vec!["a", "b", "c"]
        //         .into_iter()
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>());
        // let object_a_id = self.get_mut_category().add_object(NCategory::id(&object_a)).unwrap();
        // let object_b = DiscreteCategory::from(
        //     vec!["1", "2", "3", "4", "5"]
        //         .into_iter()
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>());
        // let object_b_id = self.get_mut_category().add_object(NCategory::id(&object_b)).unwrap();
        // let object_c = DiscreteCategory::from(
        //     vec!["x", "y", "z"]
        //         .into_iter()
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>());
        // let object_c_id = self.get_mut_category().add_object(NCategory::id(&object_c)).unwrap();
        //
        // // We add three Cells that are commuting
        // // A -> B, B -> C, C -> A
        // let cell1_id = self.generate_identifier();
        // let cell1 = GenericNCell::new(
        //     cell1_id.clone(),
        //     object_a_id.clone(),
        //     object_b_id.clone(),
        //     "cell_A_to_B".to_string(),
        // );
        // self.get_mut_category().add_cell(cell1).unwrap();
        //
        // let cell2_id = self.generate_identifier();
        // let cell2 = GenericNCell::new(
        //     cell2_id.clone(),
        //     object_b_id.clone(),
        //     object_c_id.clone(),
        //     "cell_B_to_C".to_string());
        // self.get_mut_category().add_cell(cell2).unwrap();
        //
        // let cell3_id = self.generate_identifier();
        // let cell3 = GenericNCell::new(
        //     cell3_id.clone(),
        //     object_a_id.clone(),
        //     object_c_id.clone(),
        //     "cell_C_to_A".to_string(),
        // );
        // self.get_mut_category().add_cell(cell3).unwrap();
        //
        // (
        //     vec![cell1_id, cell2_id],
        //     vec![cell3_id],
        // )
        todo!()
    }

    fn generate_non_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory<'a>>::Identifier>, Vec<<Self::Category as NCategory<'a>>::Identifier>) {
        // have 3 DiscreteCategory objects D, E, F
        // D will contain {da, db, dc} as objects
        // E will contain {11, 12, 13, 14, 15} as objects
        // F will contain {xa, yb, zc} as objects
        // let object_d = DiscreteCategory::from(
        //     vec!["da", "db", "dc"]
        //         .into_iter()
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>());
        // let object_d_id = self.get_mut_category().add_object(NCategory::id(&object_d)).unwrap();
        // let object_e = DiscreteCategory::from(
        //     vec!["11", "12", "13", "14", "15"]
        //         .into_iter()
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>());
        // let object_e_id = self.get_mut_category().add_object(NCategory::id(&object_e)).unwrap();
        // let object_f = DiscreteCategory::from(
        //     vec!["xa", "yb", "zc"]
        //         .into_iter()
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>());
        // let object_f_id = self.get_mut_category().add_object(NCategory::id(&object_f)).unwrap();
        //
        // // We add three Cells that are commuting
        // // D -> E, E -> F, D -> F
        // let cell1_id = self.generate_identifier();
        // let cell1 = GenericNCell::new(
        //     cell1_id.clone(),
        //     object_d_id.clone(),
        //     object_e_id.clone(),
        //     "cell_D_to_E".to_string(),
        // );
        // self.get_mut_category().add_cell(cell1).unwrap();
        //
        // let cell2_id = self.generate_identifier();
        // let cell2 = GenericNCell::new(
        //     cell2_id.clone(),
        //     object_e_id.clone(),
        //     object_f_id.clone(),
        //     "cell_E_to_F".to_string(),
        // );
        // self.get_mut_category().add_cell(cell2).unwrap();
        //
        // let cell3_id = self.generate_identifier();
        // let cell3 = GenericNCell::new(
        //     cell3_id.clone(),
        //     object_d_id.clone(),
        //     object_f_id.clone(),
        //     "cell_D_to_F".to_string(),
        // );
        // self.get_mut_category().add_cell(cell3).unwrap();
        //
        // (
        //     vec![cell1_id, cell2_id],
        //     vec![cell3_id],
        // )
        todo!()
    }

    fn generate_object(&self) -> Self::CategoryObject {
        let random_string = random_string(5);
        let mut object = DiscreteCategory::new();
        object.add_object(random_string).unwrap();
        object
    }

    fn expected_nested_level(&self) -> isize {
        2
    }

}


fn generate_identifier() -> String {
    String::generate()
}

fn generate_object() -> DiscreteCategoryString {
    let random_string = random_string(5);
    let mut object = DiscreteCategory::new();
    object.add_object(random_string).unwrap();
    object
}


#[test]
pub fn test_base_scenarios() {
    let mut category = GenericNCategory::new();
    // add object 1
    let object1 = generate_object();
    let object1_id = NCategory::id(&object1).clone();
    let object2_id = generate_identifier();

    category.add_object(&object1).unwrap();
    assert!(category.get_object(&object1_id).is_ok());
    // check identity morphism
    let cell = category.get_object_cells(&object1);
    assert!(cell.is_ok());
    let cell = cell.unwrap();
    assert_eq!(cell.len(), 1);
    let cell = cell.first().unwrap();
    assert_eq!(cell.source_object_id(), &object1_id);
    assert_eq!(cell.target_object_id(), &object1_id);

    // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

    // check object 2 does not exist yet
    assert!(!category.get_object(&object2_id).is_ok());

    // check identity morphism
    let cell = category.get_object_cells(&object1);
    assert!(cell.is_ok());
    let cell = cell.unwrap();
    assert_eq!(cell.len(), 1);
    let cell = cell.first().unwrap();
    assert_eq!(cell.source_object_id(), &object1_id);
    assert_eq!(cell.target_object_id(), &object1_id);

    // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

    // check object 2 does not exist yet
    assert!(!category.get_object(&object2_id).is_ok());

    // add object 2
    let object2 = generate_object();
    category.add_object(&object2).unwrap();
    let object2_id = NCategory::id(&object2).clone();
    assert!(category.get_object(&object2_id).is_ok());

    // check identity morphism
    let cells = category.get_object_cells(&object2);
    assert!(cells.is_ok());
    let cells = cells.unwrap();
    assert_eq!(cells.len(), 1);
    let cell = cells.first().unwrap();
    assert_eq!(cell.source_object_id(), &object2_id);
    assert_eq!(cell.target_object_id(), &object2_id);

    // add object 3
    let object3 = generate_object();
    let object3_id = NCategory::id(&object3);
    category.add_object(&object3);

    // check object 3 exists
    assert!(category.get_object(&object3_id).is_ok());

    // check identity morphism
    let cells = category.get_object_cells(&object3);
    assert!(cells.is_ok());
    let cells = cells.unwrap();
    assert_eq!(cells.len(), 1);
    let cell = cells.first().unwrap();
    assert_eq!(cell.source_object_id(), object3_id);
    assert_eq!(cell.target_object_id(), object3_id);

    // now add a cell between object1 and object2
    let cell_id = generate_identifier();
    let cell = GenericNCell::new(
        cell_id.clone(),
        object1_id.clone(),
        object2_id.clone(),
        "obj1 to obj2".to_string());
    category.add_cell(cell).unwrap();


    let cell = category.get_cell(&cell_id).unwrap();
    let source_id = cell.source_object_id();
    let target_id = cell.target_object_id();
    assert!(category.get_object(source_id).is_ok());
    assert!(category.get_object(target_id).is_ok());

    //
    // {
    //     // now we test for the commuting cells
    //     let (commuting_cell1, commuting_cell2) = category_test_helper.generate_commuting_cell();
    //     let commute_result = category_test_helper.get_category().cells_commute(
    //         commuting_cell1.iter().collect(),
    //         commuting_cell2.iter().collect()
    //     );
    //     assert!(commute_result.is_ok());
    //     let commute_result = commute_result.unwrap();
    //     assert!(commute_result);
    // }

    // {
    //     // now we test for the non-commuting cells
    //     let (commuting_cell1, commuting_cell2) = category_test_helper.generate_non_commuting_cell();
    //     let commute_result = category_test_helper.get_category().cells_commute(
    //         commuting_cell1.iter().collect(),
    //         commuting_cell2.iter().collect()
    //     );
    //     assert!(commute_result.is_ok());
    //     let commute_result = commute_result.unwrap();
    //     assert!(!commute_result);
    // }
}

#[test]
pub fn test_identity_cell_tree() {
    let mut setCategoryAlphabetLower = GenericNCategory::new();

    // Discrete category A with a, b, c as objects
    let mut discreteCategoryALower = DiscreteCategory::new_with_id("alphabet_lower".to_string());
    let object1_a = "a".to_string();
    let object1_b = "b".to_string();
    let object1_c = "c".to_string();
    discreteCategoryALower.add_object(object1_a).unwrap();
    discreteCategoryALower.add_object(object1_b).unwrap();
    discreteCategoryALower.add_object(object1_c).unwrap();

    // Add the discrete category A as an object in Set category alphabet
    let setCategoryAlphabetLowerId = setCategoryAlphabetLower.add_object(
        &discreteCategoryALower).unwrap();

    let identity_cell = setCategoryAlphabetLower.get_identity_cell(&discreteCategoryALower).unwrap();
    assert_eq!(identity_cell.source_object_id(), NCategory::id(&discreteCategoryALower));
    assert_eq!(identity_cell.target_object_id(), NCategory::id(&discreteCategoryALower));

    // let acutal_cell_tree = category.get_cell_tree(identity_cell).unwrap();
    //
    // // expected cell tree
    // let mut  expected_cell_tree = CellTree::new(
    //     identity_cell.id(),
    //     identity_cell.source_object_id(),
    //     identity_cell.target_object_id(),
    // );
    // // now add children cell tree for the identity cell.
    // // for our case we know its identity cell of the base category.
    // for cell in object1.get_all_cells().unwrap() {
    //     let childern_cell = cell.get_all_cells().unwrap();
    //     expected_cell_tree.add_child(
    //         CellTree::new(
    //             NCategory::id(cell),
    //             cell.source_object_id(),
    //             cell.target_object_id(),
    //         )
    //     );
    // }
    //
    // assert_eq!(acutal_cell_tree, expected_cell_tree);
}