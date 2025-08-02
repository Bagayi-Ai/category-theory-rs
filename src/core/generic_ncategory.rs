use crate::core::generic_morphism::GenericMorphism;
use crate::core::generic_nfunctor::GenericNFunctor;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::nfunctor::NFunctor;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct GenericNCategory<
    'a,
    Id: Identifier<Id = Id>,
    BaseCategory: NCategory<'a, Identifier = Id> + Debug + Eq,
> {
    id: Id,
    objects: HashMap<Id, &'a BaseCategory>,
    object_mapping: HashMap<Id, HashMap<Id, HashSet<Id>>>,
    cells: HashMap<Id, GenericMorphism<'a, Self>>,
}

impl<'a, Id: Identifier<Id = Id>, Category: NCategory<'a, Identifier = Id> + Debug + Eq>
    GenericNCategory<'a, Id, Category>
{
    pub fn new() -> Self {
        GenericNCategory {
            id: Id::generate(),
            objects: HashMap::new(),
            object_mapping: HashMap::new(),
            cells: HashMap::new(),
        }
    }
}

impl<'a, Id: Identifier<Id = Id>, BaseCategory: NCategory<'a, Identifier = Id> + 'a + Debug + Eq>
    NCategory<'a> for GenericNCategory<'a, Id, BaseCategory>
{
    type Identifier = Id;
    type Object = &'a BaseCategory;
    type Morphism = GenericMorphism<'a, Self>;
    type BaseCategory = BaseCategory;

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Self::Object) -> Result<(), NCategoryError> {
        self.objects.insert(object.category_id().clone(), object);
        let identity_cell = GenericMorphism::new(
            object.category_id().clone(),
            object,
            object,
            "identity".to_string(),
        );
        self.add_moprhism(identity_cell)?;
        Ok(())
    }

    fn add_moprhism(&mut self, cell: Self::Morphism) -> Result<Self::Identifier, NCategoryError> {
        if self.cells.contains_key(&cell.id()) {
            return Err(NCategoryError::MorphismAlreadyExists);
        }
        let cell = self.cells.entry(cell.id().clone()).or_insert(cell);
        self.object_mapping
            .entry(cell.source_object().category_id().clone())
            .or_default()
            .entry(cell.target_object().category_id().clone())
            .or_default()
            .insert(cell.cell_id().clone());
        let cell_id = cell.id().clone();
        Ok(cell_id)
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError> {
        if let Some(object) = self.objects.get(object_id) {
            Ok(object)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_identity_morphism(
        &self,
        object_id: Self::Object,
    ) -> Result<&Self::Morphism, NCategoryError> {
        // it's basically the cell with the same id as the object
        self.get_moprhism(object_id.category_id())
    }

    fn get_all_objects(&self) -> Result<HashSet<Self::Object>, NCategoryError> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let mut result: HashSet<&Self::Morphism> = HashSet::new();
        // for (_id, cell) in &self.cells {
        //     result.insert(cell);
        // }
        Ok(result)
    }

    fn get_object_morphisms(
        &self,
        object: Self::Object,
    ) -> Result<Vec<&Self::Morphism>, NCategoryError> {
        if let Some(cells) = self.object_mapping.get(object.category_id()) {
            let mut result: Vec<&Self::Morphism> = Vec::new();
            for (_to, cell_set) in cells {
                for cell_id in cell_set {
                    if let Some(cell) = self.cells.get(cell_id) {
                        if cell.source_object() == object {
                            result.push(&cell);
                        }
                    }
                }
            }
            Ok(result)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_moprhism(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError> {
        if let Some(cell) = self.cells.get(cell_id) {
            Ok(cell)
        } else {
            Err(NCategoryError::MorphismNotFound)
        }
    }

    fn base_object(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}
