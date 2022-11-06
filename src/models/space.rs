use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Space {
    id: i16,
    key: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateSpace {
    key: String,
    name: String,
}
