use crate::core::category::*;
use crate::core::discrete_category::{
    DiscreteCategory, DiscreteCategoryString, DiscreteCategoryUsize,
};
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::product_endofunctor::apply_product;
use crate::core::tests::ncategory_test_helper::*;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use std::rc::Rc;

fn generate_identifier() -> String {
    String::generate()
}

fn generate_object() -> DiscreteCategoryString {
    let random_string = random_string(5);
    let mut object = DiscreteCategory::new();
    object
        .add_object(Rc::new(DiscreteCategory::new_with_id(random_string)))
        .unwrap();
    object
}

#[test]
pub fn test_base_scenarios() {
    let mut category = Category::new();
    // add object 1
    let object1 = generate_object();
    let object1_id = CategoryTrait::category_id(&object1).clone();
    let object2_id = generate_identifier();

    category.add_object(object1.clone().into()).unwrap();
    // check identity morphism
    let cell = category.get_object_morphisms(&object1);
    assert!(cell.is_ok());
    let cell = cell.unwrap();
    assert_eq!(cell.len(), 1);
    let cell = cell.first().unwrap();
    assert_eq!(**cell.source_object(), object1);
    assert_eq!(**cell.target_object(), object1);

    // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

    // check identity morphism
    let cell = category.get_object_morphisms(&object1);
    assert!(cell.is_ok());
    let cell = cell.unwrap();
    assert_eq!(cell.len(), 1);
    let cell = cell.first().unwrap();
    assert_eq!(**cell.source_object(), object1);
    assert_eq!(**cell.target_object(), object1);

    // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

    // add object 2
    let object2 = generate_object();
    category.add_object(object2.clone().into()).unwrap();
    let object2_id = CategoryTrait::category_id(&object2).clone();

    // check identity morphism
    let cells = category.get_object_morphisms(&object2);
    assert!(cells.is_ok());
    let cells = cells.unwrap();
    assert_eq!(cells.len(), 1);
    let cell = cells.first().unwrap();
    assert_eq!(**cell.source_object(), object2);
    assert_eq!(**cell.target_object(), object2);

    // add object 3
    let object3 = generate_object();
    let object3_id = CategoryTrait::category_id(&object3);
    category.add_object(object3.clone().into());

    // check identity morphism
    let cells = category.get_object_morphisms(&object3);
    assert!(cells.is_ok());
    let cells = cells.unwrap();
    assert_eq!(cells.len(), 1);
    let cell = cells.first().unwrap();
    assert_eq!(**cell.source_object(), object3);
    assert_eq!(**cell.target_object(), object3);

    // // now add a cell between object1 and object2
    // let cell_id = generate_identifier();
    // let cell = Arrow::new(cell_id.clone(), &object1, &object2);
    // category.add_morphism(cell).unwrap();
    //
    // let cell = category.get_moprhism(&cell_id).unwrap();
    // assert_eq!(cell.source_object(), &object1);
    // assert_eq!(cell.target_object(), &object2);

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
    // Discrete category A with a, b, c as objects
    let mut discreteCategoryALower =
        Rc::new(DiscreteCategory::new_with_id("alphabet_lower".to_string()));
    let object_a: DiscreteCategory<String> = "a".to_string().into();
    let object_b: DiscreteCategory<String> = "b".to_string().into();
    let object_c: DiscreteCategory<String> = "c".to_string().into();
    Rc::get_mut(&mut discreteCategoryALower)
        .unwrap()
        .add_object(object_a.clone().into())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryALower)
        .unwrap()
        .add_object(object_b.clone().into())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryALower)
        .unwrap()
        .add_object(object_c.clone().into())
        .unwrap();

    // Discrete category A with a, b, c as objects
    let mut discreteCategoryAUpper =
        Rc::new(DiscreteCategory::new_with_id("alphabet_upper".to_string()));
    let object_A: DiscreteCategory<String> = "A".to_string().into();
    let object_B: DiscreteCategory<String> = "B".to_string().into();
    let object_C: DiscreteCategory<String> = "C".to_string().into();
    Rc::get_mut(&mut discreteCategoryAUpper)
        .unwrap()
        .add_object(object_A.clone().into())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryAUpper)
        .unwrap()
        .add_object(object_B.clone().into())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryAUpper)
        .unwrap()
        .add_object(object_C.clone().into())
        .unwrap();

    let mut discreteCategoryANumber = Rc::new(DiscreteCategory::new());
    let object_1: DiscreteCategory<usize> = 1.into();
    let object_2: DiscreteCategory<usize> = 2.into();
    let object_3: DiscreteCategory<usize> = 3.into();
    Rc::get_mut(&mut discreteCategoryANumber)
        .unwrap()
        .add_object(object_1.clone().into())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryANumber)
        .unwrap()
        .add_object(object_2.clone().into())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryANumber)
        .unwrap()
        .add_object(object_3.clone().into())
        .unwrap();

    let lower_to_numer_mappings: HashMap<
        Rc<DiscreteCategory<String>>,
        Rc<DiscreteCategory<usize>>,
    > = HashMap::from([
        // a to 1
        (
            discreteCategoryALower
                .get_identity_morphism(&object_a)
                .unwrap()
                .clone(),
            discreteCategoryANumber
                .get_identity_morphism(&object_1)
                .unwrap()
                .clone(),
        ),
        // b to 2
        (
            discreteCategoryALower
                .get_identity_morphism(&object_b)
                .unwrap()
                .clone(),
            discreteCategoryANumber
                .get_identity_morphism(&object_2)
                .unwrap()
                .clone(),
        ),
        // c to 3
        (
            discreteCategoryALower
                .get_identity_morphism(&object_c)
                .unwrap()
                .clone(),
            discreteCategoryANumber
                .get_identity_morphism(&object_3)
                .unwrap()
                .clone(),
        ),
    ]);
    // create a functor from lower to number
    let functor_lower_to_number = Functor::new(
        "functor_lower_to_number".to_string(),
        discreteCategoryALower.clone(),
        discreteCategoryANumber.clone(),
        lower_to_numer_mappings,
    );

    let number_to_upper_mappings: HashMap<Rc<DiscreteCategoryUsize>, Rc<DiscreteCategoryString>> =
        HashMap::from([
            // 1 to a
            (
                discreteCategoryANumber
                    .get_identity_morphism(&object_1)
                    .unwrap()
                    .clone(),
                discreteCategoryALower
                    .get_identity_morphism(&object_a)
                    .unwrap()
                    .clone(),
            ),
            // b to 2
            (
                discreteCategoryANumber
                    .get_identity_morphism(&object_2)
                    .unwrap()
                    .clone(),
                discreteCategoryALower
                    .get_identity_morphism(&object_b)
                    .unwrap()
                    .clone(),
            ),
            // c to c
            (
                discreteCategoryANumber
                    .get_identity_morphism(&object_3)
                    .unwrap()
                    .clone(),
                discreteCategoryALower
                    .get_identity_morphism(&object_c)
                    .unwrap()
                    .clone(),
            ),
        ]);
    let functor_number_to_upper = Functor::new(
        "functor_lower_to_number".to_string(),
        discreteCategoryANumber.clone(),
        discreteCategoryAUpper.clone(),
        number_to_upper_mappings,
    );

    // create a functor1 from lower to upper
    let lower_to_upper_mappings: HashMap<Rc<DiscreteCategoryString>, Rc<DiscreteCategoryString>> =
        HashMap::from([
            // a to A
            (
                discreteCategoryALower
                    .get_identity_morphism(&object_a)
                    .unwrap()
                    .clone(),
                discreteCategoryAUpper
                    .get_identity_morphism(&object_A)
                    .unwrap()
                    .clone(),
            ),
            // b to B
            (
                discreteCategoryALower
                    .get_identity_morphism(&object_b)
                    .unwrap()
                    .clone(),
                discreteCategoryAUpper
                    .get_identity_morphism(&object_B)
                    .unwrap()
                    .clone(),
            ),
            // c to C
            (
                discreteCategoryALower
                    .get_identity_morphism(&object_c)
                    .unwrap()
                    .clone(),
                discreteCategoryAUpper
                    .get_identity_morphism(&object_C)
                    .unwrap()
                    .clone(),
            ),
        ]);
    let functor_lower_to_upper = Rc::new(Functor::new(
        "functor_1".to_string(),
        discreteCategoryALower.clone(),
        discreteCategoryAUpper.clone(),
        lower_to_upper_mappings,
    ));

    let mut setCategoryAlphabet = Category::new();

    // Add the discrete category A as an object in Set category alphabet
    setCategoryAlphabet
        .add_object(discreteCategoryALower.clone())
        .unwrap();

    // Add the discrete category A as an object in Set category alphabet
    setCategoryAlphabet
        .add_object(discreteCategoryAUpper.clone())
        .unwrap();

    let identity_cell = setCategoryAlphabet
        .get_identity_morphism(&discreteCategoryALower)
        .unwrap();
    assert_eq!(identity_cell.source_object(), &discreteCategoryALower);
    assert_eq!(identity_cell.target_object(), &discreteCategoryALower);

    // now add morphism from lower to upper
    let morphism = Rc::new(Morphism::new(
        "lower_to_upper".to_string(),
        discreteCategoryALower.clone(),
        discreteCategoryAUpper.clone(),
        functor_lower_to_upper,
    ));
    setCategoryAlphabet.add_morphism(morphism).unwrap();

    // let product_endo_functor = apply_endofunctor(
    //     "product_endo_functor".to_string(),
    //     discreteCategoryALower.clone(),
    //     discreteCategoryANumber.clone(),
    // )
    // .unwrap();
    // // create a morphism from lower to upper using the functor
    // let morphism2 = Rc::new(Morphism::new(
    //     "lower_to_upper".to_string(),
    //     (*product_endo_functor.source_object()).clone(),
    //     (*product_endo_functor.target_object()).clone(),
    //     product_endo_functor,
    // ));
    //
    // // add the morphism to the set category alphabet
    // setCategoryAlphabet.add_morphism(morphism2).unwrap();

    // add a new object that is a result of

    // assert!(morphism.validate_composition().is_ok());
    // assert!(morphism.validate_commutation(&morphism).is_ok());
    //
    // assert!(functor_lower_to_upper.validate_composition().is_ok());
    // assert!(functor_lower_to_upper.validate_commutation(&functor_lower_to_upper).is_ok());
    // assert!(functor_lower_to_upper.validate_mappings().is_ok());

    // let actual_cell_tree = setCategoryAlphabet.get_cell_tree(identity_cell).unwrap();

    // expected cell tree
    // all the cells of the discrete category A remain as children of the identity cell
    // let expected_cell_tree = CellTree::new_with_children(
    //     identity_cell.id(),
    //     identity_cell.source_object_id(),
    //     identity_cell.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &object_a,
    //             &object_a,
    //             &object_a
    //         ),
    //         CellTree::new(
    //             &object_b,
    //             &object_b,
    //             &object_b
    //         ),
    //         CellTree::new(
    //             &object_c,
    //             &object_c,
    //             &object_c
    //         )
    //     ]
    // );
    //
    // assert_eq!(actual_cell_tree, expected_cell_tree);
    //

    // // Add the discrete category A as an object in Set category alphabet
    // setCategoryAlphabet.add_object(
    //     &discreteCategoryAUpper).unwrap();
    //
    // let identity_cell = setCategoryAlphabet.get_identity_cell(&discreteCategoryAUpper).unwrap();
    // assert_eq!(identity_cell.source_object_id(), NCategory::category_id(&discreteCategoryAUpper));
    // assert_eq!(identity_cell.target_object_id(), NCategory::category_id(&discreteCategoryAUpper));
    //
    // let actual_cell_tree = setCategoryAlphabet.get_cell_tree(identity_cell).unwrap();
    //
    // let expected_cell_tree = CellTree::new_with_children(
    //     identity_cell.id(),
    //     identity_cell.source_object_id(),
    //     identity_cell.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &object_A,
    //             &object_A,
    //             &object_A
    //         ),
    //         CellTree::new(
    //             &object_B,
    //             &object_B,
    //             &object_B
    //         ),
    //         CellTree::new(
    //             &object_C,
    //             &object_C,
    //             &object_C
    //         )
    //     ]
    // );
    //
    // assert_eq!(actual_cell_tree, expected_cell_tree);
    //
    // // now add a cell between the two objects
    // let cell_id = generate_identifier();
    // let cell = GenericNCell::new(
    //     cell_id.clone(),
    //     NCategory::category_id(&discreteCategoryALower).clone(),
    //     NCategory::category_id(&discreteCategoryAUpper).clone(),
    //     "lower to upper".to_string());
    // setCategoryAlphabet.add_cell(cell).unwrap();
    //
    // // add another cell between the two objects
    // // but its invalid semantically from lower to upper
    // let cell_id_lower_upper_reverse = generate_identifier();
    // let cell_lower_upper_reverse = GenericNCell::new(
    //     cell_id_lower_upper_reverse.clone(),
    //     NCategory::category_id(&discreteCategoryAUpper).clone(),
    //     NCategory::category_id(&discreteCategoryALower).clone(),
    //     "lower to upper reverse".to_string());
    // setCategoryAlphabet.add_cell(cell_lower_upper_reverse).unwrap();
    //
    //
    // let cell_lower_upper = setCategoryAlphabet.get_cell(&cell_id).unwrap();
    // assert_eq!(cell_lower_upper.source_object_id(),  NCategory::category_id(&discreteCategoryALower));
    // assert_eq!(cell_lower_upper.target_object_id(),  NCategory::category_id(&discreteCategoryAUpper));
    //
    // let cell_lower_upper_reverse = setCategoryAlphabet.get_cell(&cell_id_lower_upper_reverse).unwrap();
    // assert_eq!(cell_lower_upper_reverse.source_object_id(),  NCategory::category_id(&discreteCategoryAUpper));
    // assert_eq!(cell_lower_upper_reverse.target_object_id(),  NCategory::category_id(&discreteCategoryALower));
    //
    // // now get cell tree of the cell
    // let actual_cell_tree = setCategoryAlphabet.get_cell_tree(cell_lower_upper).unwrap();
    //
    // let functor_1 = "functor_1".to_string();
    // let functor_2 = "functor_2".to_string();
    // let expected_cell_tree = CellTree::new_with_children(
    //     cell_lower_upper.id(),
    //     cell_lower_upper.source_object_id(),
    //     cell_lower_upper.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &functor_1,
    //             &object_a,
    //             &object_A
    //         ),
    //         CellTree::new(
    //             &functor_1,
    //             &object_b,
    //             &object_B
    //         ),
    //         CellTree::new(
    //             &functor_1,
    //             &object_c,
    //             &object_C
    //         )
    //     ]
    // );
    //
    // assert_eq!(actual_cell_tree, expected_cell_tree);
    //
    // // now get cell tree of the reverse cell
    // let actual_cell_tree_reverse = setCategoryAlphabet.get_cell_tree(cell_lower_upper_reverse).unwrap();
    //
    // let expected_cell_tree_reverse = CellTree::new_with_children(
    //     cell_lower_upper_reverse.id(),
    //     cell_lower_upper_reverse.source_object_id(),
    //     cell_lower_upper_reverse.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &functor_2,
    //             &object_a,
    //             &object_C
    //         ),
    //         CellTree::new(
    //             &functor_2,
    //             &object_b,
    //             &object_B
    //         ),
    //         CellTree::new(
    //             &functor_2,
    //             &object_c,
    //             &object_A
    //         )
    //     ]
    // );
}
