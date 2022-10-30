use std::collections::HashMap;
use serde::Deserialize;
use base64::encode;
use reqwest::Error;

mod pages;

#[derive(Deserialize, Debug)]
struct Content {
    id: i32,
    title: String,
    body: String,
}

#[derive(Deserialize, Debug)]
struct ContentResponse {
    results: Vec<Content>,
    start: i8,
    limit: i8,
    size: i8,
    _links: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

// async fn create_page() -> Content {
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
//         .await?;
//     Content {
//         id: 1111,
//         title: "".to_string(),
//         body: "".to_string()
//     };
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("{}", ">>>>> Starting");
    let request_url= format!("http://localhost:{port}/rest/api/content",
                               port = 7201);
    let token = encode(b"admin:admin");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    // let response = client
    //     .get(&request_url)
    //     .header("Authorization", format!("Basic {token}"))
    //     .send()
    //     .await?;
    // // Result<Response, reqwest::Error>
    // let contents: Vec<Content> = response.json().await?;
    // println!("{}", ">>>>>>>>> response:");
    // println!("{:?}", contents);

    let res = client
        .get(&request_url)
        .header("Authorization", format!("Basic {token}"))
        .send()
        .await?;

    let contents: ContentResponse = res.json::<ContentResponse>().await.unwrap();
    println!("{:?}", contents);

    Ok(())
}
