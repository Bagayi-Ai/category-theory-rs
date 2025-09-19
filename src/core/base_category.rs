use crate::core::arrow::Morphism;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use async_trait::async_trait;
use dyn_clone::DynClone;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct BaseCategory<Object: CategoryTrait> {
    id: ObjectId,
    objects: HashMap<ObjectId, Arc<Object>>,
    object_mappings: HashMap<ObjectId, HashSet<String>>,
    morphism: HashMap<String, Arc<Morphism<Object>>>,
}

impl<Object: CategoryTrait> Debug for BaseCategory<Object> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Object: CategoryTrait + Clone> Hash for BaseCategory<Object> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl<Object: CategoryTrait + Clone> Default for BaseCategory<Object> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Object: CategoryTrait> BaseCategory<Object> {
    pub fn new() -> Self {
        Self::new_with_id(ObjectId::generate())
    }

    pub fn new_with_id(id: ObjectId) -> Self {
        BaseCategory {
            id,
            objects: HashMap::new(),
            object_mappings: HashMap::new(),
            morphism: HashMap::new(),
        }
    }

    pub fn id(&self) -> &ObjectId {
        &self.id
    }
}

#[async_trait]
impl<Object: CategoryTrait + Hash + Eq + DynClone + Clone> CategoryTrait for BaseCategory<Object> {
    type Object = Object;

    type Morphism = Morphism<Self::Object>;

    async fn new() -> Result<Self, Errors> {
        Ok(BaseCategory::new())
    }

    fn category_id(&self) -> &ObjectId {
        &self.id
    }

    async fn update_category_id(&mut self, new_id: ObjectId) -> Result<(), Errors> {
        self.id = new_id;
        Ok(())
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        if self.objects.contains_key(object.category_id()) {
            return Err(Errors::ObjectAlreadyExists);
        }
        let object_id = object.category_id();
        self.objects.insert(object_id.clone(), object.clone());
        let identity_cell = Morphism::new_identity(object.clone());
        self.object_mappings
            .entry(object_id.clone())
            .or_default()
            .insert(identity_cell.arrow_id().to_string());
        self.add_morphism(identity_cell.clone()).await?;
        Ok(identity_cell)
    }

    async fn add_morphism(&mut self, morphism: Arc<Morphism<Self::Object>>) -> Result<(), Errors> {
        if self.morphism.contains_key(morphism.arrow_id()) {
            return Err(Errors::MorphismAlreadyExists);
        }
        // validate target object is part of the category
        if !self
            .objects
            .contains_key(&morphism.target_object().category_id())
        {
            return Err(Errors::ObjectNotFound);
        }

        // if its not identity morphism add it to the objects as part of the hom-set
        if !morphism.is_identity() {
            self.object_mappings
                .get_mut(&morphism.source_object().category_id())
                .ok_or(Errors::ObjectNotFound)?
                .insert(morphism.arrow_id().to_string());
        }

        let cell = self
            .morphism
            .entry(morphism.arrow_id().clone())
            .or_insert(morphism);
        Ok(())
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        self.objects
            .get(object.category_id())
            .ok_or(Errors::ObjectNotFound)
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        Ok(self.objects.values().collect())
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let result: HashSet<&Arc<Morphism<Self::Object>>> = HashSet::new();
        // for (_id, cell) in &self.cells {
        //     result.insert(cell);
        // }
        Ok(result)
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors> {
        let result = self
            .object_mappings
            .get(source_object.category_id().into())
            .ok_or(Errors::ObjectNotFound)?
            .iter()
            .map(|item| self.morphism.get(item).ok_or(Errors::MorphismNotFound))
            .collect::<Result<HashSet<&Arc<Morphism<Self::Object>>>, Errors>>()?;
        Ok(result)
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Morphism<Self::Object>>>, Errors> {
        let result = self
            .object_mappings
            .get(object.category_id().into())
            .ok_or(Errors::ObjectNotFound)?
            .iter()
            .map(|item| self.morphism.get(item).ok_or(Errors::MorphismNotFound))
            .collect::<Result<Vec<&Arc<Morphism<Self::Object>>>, Errors>>()?;
        Ok(result)
    }
}
