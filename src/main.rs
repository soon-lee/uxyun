mod model;
mod service;

use std::collections::HashMap;
use std::iter::Map;
use std::thread;
use axum::Router;
use axum::routing::get;
use rand::random;
use tokio::net::TcpListener;
use tokio_stream::{Stream, StreamExt};
use futures::stream::{self, StreamExt as _FutureStreamExt};

use model::spider::Spider;
use model::spider::UserInfo;

use crate::model::spider::SpiderResult;
use crate::service::spider::{crawl_block_blogs_async, crawl_block_blogs_await, crawl_info_async, crawl_info_await};

async fn test_async() {
    if let SpiderResult::UserInfo(info) = crawl_info_async("2280813617".to_string()).await {
        let mut offset = "".to_string();
        let mut index = 0;
        loop {
            let sleep_duration = std::time::Duration::from_secs(random::<u64>() % 5 + 2);

            tokio::time::sleep(sleep_duration).await;
            if let SpiderResult::BlogList(blogs, _offset) = crawl_block_blogs_async("2280813617".to_string(), info.cid.clone(), offset.clone()).await {
                tokio_stream::iter(blogs.into_iter()).for_each_concurrent(None,|blog| {
                    async move {
                        index += 1;
                        tokio_stream::iter(blog.pictures.iter()).for_each_concurrent(None,|picture| {
                            // println!("{:?}",picture.clone());
                            async move {
                                picture.download_async(format!("res/{:04}", index)).await;
                            }
                        }).await;
                    }
                }).await;
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
fn test_await(){
    if let SpiderResult::UserInfo(info) = crawl_info_await("2280813617".to_string()) {
        let mut offset = "".to_string();
        let mut index = 0;
        loop {
            let sleep_duration = std::time::Duration::from_secs(random::<u64>() % 5 + 2);

            std::thread::sleep(sleep_duration);
            if let SpiderResult::BlogList(blogs, _offset) = crawl_block_blogs_await("2280813617".to_string(), info.cid.clone(), offset.clone()) {
                blogs.iter().for_each(|blog| {
                    index += 1;
                    blog.pictures.iter().for_each(|picture| {
                        // println!("{:?}",picture.clone());
                        picture.download_await(format!("res/{:04}", index));
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

// #[tokio::main]
fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    test_await();
}