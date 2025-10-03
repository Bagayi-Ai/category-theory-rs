use std::sync::LazyLock;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub mod core {
    pub mod discrete_category;

    pub mod identifier;

    pub mod errors;

    pub mod product_endofunctor;

    pub mod base_category;
    pub mod dynamic_category;
    pub mod epic_monic_category;
    pub mod object_id;

    pub mod utils;

    pub mod arrow;

    pub mod functor;

    pub mod persistable_category;

    pub mod functors {
        pub mod inclusion_functor;
    }

    pub mod traits {
        pub mod arrow_trait;

        pub mod functor_trait;

        pub mod category_trait;

        pub mod factorization_system_trait;
    }

    #[cfg(test)]
    mod tests {
        pub mod ncategory_test_helper;
        pub mod test_generic_ncategory;

        pub mod test_dynamic_category;
    }
}

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init_db() -> surrealdb::Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    DB.use_ns("namespace").use_db("database").await?;

    Ok(())
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
