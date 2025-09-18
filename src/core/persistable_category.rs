use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub struct PersistableCategory {
    db: Surreal<Client>,
    category_id: String,
}

impl PersistableCategory {}
