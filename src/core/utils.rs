use crate::core::traits::category_trait::CategoryTrait;
use std::hash::Hash;
use std::rc::Rc;

// pub fn contains_object<Category: CategoryTrait + Eq + Hash>(
//     objects: &Vec<&Rc<Category::Object>>,
//     target: &Category::Object,
// ) -> bool {
//     objects.iter().any(|obj| &***obj == target)
// }

pub fn find_object_index<Category: CategoryTrait + Eq + Hash>(
    objects: &Vec<&Rc<Category>>,
    target: &Category,
) -> isize {
    objects
        .iter()
        .position(|obj| &***obj == target)
        .map(|idx| idx as isize)
        .unwrap_or(-1)
}
