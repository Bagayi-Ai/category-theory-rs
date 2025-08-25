use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub type MorphismAlias<Category> = <Category as CategoryTrait>::Morphism;

pub enum MorphismCommutationResult<Morphism: MorphismTrait> {
    Commutative,
    NonCommutative(HashSet<Morphism>),
}

pub trait CategoryTrait: Debug {
    type Object: CategoryTrait + Debug + Eq + Hash;
    type Morphism: MorphismTrait<SourceObject = Self::Object, TargetObject = Self::Object>
        + Debug
        + Eq
        + Hash;

    fn new() -> Self
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

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors>;

    fn add_morphism(&mut self, morphism: Rc<Self::Morphism>)
    -> Result<&Rc<Self::Morphism>, Errors>;

    fn get_identity_morphism(&self, object: &Self::Object) -> Result<&Rc<Self::Morphism>, Errors> {
        let hom_set = self.get_hom_set(object, object)?;
        // get the identity morphism
        for morphism in hom_set {
            if morphism.is_identity() {
                return Ok(morphism);
            }
        }
        Err(Errors::IdentityMorphismNotFound)
    }

    fn get_identity_morphism_reference(&self, object: Self::Object) -> Result<&Rc<Self::Morphism>, Errors> {
        self.get_identity_morphism(&object)
    }

    fn get_all_identity_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        let mut identity_morphisms = HashSet::new();
        for object in self.get_all_objects()? {
            match self.get_identity_morphism(object) {
                Ok(identity_morphism) => {
                    identity_morphisms.insert(identity_morphism);
                }
                Err(_) => continue,
            }
        }
        Ok(identity_morphisms)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors>;

    fn get_object_reference(&self, object: Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.get_object(&object)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors>;

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors>;

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors>;

    fn get_hom_set(
        &self,
        source_object: &Self::Object,
        target_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        Ok(self
            .get_hom_set_x(source_object)?
            .iter()
            .filter(|item| &**item.target_object() == target_object)
            .map(|item| *item)
            .collect::<HashSet<_>>())
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors>;

    fn morphism_commute(
        &self,
        left_morphisms: Vec<&Self::Morphism>,
        right_morphisms: Vec<&Self::Morphism>,
    ) -> Result<MorphismCommutationResult<MorphismAlias<Self::Object>>, Errors> {
        self.validate_morphisms_commutation(left_morphisms, right_morphisms)?;
        Ok(MorphismCommutationResult::Commutative)
    }

    fn validate_morphisms_commutation(
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

    fn validate_morphisms_composition(&self, morphims: Vec<&Self::Morphism>) -> Result<(), Errors> {
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
        1 + <Self::Object as CategoryTrait>::nested_level()
    }
}
