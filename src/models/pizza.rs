// pizza.rs
// # derive_more = { version = "1.0.0", features = ["full"] }

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePizzaId {
    #[validate(length(min = 1, message = "UUID is required"))]
    pub id: String,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Pizza {
    pub id: RecordId,
    pub name: String,
}

impl BuyPizzaRequest {
    pub fn create(name: String) -> BuyPizzaRequest {
        BuyPizzaRequest { name }
    }
}
