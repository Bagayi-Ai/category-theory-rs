pub mod core {
    pub mod category;

    pub mod discrete_category;

    pub mod identifier;

    pub mod functor;

    pub mod errors;

    pub mod morphism;

    pub mod product_endofunctor;

    pub mod expand_functor;

    pub mod set_category;

    pub mod epic_monic_category;

    pub mod traits {
        pub mod arrow_trait;
        pub mod category_trait;
        pub mod functor_trait;

        pub mod morphism_trait;

        pub mod factorization_system_trait;
    }

    pub mod dynamic_category {
        pub mod dynamic_category;
        pub mod dynamic_value;
    }

    pub mod unit {
        pub mod unit_category;
        pub mod unit_morphism;

        pub mod unit_identifier;
    }

    #[cfg(test)]
    mod tests {
        pub mod ncategory_test_helper;
        pub mod test_generic_ncategory;

        pub mod test_dynamic_category;
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
