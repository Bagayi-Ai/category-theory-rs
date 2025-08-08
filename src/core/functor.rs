use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use crate::core::arrow::Arrow;
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]


pub type Functor<'a, Id, SourceObject, TargetObject> =
Arrow<'a, Id, SourceObject, TargetObject>;