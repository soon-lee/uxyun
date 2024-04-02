use std::collections::HashMap;
use std::path::Path;
use futures::StreamExt;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Picture {
    id: String,
    normal: String,
    large: String,
    video: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Blog {
    id: String,
    text: String,
    raw: String,
    source: String,
    region: String,
    create: String,
    page: String,
    pub(crate) pictures: Vec<Picture>,
}

impl Blog {
    pub(crate) fn new(id: String, text: String, raw: String, source: String, region: String, create: String, page: String, pictures: Vec<Picture>) -> Self {
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
    pub(crate) async fn download_async(&self) {
        tokio_stream::iter(self.pictures.iter().enumerate()).for_each_concurrent(None,|(index, pic)| {
            async move{
                pic.download_async(format!("{}", index)).await;
            }
        }).await;
    }
    pub(crate) fn download_await(&self) {
        self.pictures.iter().enumerate().for_each(|(index, pic)| {
            pic.download_await(format!("{}", index));
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserInfo {
    id: u32,
    name: String,
    gender: char,
    description: String,
    follow: u32,
    follower: String,
    profile: String,
    cover: String,
    avatar: String,
    pub(crate) cid: String,
}

impl UserInfo {
    pub(crate) fn new(id: u32, name: String, gender: char, description: String, follow: u32, follower: String, profile: String, cover: String, avatar: String, cid: String) -> Self {
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

#[derive(Serialize, Deserialize, Debug)]
struct User {
    info: UserInfo,
    blogs: Vec<Blog>,
}

mod util {
    use tokio::io::AsyncWriteExt;

    pub(crate) async fn download_async(url: String, path: String) -> Result<(), String> {
        println!("async download: {} to {}", url, path);
        let client = reqwest::Client::new();

        let mut response = client.get(url.clone()).send().await.unwrap();
        if !response.status().is_success() {
            return Err(response.status().as_str().parse().unwrap());
        }

        let mut file = tokio::fs::File::create(path.clone()).await.unwrap();

        file.write_all(response.bytes().await.unwrap().as_ref()).await.unwrap();

        Ok(())
    }

    pub(crate) fn download_await(url: String, path: String) -> Result<(), String> {
        println!("await download : {} to {}", url, path);
        let mut response = reqwest::blocking::get(url).unwrap();

        if !response.status().is_success() {
            return Err(response.status().to_string());
        }

        let mut dest_file = std::fs::File::create(path).unwrap();

        response.copy_to(&mut dest_file).unwrap();

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
        Self::new(id, normal, "".to_string(), "".to_string())
    }
    pub(crate) async fn download_async(&self, path: String) {
        if !Path::new(&path.clone()).exists() {
            tokio::fs::create_dir_all(path.clone()).await.unwrap();
        }
        let mut normal_path = String::new();
        let mut large_path = String::new();
        let sleep_duration = std::time::Duration::from_secs(random::<u64>() % 2 + 1);

        tokio::time::sleep(sleep_duration).await;
        if !self.normal.is_empty() {
            let mut path = path.clone();
            let ext = self.normal.split('.').last().unwrap();
            normal_path = format!("{}/{}.{}", path, self.id, ext);
            util::download_async(self.normal.clone(), normal_path).await.expect("打开图片失败");
        }
        if !self.large.is_empty() {
            let mut path = path.clone();
            let ext = self.large.split('.').last().unwrap();
            large_path = format!("{}/{}-large.{}", path, self.id, ext);
            util::download_async(self.normal.clone(), large_path).await.expect("打开图片失败");
        }
        if !self.video.is_empty() {
            let mut path = path.clone();
            let ext = self.video.split('.').last().unwrap();
            let video_path = format!("{}/{}-video.{}", path, self.id, ext);
            util::download_async(self.video.clone(), video_path).await.expect("下载视频失败");
        }
    }
    pub(crate) fn download_await(&self, path: String) {
        if !Path::new(&path.clone()).exists() {
            std::fs::create_dir_all(path.clone()).unwrap();
        }
        let mut normal_path = String::new();
        let mut large_path = String::new();
        let sleep_duration = std::time::Duration::from_secs(random::<u64>() % 2 + 1);

        std::thread::sleep(sleep_duration);
        if !self.normal.is_empty() {
            let mut path = path.clone();
            let ext = self.normal.split('.').last().unwrap();
            normal_path = format!("{}/{}.{}", path, self.id, ext);
            util::download_await(self.normal.clone(), normal_path).expect("打开图片失败");
        }
        if !self.large.is_empty() {
            let mut path = path.clone();
            let ext = self.large.split('.').last().unwrap();
            large_path = format!("{}/{}-large.{}", path, self.id, ext);
            util::download_await(self.normal.clone(), large_path).expect("打开图片失败");
        }
        if !self.video.is_empty() {
            let mut path = path.clone();
            let ext = self.video.split('.').last().unwrap();
            let video_path = format!("{}/{}-video.{}", path, self.id, ext);
            util::download_await(self.video.clone(), video_path).expect("下载视频失败");
        }
    }
}

pub(crate) struct Mapper {
    path: String,
    method: String,
    params: HashMap<String, String>,
    handler: fn(String) -> SpiderResult,
}

pub(crate) struct Spider {
    origin: String,
    mappers: HashMap<String, Mapper>,
}

impl Mapper {
    pub(crate) fn new(path: String, method: String, params: HashMap<String, String>, handler: fn(String) -> SpiderResult) -> Self {
        Self {
            path,
            method,
            params,
            handler,
        }
    }
    fn sub_url(&self) -> String {
        format!("{}?{}", self.path, self.params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&"))
    }
}

pub(crate) enum SpiderResult {
    UserInfo(UserInfo),
    BlogList(Vec<Blog>, String),
}

impl Spider {
    pub(crate) fn new(origin: String, mappers: HashMap<String, Mapper>) -> Self {
        Self {
            origin,
            mappers,
        }
    }

    pub(crate) async fn get_mapper_result_async(&self,name:String) -> Result<SpiderResult, String> {
        let mapper = self.mappers.get(&name).unwrap();
        let url = format!("{}{}", self.origin, mapper.sub_url());
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();
        if response.status().is_success() {
            let body = response.text().await.unwrap();
            Ok((mapper.handler)(body))
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) fn get_mapper_result_await(&self,name:String) -> Result<SpiderResult, String> {
        let mapper = self.mappers.get(&name).unwrap();
        let url = format!("{}{}", self.origin, mapper.sub_url());
        println!("{}", url);
        let response = reqwest::blocking::get(url).unwrap();
        if response.status().is_success() {
            let body = response.text().unwrap();
            Ok((mapper.handler)(body))
        } else {
            Err(response.status().to_string())
        }
    }
}