use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct FyydResponse<T> {
    status: u16,
    msg: String,
    pub meta: FyydMetadata,
    pub data: Option<T>,
    //pub(crate) data: Option<FyydData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FyydData {
    FyydEpisode(Box<FyydEpisode>),
    FyydPodcastFeeds(Vec<FyydPodcast>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FyydPodcastFeeds {
    pub(crate) feeds: Vec<FyydPodcast>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FyydMetadata {
    count: Option<u16>,
    #[serde(rename = "API_INFO")]
    api_info: FyydApiInfo,
    pub paging: Option<FyydPaging>,
    // todo: IP Address
    #[serde(rename = "SERVER")]
    server: Option<IpAddr>,
    duration: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FyydPaging {
    pub count: u16,
    pub page: u16,
    pub first_page: u16,
    pub last_page: i32,
    pub next_page: Option<u16>,
    pub prev_page: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FyydApiInfoFromApi {
    #[serde(rename = "API_VERSION")]
    api_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FyydApiInfo {
    #[serde(rename = "API_VERSION")]
    api_version: FyydApiVersion,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FyydApiVersion {
    ApiVersionString(String),
    ApiVersionFloat(f32),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FyydEpisode {
    pub title: String,
    pub num_episode: u64,
    num_season: Option<u16>,
    id: u64,
    // hex?
    guid: String,
    url: String,
    podcast_id: u64,
    img_url: Option<String>,
    // date
    content_type: Option<String>,
    inserted: Option<String>,
    pub_date: Option<String>,
    duration: Option<u16>,
    url_fyyd: Option<String>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum StringOrU32 {
    String(String),
    U32(u32),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FyydPodcast {
    pub title: String,
    pub id: u64,
    // hex?
    pub guid: Option<String>,
    #[serde(rename = "xmlURL")]
    pub xml_url: String,
    #[serde(rename = "htmlURL")]
    pub html_url: Option<String>,
    #[serde(rename = "imgURL")]
    pub img_url: String,
    pub status: u16,
    // FIXME: dateobject
    pub status_since: String,
    pub slug: String,
    #[serde(rename = "layoutImageURL")]
    pub layout_image_url: String,
    #[serde(rename = "thumbImageURL")]
    pub thumb_image_url: String,
    #[serde(rename = "smallImageURL")]
    pub small_image_url: String,
    #[serde(rename = "microImageURL")]
    pub micro_image_url: String,
    pub language: Option<String>,
    // FIXME: dateobject
    pub lastpoll: Option<String>,
    pub generator: Option<String>,
    pub categories: Vec<u16>,
    // date
    pub lastpub: String,
    pub rank: u16,
    pub url_fyyd: String,
    pub description: String,
    pub subtitle: String,
    pub tcolor: Option<String>,
    pub color: Option<String>,
    pub episode_count: Option<StringOrU32>,
    pub episodes: Option<Vec<FyydEpisode>>,
    pub iflags: Option<String>,
    #[serde(rename = "paymentURL")]
    pub payment_url: Option<String>,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FyydPodcastStats {}

#[derive(Serialize, Deserialize, Debug)]
pub struct FyydCuration {
    title: String,
    id: u64,
    description: String,
    layout_image_url: String,
    thumb_image_url: String,
    micro_image_url: String,
    public: u64,
    fyyd_type: u16,
    slug: String,
    user_id: u64,
    url: String,
    xml_url: String,
    episodes: Vec<FyydEpisode>,
}
