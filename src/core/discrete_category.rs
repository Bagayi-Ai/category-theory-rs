use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::nfunctor::NFunctor;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::unit::unit_functor::UnitFunctor;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiscreteCategory<T: Identifier> {
    category_id: T,
    // TODO: Find a way of avoiding storing identity cells
    // as we could derive them from the objects.
    cells: Option<HashMap<T, UnitCategory<T>>>,
}

impl<T: Identifier + Display> Display for DiscreteCategory<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.category_id)
    }
}

impl<T: Identifier> Hash for DiscreteCategory<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category_id.hash(state);
        if let Some(cells) = &self.cells {
            for key in cells.keys() {
                key.hash(state);
            }
        }
    }
}

impl<T: Eq + Clone + Debug + Hash + Identifier> DiscreteCategory<T> {
    pub fn new() -> Self {
        DiscreteCategory {
            category_id: T::generate(),
            cells: Some(HashMap::new()),
        }
    }

    pub fn new_with_id(category_id: T) -> Self {
        DiscreteCategory {
            category_id,
            cells: None,
        }
    }
}

impl<'a, T: Eq + Clone + Hash + Debug + Identifier + ToString + 'a + Display> NCategory<'a>
    for DiscreteCategory<T>
{
    type Identifier = T;
    type Object = UnitCategory<T>;
    type Morphism = UnitCategory<T>;

    fn category_id(&self) -> Self::Identifier {
        self.category_id.clone()
    }

    fn add_object(&mut self, object: &Self::Object) -> Result<(), NCategoryError> {
        self.add_morphism(object.clone())?;
        Ok(())
    }

    fn add_morphism(
        &mut self,
        morphism: Self::Morphism,
    ) -> Result<Self::Identifier, NCategoryError> {
        // morphsims in discrete category are only identity morphisms,
        if morphism.source_object() != morphism.target_object() {
            return Err(NCategoryError::OnlyIdentityMorphismDiscreteCategory);
        }

        let cell_id = morphism.cell_id().clone();

        if let Some(cells) = &mut self.cells {
            if cells.contains_key(&cell_id) {
                return Err(NCategoryError::MorphismAlreadyExists);
            }
            cells.insert(cell_id.clone(), morphism);
        } else {
            self.cells = Some(HashMap::new());
            self.cells
                .as_mut()
                .unwrap()
                .insert(cell_id.clone(), morphism);
        }

        Ok(cell_id)
    }

    fn get_identity_morphism(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Morphism, NCategoryError> {
        self.get_moprhism(&object_id)
    }

    fn get_all_object_ids(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        // if let Some(cells) = &self.cells {
        //     Ok(cells.values().collect())
        // } else {
        //     Err(NCategoryError::NoObjectsInCategory)
        // }
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError> {
        self.get_all_object_ids()?
            .into_iter()
            .map(|object_id| self.get_identity_morphism(object_id))
            .collect()
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Morphism>, NCategoryError> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_morphism(&object_id)?])
    }

    fn get_moprhism(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(cell_id) {
                return Ok(cell);
            }
        }
        Err(NCategoryError::MorphismNotFound)
    }
    fn nested_level() -> usize {
        1
    }
}

impl<'a, T: Eq + Clone + Hash + Debug + Identifier + 'a + Display> Morphism<'a>
    for DiscreteCategory<T>
{
    type Identifier = T;
    type Object = UnitCategory<T>;
    type Functor = UnitFunctor<'a, T, UnitCategory<T>, UnitCategory<T>>;

    fn cell_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object(&self) -> &Self::Object {
        // &self
        todo!()
    }

    fn target_object(&self) -> &Self::Object {
        // &self
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> From<T> for DiscreteCategory<T> {
    fn from(object: T) -> Self {
        let mut category = DiscreteCategory::new();
        let object = UnitCategory {
            category_id: object,
        };
        category.add_object(&object).unwrap();
        category
    }
}

impl<T: Eq + Clone + Hash + Debug + Identifier + Display> From<Vec<T>> for DiscreteCategory<T> {
    fn from(objects: Vec<T>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            let object = UnitCategory {
                category_id: object,
            };
            category.add_object(&object).unwrap();
        }
        category
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;

    fn generate_object() -> UnitCategory<String> {
        UnitCategory {
            category_id: random_string(5),
        }
    }

    fn generate_identifier() -> String {
        String::generate()
    }

    #[test]
    pub fn test_base_scenarios() {
        let mut category = DiscreteCategory::new();
        // add object 1
        let object1 = generate_object();

        category.add_object(&object1.clone()).unwrap();
        // check identity morphism
        let cell = category.get_object_morphisms(&object1.category_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object(), &object1);
        assert_eq!(cell.target_object(), &object1);

        // check identity morphism
        let cell = category.get_object_morphisms(&object1.category_id);
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert_eq!(cell.source_object(), &object1);
        assert_eq!(cell.target_object(), &object1);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // add object 2
        let object2 = generate_object();
        assert!(category.add_object(&object2.clone()).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&object2.category_id);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object(), &object2);
        assert_eq!(cell.target_object(), &object2);

        // add object 3 without id
        let object3 = generate_object();
        assert!(category.add_object(&object3.clone()).is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&object3.category_id);
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert_eq!(cell.source_object(), &object3);
        assert_eq!(cell.target_object(), &object3);
    }
}
