use crate::core::errors::Errors;
use crate::core::errors::Errors::{InvalidArrowNoFunctorFound, NoFunctorForIdentityArrow};
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{
    CategoryIdentifierAlias, CategoryObjectAlias, CategoryTrait,
};
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use std::hash::{Hash, Hasher};

pub struct Morphism<'a, Id, Category>
where
    Id: Identifier,
    Category: CategoryTrait<'a>,
{
    id: Id,
    source: &'a CategoryObjectAlias<'a, Category>,
    target: &'a CategoryObjectAlias<'a, Category>,
    functor: Option<
        &'a Functor<'a, Id, CategoryObjectAlias<'a, Category>, CategoryObjectAlias<'a, Category>>,
    >,
    identity: bool,
}

impl<'a, Id: Identifier, Category: CategoryTrait<'a, Identifier = Id>> Morphism<'a, Id, Category>
where
    <Category as CategoryTrait<'a>>::Object: CategoryTrait<'a, Identifier = Id>,
{
    pub fn new(
        id: Id,
        source: &'a CategoryObjectAlias<'a, Category>,
        target: &'a CategoryObjectAlias<'a, Category>,
        functor: &'a Functor<
            'a,
            Id,
            CategoryObjectAlias<'a, Category>,
            CategoryObjectAlias<'a, Category>,
        >,
    ) -> Self {
        Morphism {
            id,
            source,
            target,
            functor: functor.into(),
            identity: false,
        }
    }

    pub fn new_identity_morphism(object: &'a CategoryObjectAlias<'a, Category>) -> Self {
        Morphism {
            id: object.category_id().clone(),
            source: object,
            target: object,
            functor: None,
            identity: true,
        }
    }

    pub fn id(&self) -> &CategoryIdentifierAlias<'a, Category> {
        &self.id
    }
}

impl<'a, Id: Identifier, Category: CategoryTrait<'a>> Hash for Morphism<'a, Id, Category> {
    fn hash<H>(&self, _: &mut H)
    where
        H: Hasher,
    {
        todo!()
    }
}

impl<'a, Id: Identifier, Category: CategoryTrait<'a>> ArrowTrait<'a>
    for Morphism<'a, Id, Category>
{
    type SourceObject = CategoryObjectAlias<'a, Category>;
    type TargetObject = CategoryObjectAlias<'a, Category>;
    type Identifier = Id;

    fn arrow_id(&self) -> &Self::Identifier {
        &self.id
    }

    fn source_object(&self) -> &Self::SourceObject {
        self.source
    }

    fn target_object(&self) -> &Self::TargetObject {
        self.target
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn arrows(&self) -> Vec<&Morphism<'a, Id, Category>> {
        todo!()
    }
}

impl<'a, Id: Identifier, Category: CategoryTrait<'a, Identifier = Id>> MorphismTrait<'a>
    for Morphism<'a, Id, Category>
{
    fn functor(
        &self,
    ) -> Result<&Functor<'a, Self::Identifier, Self::SourceObject, Self::TargetObject>, Errors>
    {
        if self.identity {
            return Err(Errors::NoFunctorForIdentityArrow);
        }
        self.functor.ok_or(InvalidArrowNoFunctorFound)
    }
}
