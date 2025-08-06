mod core {
    pub mod category;

    pub mod discrete_category;

    pub mod arrow;

    pub mod identifier;

    pub mod arrow_mapping;

    pub mod functor;

    pub mod type_alias;

    pub mod traits {
        pub mod arrow_trait;
        pub mod category_trait;
        pub mod functor_trait;

    }

    pub mod unit {
        pub mod unit_category;
        pub mod unit_morphism;

        pub mod unit_identifier;

        pub mod unit_functor;
    }

    #[cfg(test)]
    mod tests {
        pub mod ncategory_test_helper;
        pub mod test_generic_ncategory;
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
