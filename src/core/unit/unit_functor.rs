// use std::marker::PhantomData;
// use crate::core::nfunctor::{NFunctor};
// use crate::core::functor_mapping::FunctorMappings;
// use crate::core::identifier::Identifier;
// use crate::core::ncategory::{NCategory, NCategoryError};
// use crate::core::unit::unit_identifier::UnitIdentifier;
// use crate::core::unit::unit_category::UnitCategory;
//
// macro_rules! create_unit_functor_instance {
//     ($name:ident, $id_type:ty) => {
//         pub const $name: UnitFunctor<$id_type> = UnitFunctor {
//             phantom_data: PhantomData,
//         };
//     };
// }
//
// create_unit_functor_instance!(UNIT_FUNCTOR_STRING, String);
// create_unit_functor_instance!(UNIT_FUNCTOR_UNIT_IDENTIFIER, UnitIdentifier);
//
// macro_rules! match_unit_functor_instance {
//     ($id_type:ty, $($name:ident => $type:ty),*) => {
//         {
//             let type_id = std::any::TypeId::of::<$id_type>();
//             $(
//                 if type_id == std::any::TypeId::of::<$type>() {
//                     return Some(unsafe { &*(&$name as *const UnitFunctor<$type> as *const UnitFunctor<$id_type>) });
//                 }
//             )*
//             None
//         }
//     };
// }
//
// pub fn get_unit_functor_instance<'a, Id: Identifier + 'static>() -> Option<&'a UnitFunctor<Id>> {
//     match_unit_functor_instance!(Id,
//         UNIT_FUNCTOR_STRING => String,
//         UNIT_FUNCTOR_UNIT_IDENTIFIER => UnitIdentifier
//     )
// }
//
// // Helper function to cast to the expected type
// // pub fn get_unit_functor_instance<'a, Id: Identifier>() -> &'a UnitFunctor<Id> {
// //     // SAFETY: This assumes the caller ensures the type matches
// //     unsafe { &*(&UNIT_FUNCTOR_STRING as *const _ as *const UnitFunctor<Id>) }
// // }
//

use crate::core::functor_mapping::FunctorMappings;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::DynArrowTraitType;
use crate::core::traits::category_trait::NCategoryError;
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::unit::unit_category::UnitCategory;

pub const UNIT_FUNCTOR_STRING: UnitFunctor<String> = UnitFunctor {
    _phantom1: std::marker::PhantomData,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitFunctor<T: Identifier> {
    _phantom1: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> FunctorTrait<'a> for UnitFunctor<T> {
    type Identifier = T;
    type SourceCategory = UnitCategory;
    type TargetCategory = UnitCategory;

    fn functor_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_category(&self) -> &Self::SourceCategory {
        todo!()
    }

    fn target_category(&self) -> &Self::TargetCategory {
        todo!()
    }

    fn mappings(
        &self,
    ) -> Result<
        &FunctorMappings<'a, Self::Identifier, Self::SourceCategory, Self::TargetCategory>,
        NCategoryError,
    > {
        todo!()
    }

    fn morphisms(
        &self,
    ) -> Result<
        Vec<
            &'a DynArrowTraitType<'a, Self::SourceCategory, Self::TargetCategory, Self::Identifier>,
        >,
        NCategoryError,
    > {
        todo!()
    }
}
