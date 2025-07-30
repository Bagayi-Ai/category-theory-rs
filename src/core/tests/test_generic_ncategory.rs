use std::cell::Cell;
use crate::core::tests::ncategory_test_helper::*;
use crate::core::discrete_category::{DiscreteCategory};
use crate::core::generic_ncategory::*;
use crate::core::ncategory::{NCategory};
use crate::core::cell_tree::CellTree;
use crate::core::ncell::NCell;
use crate::core::generic_ncell::GenericNCell;

type DiscreteCategoryString = DiscreteCategory<String>;

struct GenericCategory1TestHelper {
    category: GenericNCategory<String, DiscreteCategoryString>,
}

impl GenericCategory1TestHelper {
    pub fn new() -> Self {
        GenericCategory1TestHelper {
            category: GenericNCategory::new(),
        }
    }
}

impl NCategoryTestHelper for GenericCategory1TestHelper {
    type Category = GenericNCategory<String, DiscreteCategoryString>;

    fn get_category(&self) -> &Self::Category {
        &self.category
    }

    fn get_mut_category(&mut self) -> &mut Self::Category {
        &mut self.category
    }

    fn generate_cell(&mut self) -> <Self::Category as NCategory>::Identifier {
        let object1 = self.generate_object();
        let object2 = self.generate_object();
        let object1_id = self.get_mut_category().add_object(object1).unwrap();
        let object2_id = self.get_mut_category().add_object(object2).unwrap();
        let cell_id = self.generate_identifier();
        let cell = GenericNCell::new(
            cell_id.clone(),
            object1_id,
            object2_id,
            "test_cell".to_string());
        self.get_mut_category().add_cell(cell).unwrap();
        cell_id
    }

    fn generate_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory>::Identifier>, Vec<<Self::Category as NCategory>::Identifier>) {
        // have 3 DiscreteCategory objects A, B, C
        // A will contain {a,b,c} as objects
        // B will contain {1, 2, 3, 4, 5} as objects
        // C will contain {x, y, z} as objects
        let object_a = DiscreteCategory::from(
            vec!["a", "b", "c"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        let object_a_id = self.get_mut_category().add_object(object_a).unwrap();
        let object_b = DiscreteCategory::from(
            vec!["1", "2", "3", "4", "5"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        let object_b_id = self.get_mut_category().add_object(object_b).unwrap();
        let object_c = DiscreteCategory::from(
            vec!["x", "y", "z"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        let object_c_id = self.get_mut_category().add_object(object_c).unwrap();

        // We add three Cells that are commuting
        // A -> B, B -> C, C -> A
        let cell1_id = self.generate_identifier();
        let cell1 = GenericNCell::new(
            cell1_id.clone(),
            object_a_id.clone(),
            object_b_id.clone(),
            "cell_A_to_B".to_string(),
        );
        self.get_mut_category().add_cell(cell1).unwrap();

        let cell2_id = self.generate_identifier();
        let cell2 = GenericNCell::new(
            cell2_id.clone(),
            object_b_id.clone(),
            object_c_id.clone(),
            "cell_B_to_C".to_string());
        self.get_mut_category().add_cell(cell2).unwrap();

        let cell3_id = self.generate_identifier();
        let cell3 = GenericNCell::new(
            cell3_id.clone(),
            object_a_id.clone(),
            object_c_id.clone(),
            "cell_C_to_A".to_string(),
        );
        self.get_mut_category().add_cell(cell3).unwrap();

        (
            vec![cell1_id, cell2_id],
            vec![cell3_id],
        )
    }

    fn generate_non_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory>::Identifier>, Vec<<Self::Category as NCategory>::Identifier>) {
        // have 3 DiscreteCategory objects D, E, F
        // D will contain {da, db, dc} as objects
        // E will contain {11, 12, 13, 14, 15} as objects
        // F will contain {xa, yb, zc} as objects
        let object_d = DiscreteCategory::from(
            vec!["da", "db", "dc"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        let object_d_id = self.get_mut_category().add_object(object_d).unwrap();
        let object_e = DiscreteCategory::from(
            vec!["11", "12", "13", "14", "15"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        let object_e_id = self.get_mut_category().add_object(object_e).unwrap();
        let object_f = DiscreteCategory::from(
            vec!["xa", "yb", "zc"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        let object_f_id = self.get_mut_category().add_object(object_f).unwrap();

        // We add three Cells that are commuting
        // D -> E, E -> F, D -> F
        let cell1_id = self.generate_identifier();
        let cell1 = GenericNCell::new(
            cell1_id.clone(),
            object_d_id.clone(),
            object_e_id.clone(),
            "cell_D_to_E".to_string(),
        );
        self.get_mut_category().add_cell(cell1).unwrap();

        let cell2_id = self.generate_identifier();
        let cell2 = GenericNCell::new(
            cell2_id.clone(),
            object_e_id.clone(),
            object_f_id.clone(),
            "cell_E_to_F".to_string(),
        );
        self.get_mut_category().add_cell(cell2).unwrap();

        let cell3_id = self.generate_identifier();
        let cell3 = GenericNCell::new(
            cell3_id.clone(),
            object_d_id.clone(),
            object_f_id.clone(),
            "cell_D_to_F".to_string(),
        );
        self.get_mut_category().add_cell(cell3).unwrap();

        (
            vec![cell1_id, cell2_id],
            vec![cell3_id],
        )
    }

    fn generate_object(&mut self) -> <Self::Category as NCategory>::Object {
        let random_string = random_string(5);
        let mut object = DiscreteCategory::new();
        object.add_object(random_string).unwrap();
        object
    }

    fn expected_nested_level(&self) -> isize {
        2
    }

}
#[test]
pub fn test_base_scenarios() {
    let category_test_helper = GenericCategory1TestHelper::new();
    basic_object_cell_test(category_test_helper);
}

#[test]
pub fn test_identity_cell_tree() {
    // starting with identity cell
    let mut category_test_helper = GenericCategory1TestHelper::new();
    let object1_id = category_test_helper.generate_identifier();
    let mut object1 = DiscreteCategory::new();
    object1.add_object("a".to_string()).unwrap();
    object1.add_object("b".to_string()).unwrap();
    object1.add_object("c".to_string()).unwrap();
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object1_id.clone(), object1).unwrap();
    }
    let category = category_test_helper.get_category();
    let identity_cell = category.get_identity_cell(&object1_id).unwrap();
    assert_eq!(identity_cell.source_object_id(), &object1_id);
    assert_eq!(identity_cell.target_object_id(), &object1_id);

    // let cell_tree = category.get_cell_tree(&object1_id).unwrap();
    //
    // // expected cell tree
    // let expected_cell_tree = CellTree::new(
    //     identity_cell.id(),
    //     identity_cell.source_object_id(),
    //     identity_cell.target_object_id(),
    // );
    // now add children cell tree for the identity cell.
    // for our case we know its identity cell of the base category.
    // for cells in object1.get_all_cells() {
    //     let cell = category.get_cell(cells).unwrap();
    //     expected_cell_tree.add_child(
    //         CellTree::new(
    //             cell.id(),
    //             cell.source_object_id(),
    //             cell.target_object_id(),
    //         )
    //     );
    // }
}