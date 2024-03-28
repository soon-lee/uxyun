mod model;

use std::collections::HashMap;
use std::iter::Map;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

use model::spider::Spider;
use model::spider::UserInfo;

async fn test(uid: String)-> UserInfo {
    let info_mapper = model::spider::Mapper::new(
        "/api/container/getIndex".to_string(),
        "GET".to_string(),
        HashMap::from([
            ("jumpfrom".to_string(), "weibocom".to_string()),
            ("type".to_string(), "uid".to_string()),
            ("value".to_string(), uid)
        ]),
        |text| {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();
            UserInfo::new(
                json["data"]["userInfo"]["id"].as_u64().unwrap() as u32,
                json["data"]["userInfo"]["screen_name"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["gender"].as_str().unwrap().chars().next().unwrap(),
                json["data"]["userInfo"]["description"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["follow_count"].as_u64().unwrap() as u32,
                json["data"]["userInfo"]["followers_count"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["profile_url"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["cover_image_phone"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["avatar_hd"].as_str().unwrap().to_string(),
            )
        }
    );
    let mappers = HashMap::from([("info".to_string(), info_mapper)]);
    let spider = Spider::new("https://m.weibo.cn".to_string(), mappers);
    spider.get_user_info().await.expect("获取用户信息失败")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    //
    // // run our app with hyper, listening globally on port 3000
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    println!("{:?}",test("2280813617".to_string()).await);
}