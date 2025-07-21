
mod core {
    pub mod uuid_id;

    pub mod ncategory;

    pub mod generic_ncategory;

    pub mod category0;

    pub mod dynamic_ncategory;

    pub mod generic_id;

    #[cfg(test)]
    mod tests {
        pub mod ncategory_test_helper;
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
