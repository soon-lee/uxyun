use crate::model;

pub(crate) fn build_crawl_info(uid: String) -> model::spider::Mapper {
    model::spider::Mapper::new(
        "/api/container/getIndex".to_string(),
        "GET".to_string(),
        std::collections::HashMap::from([
            ("jumpfrom".to_string(), "weibocom".to_string()),
            ("type".to_string(), "uid".to_string()),
            ("value".to_string(), uid)
        ]),
        |text| {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();
            let mut cid = String::new();
            json["data"]["tabsInfo"]["tabs"].as_array().unwrap().iter().for_each(|tab| {
                if tab.as_object().unwrap().get("tab_type").unwrap().as_str().unwrap().eq_ignore_ascii_case("weibo") {
                    cid = tab.as_object().unwrap().get("containerid").unwrap().as_str().unwrap().to_string();
                }
            });
            model::spider::SpiderResult::UserInfo(model::spider::UserInfo::new(
                json["data"]["userInfo"]["id"].as_u64().unwrap() as u32,
                json["data"]["userInfo"]["screen_name"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["gender"].as_str().unwrap().chars().next().unwrap(),
                json["data"]["userInfo"]["description"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["follow_count"].as_u64().unwrap() as u32,
                json["data"]["userInfo"]["followers_count"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["profile_url"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["cover_image_phone"].as_str().unwrap().to_string(),
                json["data"]["userInfo"]["avatar_hd"].as_str().unwrap().to_string(),
                cid,
            ))
        },
    )
}
pub(crate) async fn crawl_info_async(uid: String) -> model::spider::SpiderResult {
    let info = build_crawl_info(uid);
    let mappers = std::collections::HashMap::from([("info".to_string(), info)]);
    let spider = model::spider::Spider::new("https://m.weibo.cn".to_string(), mappers);
    spider.get_mapper_result_async("info".parse().unwrap()).await.expect("获取用户信息失败")
}
pub(crate) fn crawl_info_await(uid: String) -> model::spider::SpiderResult {
    let info = build_crawl_info(uid);
    let mappers = std::collections::HashMap::from([("info".to_string(), info)]);
    let spider = model::spider::Spider::new("https://m.weibo.cn".to_string(), mappers);
    spider.get_mapper_result_await("info".parse().unwrap()).expect("获取用户信息失败")
}

pub(crate) fn build_crawl_block_blogs(uid: String, cid: String, offset: String) -> model::spider::Mapper {
    model::spider::Mapper::new(
        "/api/container/getIndex".to_string(),
        "GET".to_string(),
        std::collections::HashMap::from([
            ("jumpfrom".to_string(), "weibocom".to_string()),
            ("type".to_string(), "uid".to_string()),
            ("value".to_string(), uid.clone()),
            ("containerid".to_string(), cid.clone()),
            ("since_id".to_string(), offset.clone())
        ]), |text| {
            let json: serde_json::Value = serde_json::from_str(&text).expect("解析失败");
            model::spider::SpiderResult::BlogList(json["data"]["cards"].as_array().unwrap().iter().map(|card| {
                let mut pics = vec![];
                if card["mblog"]["pics"].is_array() {
                    pics = card["mblog"]["pics"].as_array().unwrap().iter().map(|pic| {
                        model::spider::Picture::new(
                            if  pic["pid"].is_null() { "".to_string() } else { pic["pid"].as_str().unwrap().to_string() },
                            if pic["url"].is_null() { "".to_string() } else { pic["url"].as_str().unwrap().to_string() },
                            if pic["large"]["url"].is_null() { "".to_string() } else { pic["large"]["url"].as_str().unwrap().to_string() },
                            if pic["videoSrc"].is_null() { "".to_string() } else { pic["videoSrc"].as_str().unwrap().to_string() },
                        )
                    }).collect::<Vec<model::spider::Picture>>();
                }
                if card["mblog"]["pics"].is_object(){
                    pics = card["mblog"]["pics"].as_object().unwrap().values().into_iter().map(|pic| {
                        model::spider::Picture::new(
                            if  pic["pid"].is_null() { "".to_string() } else { pic["pid"].as_str().unwrap().to_string() },
                            if pic["url"].is_null() { "".to_string() } else { pic["url"].as_str().unwrap().to_string() },
                            if pic["large"]["url"].is_null() { "".to_string() } else { pic["large"]["url"].as_str().unwrap().to_string() },
                            if pic["videoSrc"].is_null() { "".to_string() } else { pic["videoSrc"].as_str().unwrap().to_string() },
                        )
                    }).collect::<Vec<model::spider::Picture>>();
                }
                return model::spider::Blog::new(
                     if card["mblog"]["id"].is_null() { "".to_string() } else { card["mblog"]["id"].as_str().unwrap().to_string() },
                    if card["mblog"]["text"].is_null() { "".to_string() } else { card["mblog"]["text"].as_str().unwrap().to_string() },
                    if card["mblog"]["raw_text"].is_null() { "".to_string() } else { card["mblog"]["raw_text"].as_str().unwrap().to_string() },
                    if card["mblog"]["source"].is_null() { "".to_string() } else { card["mblog"]["source"].as_str().unwrap().to_string() },
                     if card["mblog"]["region"].is_null() { "".to_string() } else { card["mblog"]["region"].as_str().unwrap().to_string() },
                    if card["mblog"]["create_at"].is_null() { "".to_string() } else { card["mblog"]["create_at"].as_str().unwrap().to_string() },
                     if card["scheme"].is_null() { "".to_string() } else { card["scheme"].as_str().unwrap().to_string() },
                    pics);
            }).collect::<Vec<model::spider::Blog>>(), if json["data"]["cardlistInfo"]["since_id"].is_null() { "".to_string() } else {json["data"]["cardlistInfo"]["since_id"].as_number().unwrap().to_string() })
        })
}
pub(crate) async fn crawl_block_blogs_async(uid: String, cid: String, offset: String) -> model::spider::SpiderResult {
    let block = build_crawl_block_blogs(uid,cid,offset);
    let mappers = std::collections::HashMap::from([("block".to_string(), block)]);
    let spider = model::spider::Spider::new("https://m.weibo.cn".to_string(), mappers);
    spider.get_mapper_result_async("block".parse().unwrap()).await.expect("获取用户信息失败")
}
pub(crate) fn crawl_block_blogs_await(uid: String, cid: String, offset: String) -> model::spider::SpiderResult {
    let block = build_crawl_block_blogs(uid, cid, offset);
    let mappers = std::collections::HashMap::from([("block".to_string(), block)]);
    let spider = model::spider::Spider::new("https://m.weibo.cn".to_string(), mappers);
    spider.get_mapper_result_await("block".parse().unwrap()).expect("获取用户信息失败")
}