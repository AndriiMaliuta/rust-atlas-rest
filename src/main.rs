use std::collections::HashMap;
use std::fmt::format;
use std::future::Future;
use serde::{Deserialize, Serialize, SerializeAs};
use base64::encode;
use reqwest::Error;
use serde_json::Value;
use crate::models::space::Space;
use crate::models::content::{CreatePage, Content, User, Expandable, ContentResponse};

mod models;

// async fn create_page(page: CreatePage) -> Content {
//     let mut map = HashMap::new();
//     map.insert("title", "Some page");
//     map.insert("space", "test45");
//
//     let request_url= format!("http://localhost:{port}/rest/api/content",
//                              port = 7201);
//     let client = reqwest::Client::new();
//     let res = client.post(&request_url)
//         .json(&map)
//         .send()
//         .await.unwrap();
//     let created: Content = res.json().await.unwrap();
//     created
// }

#[tokio::main]
async fn main() -> Result<(),   Error> {
    println!("{}", "[ *** ] Starting");
    let cloud_confl = "http://confl-loadb-pxymvhygf6ct-1493255270.us-west-2.elb.amazonaws.com/";
    let request_url= format!("{}rest/api/content", cloud_confl);
    let token = encode(b"admin:admin");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let res = client
        .get(&request_url)
        .header("Authorization", format!("Basic {token}"))
        .send()
        .await?;

    // let pages: ContentResponse = res.json().await?;

    // for page in pages.results {
    //     // let get_page = format!("{cloud_confl}rest/api/content/{id}", port=80, id=page.id);
    //     let get_page = format!("{cloud_confl}rest/api/content/{id}", id=page.id);
    //     let page_res = client.get(get_page)
    //         .header("Authorization", format!("Basic {token}"))
    //         .send()
    //         .await?;
    //     let page: Content = page_res.json().await?;
    //     println!("{:?}", page)
    // }

    Ok(())
}
