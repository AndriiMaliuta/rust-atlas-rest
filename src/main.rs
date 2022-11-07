use std::collections::HashMap;
use std::fmt::format;
use std::future::Future;
use std::iter::Map;
use serde::{Deserialize, Serialize};
use base64::encode;
use reqwest::{Body, Error};
use serde_json::{json, Value};


// create page
async fn create_page(page: CreatePage) {
    // let request_url= format!("http://localhost:{port}/rest/api/content", port = 7201);
    let token = encode(b"admin:admin");
    let request_url=
        format!("http://confl-loadb-pxymvhygf6ct-1493255270.us-west-2.elb.amazonaws.com/rest/api/content/");
    let client = reqwest::Client::new();
    let res = client.post(&request_url)
        .json(&page)
        .header("Authorization", format!("Basic {token}"))
        .send()
        .await.unwrap();
    let created: String= res.text().await.unwrap();
    println!("{:?}", created);
}


// MAIN
#[tokio::main]
async fn main() -> Result<(),   Error> {
    println!("{}", "[ *** ] Starting");
    let dc_url = "http://confl-loadb-pxymvhygf6ct-1493255270.us-west-2.elb.amazonaws.com";
    // let request_url= format!("http://localhost:{port}/rest/api/content",
    //                          port = 7201);

    /*
    let request_url= format!("{}/rest/api/content/?type=page", dc_url);
    let token = encode(b"admin:admin");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let res = client
        .get(&request_url)
        .header("Authorization", format!("Basic {token}"))
        .send()
        .await?;

    let pages: ContentResponse = res.json().await?;
    // println!("{:?}", pages.results);

    for page in pages.results {
        let get_page = format!("{}/rest/api/content/{id}/", dc_url, id=page.id);
        let page_res = client.get(get_page)
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await?;
        let page: CntPage = page_res.json().await?;
        println!("{:?}", page.id)
    }
     */


    // CREATE PAGE
    for a in 2..20 {
        let mut to_create: CreatePage = CreatePage {
            title: format!("Page Rust {a}"),
            ctype: "page".to_string(),
            space: CreatePageSpace {
                key: "DEV14".to_string(),
            },
            body: PageBody {
                storage: Storage {
                    representation: "storage".to_string(),
                    value: "Lorem ipsum dolor sit amet consectetur adipiscing elit mal...".to_string(),
                }
            },
            ancestors: vec![Ancestor {
                id: 1048691
            }]
        };

        println!("{:?}", serde_json::to_string(&to_create));

        let created = create_page(to_create);
        let fin = created.await;
    }

    Ok(())
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Extentions {
    // #[serde(skip_serializing_if = "String::is_empty")]
    position: Value,
}

// #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
// #[repr(u8)]
// enum PositionEnum {
//     Str = 2,
//     Numm = 3,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Content {
    id: String,
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    Type: String,
    status: String,
    title: String,
    extensions: Extentions,
    #[serde(rename(deserialize = "_expandable"))]
    _expandable: Expandable,
    _links: ContentLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Expandable {
    container: String,
    metadata: String,
    operations: String,
    children: String,
    restrictions: String,
    history: String,
    ancestors: String,
    body: String,
    version: String,
    descendants: String,
    space: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ContentLinks {
    #[serde(rename(serialize = "self"))]
    #[serde(rename(deserialize = "self"))]
    sself: String,
    webui: String,
    edit: String,
    tinyui: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Links {
    #[serde(rename(serialize = "self"))]
    #[serde(rename(deserialize = "self"))]
    sself: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    // #[serde(skip_deserializing_if = "String::is_empty")]
    next: String,
    base: String,
    context: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ContentResponse {
    results: Vec<Content>,
    start: i8,
    limit: i8,
    size: i8,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip)]
    _links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    login: String,
    id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePage {
    title: String,
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    ctype: String,
    space: CreatePageSpace,
    body: PageBody,
    ancestors: Vec<Ancestor>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBody {
    storage: Storage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Storage {
    representation: String,
    value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePageSpace {
    key : String
}

/////////


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ancestor {
    id: i32,
}

// One content page
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CntPage {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub status: String,
    pub title: String,
    pub space: CntSpace,
    pub history: CntHistory,
    pub version: Version,
    pub extensions: Extentions,
    #[serde(rename = "_links")]
    #[serde(skip)]
    pub links: Links,
    #[serde(skip)]
    // #[serde(rename = "_expandable")]
    pub expandable: Expandable,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CntSpace {
    pub id: i64,
    pub key: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "_links")]
    #[serde(skip)]
    pub links: Links,
    // #[serde(rename = "_expandable")]
    #[serde(skip)]
    pub expandable: CntExpandable,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CntHistory {
    pub latest: bool,
    pub created_by: CreatedBy,
    pub created_date: String,
    #[serde(rename = "_links")]
    #[serde(skip)]
    pub links: Links,
    // #[serde(rename = "_expandable")]
    #[serde(skip)]
    pub expandable: CntExpandable,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    #[serde(rename = "type")]
    pub type_field: String,
    pub profile_picture: ProfilePicture,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePicture {
    pub path: String,
    pub width: i64,
    pub height: i64,
    pub is_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub by: By,
    pub when: String,
    pub message: String,
    pub number: i64,
    pub minor_edit: bool,
    pub hidden: bool,
    #[serde(rename = "_links")]
    #[serde(skip)]
    pub links: Links,
    // #[serde(rename = "_expandable")]
    #[serde(skip)]
    pub expandable: CntExpandable,
}

#[serde(rename_all = "camelCase")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct By {
    #[serde(rename = "type")]
    pub type_field: String,
    pub profile_picture: ProfilePicture,
    pub display_name: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CntExpandable {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub container: String,
    pub metadata: String,
    pub operations: String,
    pub children: String,
    pub restrictions: String,
    pub ancestors: String,
    pub body: String,
    pub descendants: String,
}
