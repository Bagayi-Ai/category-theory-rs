use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category<Id: Identifier<Id = Id>, Object: CategoryTrait<Identifier = Id>> {
    id: Id,
    objects: HashMap<Id, Rc<Object>>,
    object_mapping: HashMap<Id, HashMap<Id, HashSet<Id>>>,
    morphism: HashMap<Id, Rc<Morphism<Id, Self>>>,
}

impl<'a, Id: Identifier<Id = Id>, Object: CategoryTrait<Identifier = Id>> Category<Id, Object> {
    pub fn new() -> Self {
        Self::new_with_id(Id::generate())
    }

    pub fn new_with_id(id: Id) -> Self {
        Category {
            id,
            objects: HashMap::new(),
            object_mapping: HashMap::new(),
            morphism: HashMap::new(),
        }
    }
}

impl<Id: Identifier<Id = Id>, Object: CategoryTrait<Identifier = Id>> CategoryTrait
    for Category<Id, Object>
{
    type Identifier = Id;
    type Object = Object;
    type Morphism = Morphism<Id, Self>;

    fn new() -> Self {
        Category::new()
    }

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        self.objects
            .insert(object.category_id().clone(), object.clone());
        let identity_cell = Morphism::new_identity_morphism(object);
        self.add_morphism(identity_cell)?;
        Ok(())
    }

    fn add_morphism(&mut self, cell: Rc<Self::Morphism>) -> Result<&Rc<Self::Morphism>, Errors> {
        if self.morphism.contains_key(cell.id()) {
            return Err(Errors::MorphismAlreadyExists);
        }
        let cell = self.morphism.entry(cell.id().clone()).or_insert(cell);
        self.object_mapping
            .entry(cell.source_object().category_id().clone())
            .or_default()
            .entry(cell.target_object().category_id().clone())
            .or_default()
            .insert(cell.arrow_id().clone());
        Ok(cell)
    }

    fn get_object(&self, object: Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.objects
            .get(object.category_id())
            .ok_or(Errors::ObjectNotFound)
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let result: HashSet<&Rc<Self::Morphism>> = HashSet::new();
        // for (_id, cell) in &self.cells {
        //     result.insert(cell);
        // }
        Ok(result)
    }

    fn get_hom_set(
        &self,
        source_object: &Self::Object,
        target_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        let mut result: HashSet<&Rc<Self::Morphism>> = HashSet::new();
        if let Some(cells) = self.object_mapping.get(source_object.category_id()) {
            if let Some(cell_set) = cells.get(target_object.category_id()) {
                for cell_id in cell_set {
                    if let Some(cell) = self.morphism.get(cell_id) {
                        result.insert(cell);
                    }
                }
            }
        } else {
            return Err(Errors::ObjectNotFound);
        }
        Ok(result)
    }

    fn get_object_morphisms(&self, object: &Self::Object) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        if let Some(cells) = self.object_mapping.get(object.category_id()) {
            let mut result: Vec<&Rc<Self::Morphism>> = Vec::new();
            for cell_set in cells.values() {
                for cell_id in cell_set {
                    if let Some(cell) = self.morphism.get(cell_id) {
                        if cell.source_object().category_id() == object.category_id() {
                            result.push(cell);
                        }
                    }
                }
            }
            Ok(result)
        } else {
            Err(Errors::ObjectNotFound)
        }
    }
}

impl<Id: Identifier<Id = Id>, Object: CategoryTrait<Identifier = Id>> Hash
    for Category<Id, Object>
{
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        todo!()
    }
}
