use std::collections::HashMap;
use std::iter::Map;
use std::ptr::null;
use reqwest::{StatusCode, Url};
use tokio::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct Picture{
    id:String,
    normal: String,
    large: String,
    video: String,
}
#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct Blog {
    id: String,
    text: String,
    raw: String,
    source: String,
    region: String,
    create: String,
    page:String,
    pub(crate) pictures: Vec<Picture>,
}

impl Blog {
    pub(crate) fn new(id: String, text: String, raw: String, source: String, region: String, create: String, page:String, pictures:Vec<Picture>) -> Self {
        Self {
            id,
            text,
            raw,
            source,
            region,
            create,
            page,
            pictures,
        }
    }
    pub(crate) fn download(&self) {
        self.pictures.iter().enumerate().for_each(|(index,pic)| {
            let _ = pic.download(format!("{}",index));
        })
    }
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
    pub(crate) cid: String,
}
impl UserInfo {
    pub(crate) fn new(id: u32, name: String, gender: char, description: String, follow: u32, follower:String, profile: String, cover: String, avatar: String,cid:String) -> Self {
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
            cid,
        }
    }
}
struct User {
    info: UserInfo,
    blogs: Vec<Blog>,
}

mod util {
    use axum::http::StatusCode;
    use reqwest::{Client, Url};
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    pub(crate) async fn download_picture(url: String, path: String) -> Result<(), String> {
        let client = reqwest::Client::new();

        let response = client.get(Url::parse(&*url).unwrap()).send().await.unwrap();
        if !response.status().is_success() {
            return Err(response.status().to_string());
        }

        let mut output_file = File::create(path).await.unwrap();

        let mut body = response.bytes().await.unwrap();
        output_file.write_all(&body).await.unwrap();

        Ok(())
    }

    pub(crate) async fn download_video(url: String, path: String)-> Result<(), String>{
        let client = Client::new();
        let response = client.get(url).send().await.unwrap();

        // 检查响应状态是否表示成功
        if !response.status().is_success() {
            return Err(response.status().to_string());
        }

        let file = File::create(path).await.unwrap();
        let mut writer = tokio::io::BufWriter::new(file);

        response.bytes().await.unwrap().iter().for_each(|chunk| {
            let _ = writer.write_u8(*chunk);
        });

        Ok(())
    }
}

impl Picture {
    pub(crate) fn new(id: String, normal: String, large: String, video: String) -> Self {
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
    pub(crate) async fn download(&self, path:String) {
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
        if !self.video.is_empty() {
            let mut path = path.clone();
            let ext = self.video.split('.').last().unwrap();
            let video_path = format!("{}/{}-video{}",path,self.id,ext);
            util::download_video(self.video.clone(), video_path).await.expect("下载视频失败");
        }
    }
}

pub(crate) struct Mapper {
    path: String,
    method: String,
    params:HashMap<String,String>,
    handler: fn(String) -> SpiderResult,
}
pub(crate) struct Spider {
    origin: String,
    mappers: HashMap<String,Mapper>,
}

impl Mapper {
    pub(crate) fn new(path: String, method: String, params: HashMap<String,String>, handler: fn(String) -> SpiderResult) -> Self {
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
pub(crate) enum SpiderResult {
    UserInfo(UserInfo),
    BlogList(Vec<Blog>, String),
}
impl Spider {
    pub(crate) fn new(origin: String, mappers: HashMap<String,Mapper>) -> Self {
        Self {
            origin,
            mappers,
        }
    }

    pub(crate) async fn get_user_info(&self)-> Result<SpiderResult,String> {
        let info = self.mappers.get("info").unwrap();
        let url = format!("{}{}",self.origin,info.sub_url());
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();
        if response.status().is_success() {
            let body = response.text().await.unwrap();
            Ok((info.handler)(body))
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) async fn get_blogs(&self,uid:String,cid:String,offset:String)-> Result<SpiderResult,String> {
        let block = self.mappers.get("block").unwrap();
        let url = format!("{}{}",self.origin,block.sub_url());
        println!("{}",url);
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();
        if response.status().is_success() {
            let body = response.text().await.unwrap();
            Ok((block.handler)(body))
        } else {
            Err(response.status().to_string())
        }
    }
}