use crate::core::arrow::{Arrow, Morphism};
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use async_trait::async_trait;
use dyn_clone::DynClone;
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

pub type CategorySubObjectAlias<Category> = <Category as CategoryTrait>::Object;
pub enum MorphismCommutationResult<Category: CategoryTrait> {
    Commutative,
    NonCommutative(HashSet<Arc<Category::Morphism>>),
}

#[async_trait]
pub trait CategoryTrait: Debug + Any + DynClone + Send + Sync {
    type Object: CategoryTrait + Debug + Eq + Hash + DynClone + Send + Sync;

    type Morphism: ArrowTrait<Self::Object, Self::Object> + Debug + Send + Sync;

    fn new() -> Self
    where
        Self: Sized;

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized;

    fn level(&self) -> usize
    where
        Self: Sized,
    {
        Self::nested_level()
    }

    fn new_instance(&self) -> Self
    where
        Self: Sized,
    {
        Self::new()
    }

    fn new_instance_with_id(&self, id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        Self::new_with_id(id)
    }

    fn category_id(&self) -> &ObjectId;

    /*
    This should be used very carefully, as changing the category ID might lead to inconsistencies
    it should only be used in scenarios of creating a new category based on an existing
     */
    fn update_category_id(&mut self, new_id: ObjectId);
    fn update_category_id_generate(&mut self) {
        self.update_category_id(ObjectId::generate())
    }

    fn equal_to(&self, other: &Self::Object) -> bool {
        self.category_id() == other.category_id()
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors>;

    async fn add_morphism(
        &mut self,
        morphism: Arc<Self::Morphism>,
    ) -> Result<&Arc<Self::Morphism>, Errors>;

    async fn get_identity_morphism(
        &self,
        object: &Self::Object,
    ) -> Result<&Arc<Self::Morphism>, Errors> {
        let hom_set = self.get_hom_set(object, object).await?;
        // get the identity morphism
        for morphism in hom_set {
            if morphism.is_identity() {
                return Ok(morphism);
            }
        }
        Err(Errors::IdentityMorphismNotFound)
    }

    async fn get_all_identity_morphisms(&self) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        let mut identity_morphisms = HashSet::new();
        for object in self.get_all_objects().await? {
            match self.get_identity_morphism(&**object).await {
                Ok(identity_morphism) => {
                    identity_morphisms.insert(identity_morphism);
                }
                Err(_) => continue,
            }
        }
        Ok(identity_morphisms)
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors>;

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors>;

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Self::Morphism>>, Errors>;

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Self::Morphism>>, Errors>;

    async fn get_hom_set(
        &self,
        source_object: &Self::Object,
        target_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        Ok(self
            .get_hom_set_x(source_object)
            .await?
            .iter()
            .filter(|item| &**item.target_object() == target_object)
            .copied()
            .collect::<HashSet<_>>())
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Self::Morphism>>, Errors>;

    async fn morphism_commute(
        &self,
        left_morphisms: Vec<&Self::Morphism>,
        right_morphisms: Vec<&Self::Morphism>,
    ) -> Result<MorphismCommutationResult<Self::Object>, Errors> {
        self.validate_morphisms_commutation(left_morphisms, right_morphisms)
            .await?;
        Ok(MorphismCommutationResult::Commutative)
    }

    async fn validate_morphisms_commutation(
        &self,
        left_morphisms: Vec<&Self::Morphism>,
        right_morphisms: Vec<&Self::Morphism>,
    ) -> Result<(), Errors> {
        // // source and target of left cells id should be same with right cells
        // let left_source_object = left_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .source_object();
        // let right_source_object = right_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .source_object();
        //
        // if left_source_object != right_source_object {
        //     return Err(NCategoryError::InvalidMorphismComposition);
        // }
        //
        // let left_target_object = left_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .target_object();
        // let right_target_object = right_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .target_object();
        //
        // if left_target_object != right_target_object {
        //     return Err(NCategoryError::InvalidMorphismComposition);
        // }
        //
        // // confirm composition is correct
        // self.validate_morphisms_composition(left_morphisms)?;
        // self.validate_morphisms_composition(right_morphisms)?;
        //
        Ok(())
        // todo!()
    }

    async fn validate_morphisms_composition(
        &self,
        morphims: Vec<&Self::Morphism>,
    ) -> Result<(), Errors> {
        // if morphims.is_empty() {
        //     return Err(NCategoryError::InvalidMorphismComposition);
        // }
        //
        // // composition of only once cell is always valid
        // if morphims.len() <= 1 {
        //     return Ok(());
        // }
        // // target of first cell needs to be the source of subsequent cell
        // let mut target_object = morphims
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismComposition)?
        //     .target_object();
        //
        // for morphism in &morphims[1..] {
        //     if morphism.source_object() != target_object {
        //         return Err(NCategoryError::InvalidMorphismComposition);
        //     }
        //     target_object = morphism.target_object();
        // }
        // Ok(())
        todo!()
    }

    fn is_zero_category(&self) -> bool {
        false
    }

    fn nested_level() -> usize
    where
        Self: Sized,
    {
        todo!()
        // 1 + <Self::Object as CategoryTrait>::nested_level()
    }
}

#[async_trait]
pub trait CategoryFromObjects: CategoryTrait {
    async fn from_objects<T>(objects: Vec<T>) -> Result<Self, Errors>
    where
        T: Into<Self::Object> + Send,
        Self: Sized;
}

#[async_trait]
impl<C> CategoryFromObjects for C
where
    C: CategoryTrait + Sized,
{
    async fn from_objects<T>(objects: Vec<T>) -> Result<Self, Errors>
    where
        T: Into<C::Object> + Send,
    {
        let mut category = Self::new();
        for object in objects {
            category.add_object(Arc::new(object.into())).await?;
        }
        Ok(category)
    }
}
