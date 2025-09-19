use crate::core::arrow::Morphism;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiscreteCategory {
    category_id: ObjectId,
    // TODO: Find a way of avoiding storing identity cells
    // as we could derive them from the objects.
    cells: Option<HashMap<ObjectId, Arc<Morphism<Self>>>>,
}

impl Hash for DiscreteCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category_id.hash(state);
        if let Some(cells) = &self.cells {
            for key in cells.keys() {
                key.hash(state);
            }
        }
    }
}

impl Default for DiscreteCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscreteCategory {
    pub fn new() -> Self {
        DiscreteCategory {
            category_id: ObjectId::Str(String::generate()),
            cells: Some(HashMap::new()),
        }
    }

    pub fn new_with_id(category_id: ObjectId) -> Self {
        DiscreteCategory {
            category_id,
            cells: None,
        }
    }

    pub fn clone_with_new_id(&self) -> Self {
        Self {
            category_id: ObjectId::Str(String::generate()),
            cells: self.cells.clone(),
        }
    }
    pub fn category_id(&self) -> &ObjectId {
        &self.category_id
    }
}

#[async_trait]
impl CategoryTrait for DiscreteCategory {
    type Object = Self;

    type Morphism = Morphism<Self::Object>;

    async fn new() -> Result<Self, Errors> {
        Ok(DiscreteCategory::new())
    }

    fn category_id(&self) -> &ObjectId {
        &self.category_id
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        self.category_id = new_id;
        Ok(())
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        let identity_morphism = Morphism::new_identity(object.clone());
        if let Some(cells) = &mut self.cells {
            if cells.contains_key(&object.category_id()) {
                return Err(Errors::ObjectAlreadyExists);
            }
            cells.insert(object.category_id().clone(), identity_morphism.clone());
        } else {
            self.cells = Some(HashMap::new());
            self.cells
                .as_mut()
                .unwrap()
                .insert(object.category_id().clone(), identity_morphism.clone());
        }
        Ok(identity_morphism)
    }

    async fn add_morphism(&mut self, morphism: Arc<Morphism<Self::Object>>) -> Result<(), Errors> {
        Err(Errors::CannotAddMorphismToDiscreteCategory)
    }

    async fn get_identity_morphism(
        &self,
        object: &Self::Object,
    ) -> Result<&Arc<Morphism<Self::Object>>, Errors> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(&object.category_id()) {
                return Ok(cell);
            }
        }
        Err(Errors::ObjectNotFound)
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(&object.category_id()) {
                return Ok(cell.source_object());
            }
        }
        Err(Errors::ObjectNotFound)
    }

    // fn get_all_objects(&self) -> Result<HashSet<&Arc<DiscreteCategory>>, Errors> {
    //     let result = if let Some(cells) = &self.cells {
    //         cells.values().map(|item| item.source_object()).collect()
    //     } else {
    //         HashSet::new()
    //     };
    //     Ok(result)
    // }
    // Rust
    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        let result = if let Some(cells) = &self.cells {
            cells.values().map(|item| item.source_object()).collect()
        } else {
            HashSet::new()
        };
        Ok(result)
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors> {
        let result = if let Some(cells) = &self.cells {
            cells.values().collect()
        } else {
            HashSet::new()
        };
        Ok(result)
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors> {
        // only one morphism in discrete category, the identity morphism.
        Ok(HashSet::from([self
            .get_identity_morphism(source_object)
            .await?]))
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Morphism<Self::Object>>>, Errors> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_morphism(object).await?])
    }

    fn nested_level() -> usize {
        1
    }
}

impl From<ObjectId> for DiscreteCategory {
    fn from(object: ObjectId) -> Self {
        DiscreteCategory::new_with_id(object)
    }
}

impl From<i32> for DiscreteCategory {
    fn from(object: i32) -> Self {
        DiscreteCategory::new_with_id(ObjectId::Int(object))
    }
}

impl From<&str> for DiscreteCategory {
    fn from(object: &str) -> Self {
        DiscreteCategory::new_with_id(object.into())
    }
}

impl From<String> for DiscreteCategory {
    fn from(object: String) -> Self {
        DiscreteCategory::new_with_id(object.into())
    }
}

impl From<Arc<DiscreteCategory>> for DiscreteCategory {
    fn from(rc: Arc<DiscreteCategory>) -> Self {
        (*rc).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;

    fn generate_morphism() -> Arc<DiscreteCategory> {
        Arc::new(DiscreteCategory::new_with_id(ObjectId::Str(random_string(
            5,
        ))))
    }

    fn generate_identifier() -> String {
        String::generate()
    }

    #[tokio::test]
    pub async fn test_base_scenarios() {
        let mut category = DiscreteCategory::new();
        // add object 1
        let object1 = generate_morphism();

        category.add_object(object1.clone()).await.unwrap();
        // check identity morphism
        let cell = category.get_object_morphisms(&*object1).await;
        assert!(cell.clone().is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert!(cell.source_object().equal_to(&*object1));
        assert!(cell.target_object().equal_to(&*object1));

        // check identity morphism
        let cell = category.get_object_morphisms(&*object1).await;
        assert!(cell.is_ok());
        let cell = cell.unwrap();
        assert_eq!(cell.len(), 1);
        let cell = cell.first().unwrap();
        assert!(cell.source_object().equal_to(&*object1));
        assert!(cell.target_object().equal_to(&*object1));

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // add object 2
        let object2 = generate_morphism();
        assert!(category.add_object(object2.clone()).await.is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&*object2).await;
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert!(cell.source_object().equal_to(&*object2));
        assert!(cell.target_object().equal_to(&*object2));

        // add object 3 without id
        let object3 = generate_morphism();
        assert!(category.add_object(object3.clone()).await.is_ok());

        // check identity morphism
        let cells = category.get_object_morphisms(&*object3).await;
        assert!(cells.is_ok());
        let cells = cells.unwrap();
        assert_eq!(cells.len(), 1);
        let cell = cells.first().unwrap();
        assert!(cell.source_object().equal_to(&*object3));
        assert!(cell.target_object().equal_to(&*object3));
    }
}
