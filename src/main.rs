mod model;
mod service;

use std::collections::HashMap;
use std::iter::Map;
use std::thread;
use axum::Router;
use axum::routing::get;
use rand::random;
use tokio::net::TcpListener;

use model::spider::Spider;
use model::spider::UserInfo;

use service::spider::crawl_info;
use crate::model::spider::SpiderResult;
use crate::service::spider::crawl_block_blogs;


#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    if let SpiderResult::UserInfo(info) = crawl_info("2280813617".to_string()).await {
        // if let SpiderResult::BlogList(blogs, offset) = crawl_block_blogs("2280813617".to_string(), info.cid, "".to_string()).await{
        //
        // };
        let mut offset = "".to_string();
        loop {
            let sleep_duration = std::time::Duration::from_secs(random::<u64>() % 5 + 2);

            tokio::time::sleep(sleep_duration).await;
            if let SpiderResult::BlogList(blogs, _offset) = crawl_block_blogs("2280813617".to_string(), info.cid.clone(), offset.clone()).await {
                blogs.iter().for_each(|blog| {
                    blog.pictures.iter().for_each(|picture| {
                        let _ = picture.download("./pictures".to_string());
                    });
                });
                offset = _offset;
            } else {
                break;
            }
            if offset.is_empty() {
                break;
            }
        }
    };
}