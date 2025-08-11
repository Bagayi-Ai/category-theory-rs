/*
Endo functor maps objects and morphisms of a category to itself.
This is a specific case of a functor where the source and target categories are the same.

For our category framework since we always have a definite set of objects and morphisms

Endo functor will create a new category with the objects and morphisms of the original category.
and the new objects and morphisms mapped to the new values.

For instance if category A has objects {a, b} and morphisms {f: a -> b, g: b -> a},
an endo functor F would map these to a new category B with objects {F(a), F(b)} and morphisms {F(f): F(a) -> F(b), F(g): F(b) -> F(a)}.
This is useful for creating new categories that are derived from the original category, such as the category of sets or the category of groups.
 */
use crate::core::errors::Errors;
use crate::core::functor::Functor;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::HashMap;

pub struct ProductEndoFunctor<'a, Id, Category>
where
    Id: Identifier,
    Category: CategoryTrait<'a>,
{
    id: Id,
    source_category: &'a Category,
}

impl<'a, Id, Category> ProductEndoFunctor<'a, Id, Category>
where
    Id: Identifier,
    Category: CategoryTrait<'a> + Clone,
{
    pub fn apply_endofunctor(
        id: Id,
        source_category: &Category,
        fixed_category: &Category,
        target_category: &mut Category,
    ) -> Result<Functor<'a, Id, Category, Category>, Errors> {
        todo!()
    }
}
