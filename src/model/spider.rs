use std::collections::HashMap;
use std::iter::Map;
use std::ptr::null;
use reqwest::{StatusCode, Url};
use tokio::fs::File;
use serde::{Deserialize, Serialize};

struct Picture{
    id:String,
    normal: String,
    large: String,
    video: String,
}
struct Blog {
    id: u32,
    title: String,
    text: String,
    raw: String,
    source: String,
    page:String,
    pictures: Vec<Picture>,
}
#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct UserInfo {
    id: u32,
    name: String,
    gender: char,
    description: String,
    follow: u32,
    follower:String,
    profile: String,
    cover: String,
    avatar: String,
}
impl UserInfo {
    pub(crate) fn new(id: u32, name: String, gender: char, description: String, follow: u32, follower:String, profile: String, cover: String, avatar: String) -> Self {
        Self {
            id,
            name,
            gender,
            description,
            follow,
            follower,
            profile,
            cover,
            avatar,
        }
    }
}
struct User {
    info: UserInfo,
    blogs: Vec<Blog>,
}

mod util {
    use axum::http::StatusCode;
    use reqwest::Url;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    pub(crate) async fn download_picture(url: String, path: String) -> Result<(), StatusCode> {
        let client = reqwest::Client::new();

        let response = client.get(Url::parse(&*url).unwrap()).send().await.unwrap();
        if !response.status().is_success() {
            return Err(response.status());
        }

        let mut output_file = File::create(path).await.unwrap();

        let mut body = response.bytes().await.unwrap();
        output_file.write_all(&body).await.unwrap();

        Ok(())
    }
}

impl Picture {
    fn new(id: String, normal: String, large: String, video: String) -> Self {
        Self {
            id,
            normal,
            large,
            video,
        }
    }
    fn default(id: String, normal: String) -> Self {
        Self::new(id,normal,"".to_string(),"".to_string())
    }
    async fn download(&self,path:String) {
        let mut normal_path = String::new();
        let mut large_path = String::new();
        if !self.normal.is_empty() {
            let mut path = path.clone();
            let ext = self.normal.split('.').last().unwrap();
            normal_path = format!("{}/{}{}",path,self.id,ext);
            util::download_picture(normal_path, path).await.expect("打开图片失败");
        }
        if !self.large.is_empty() {
            let mut path = path.clone();
            let ext = self.large.split('.').last().unwrap();
            large_path = format!("{}/{}-large{}",path,self.id,ext);
            util::download_picture(large_path, path).await.expect("打开图片失败");
        }
    }
}

pub(crate) struct Mapper {
    path: String,
    method: String,
    params:HashMap<String,String>,
    handler: fn(String) -> UserInfo,
}
pub(crate) struct Spider {
    origin: String,
    mappers: HashMap<String,Mapper>,
}

impl Mapper {
    pub(crate) fn new(path: String, method: String, params: HashMap<String,String>, handler: fn(String) -> UserInfo) -> Self {
        Self {
            path,
            method,
            params,
            handler,
        }
    }
    fn sub_url(&self) -> String {
        format!("{}?{}",self.path,self.params.iter().map(|(k,v)| format!("{}={}",k,v)).collect::<Vec<String>>().join("&"))
    }
}
impl Spider {
    pub(crate) fn new(origin: String, mappers: HashMap<String,Mapper>) -> Self {
        Self {
            origin,
            mappers,
        }
    }

    pub(crate) async fn get_user_info(&self)-> Result<UserInfo,String> {
        let info_mapper = self.mappers.get("info").unwrap();
        let url = format!("{}{}",self.origin,info_mapper.sub_url());
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();
        if response.status().is_success() {
            let body = response.text().await.unwrap();
            Ok((info_mapper.handler)(body))
        } else {
            Err(response.status().to_string())
        }
    }
}