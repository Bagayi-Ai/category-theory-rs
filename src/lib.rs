mod core {
    pub mod ncategory;

    pub mod generic_ncategory;

    pub mod discrete_category;

    pub mod morphism;

    pub mod generic_morphism;

    pub mod generic_nfunctor;

    pub mod functor_mapping;

    pub mod nfunctor;

    pub mod morphism_tree;

    pub mod identifier;

    pub mod unit {
        pub mod unit_category;
        pub mod unit_functor;

        pub mod unit_morphism;

        pub mod unit_identifier;
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
