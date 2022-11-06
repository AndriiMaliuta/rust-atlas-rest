use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Extentions {
    // #[serde(skip_serializing_if = "String::is_empty")]
    pub position: Value,
}

// #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
// #[repr(u8)]
// enum PositionEnum {
//     Str = 2,
//     Numm = 3,
// }

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Content {
    pub id: String,
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    pub Type: String,
    pub status: String,
    pub title: String,
    pub extensions: Extentions,
    pub _expandable: Expandable,
    pub _links: ContentLinks,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Expandable {
    pub container: String,
    pub metadata: String,
    pub operations: String,
    pub children: String,
    pub restrictions: String,
    pub history: String,
    pub ancestors: String,
    pub body: String,
    pub version: String,
    pub descendants: String,
    pub space: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ContentLinks {
    #[serde(rename(serialize = "self"))]
    #[serde(rename(deserialize = "self"))]
    pub sself: String,
    pub webui: String,
    pub edit: String,
    pub tinyui: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Links {
    #[serde(rename(serialize = "self"))]
    #[serde(rename(deserialize = "self"))]
    pub sself: String,
    // #[serde(skip_serializing_if = "String::is_empty()")]
    // #[serde(skip_deserializing_if = "String::is_empty()")]
    // pub next: String,
    #[serde(skip_serializing_if = "String::is_empty()")]
    #[serde(skip_deserializing_if = "String::is_empty()")]
    pub base: String,
    #[serde(skip_serializing_if = "String::is_empty()")]
    pub context: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ContentResponse {
    pub results: Vec<Content>,
    pub start: i8,
    pub limit: i8,
    pub size: i8,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "_links"))]
    #[serde(rename(deserialize = "_links"))]
    pub _links: Links,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub login: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CreatePage {
    pub title: String,
    pub body: String,
    pub space_key: String,
    pub parent_id: String,
}
