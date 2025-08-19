use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::category_trait::MorphismAlias;
use std::collections::HashMap;
use std::rc::Rc;

pub trait FunctorTrait: ArrowTrait {
    fn validate_mappings(&self) -> Result<(), Errors> {
        todo!()
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<Rc<MorphismAlias<Self::SourceObject>>, Rc<MorphismAlias<Self::TargetObject>>>;
}
