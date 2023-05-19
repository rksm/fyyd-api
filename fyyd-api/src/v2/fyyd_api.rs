use fyyd_types::v2::fyyd_response::{FyydPodcast, FyydResponse};

const FYYD_API_V2: &str = "https://api.fyyd.de/0.2";
const PODCAST: &str = "/podcast";
const EPISODES: &str = "/episodes";
//const PODCAST_EPISODES: &str = "/podcast/episodes";
//const PODCASTS: &str = " /podcasts";
//const CATEGORIES: &str = "/categories";
//const CATEGORY: &str = "/category";
//const PODCAST_RECOMMEND: &str = "/podcast/recommend";
const _PODCAST_LATEST: &str = "/podcast/latest";
//const PODCAST_COLLECTIONS: &str = "/podcast/collections";
//const EPISODE: &str = "/episode";
//const EPISODE_LATEST: &str = "/episode/latest";
//const EPISODE_CURATIONS: &str = "/episode/curations";

const SEARCH_PODCAST: &str = "/search/podcast";
//const SEARCH_CURATION: &str = "/search/curation";
//const SEARCH_EPISODE: &str = "/search/episode";
//const SEARCH_USER: &str = "/search/user";
//const SEARCH_COLOR: &str = "/search/color";
//const FEATURE_HOT: &str = "/feature/podcast/hot";
//const FEATURE_HOT_LANGUAGES: &str = "/feature/podcast/hot/languages";

/// Search for a podcast feed inside fyyd's database,
/// matches any - or some set of a given critetia.
///
/// Arguments:
///
/// - `title` - the podcast's title
///
/// - `url` - the podcast's url
///
/// - `term` - a search term inside the podcast
///
/// - `page` - the page of the search,
///         WARNING: if the page is overshot it will still return valid pages
pub async fn search_podcast_feed(
    title: Option<String>,
    url: Option<String>,
    term: Option<String>,
    page: Option<u16>,
) -> Result<FyydResponse<Vec<FyydPodcast>>, Box<dyn std::error::Error>> {
    let path = FYYD_API_V2.to_owned() + SEARCH_PODCAST;
    let client = reqwest::Client::new();

    let request = client
        .get(path)
        .query(&[("title", title.unwrap_or_default().as_str())])
        .query(&[("url", url.unwrap_or_default().as_str())])
        .query(&[("term", term.unwrap_or_default().as_str())])
        .query(&[("page", page.unwrap_or_default())]);

    let response = request.send().await?;
    //error!("{:?}", response);
    let fyyd_response = response.text().await?;
    //error!("{:?}", fyyd_response);
    let fyyd_response: FyydResponse<Vec<FyydPodcast>> = serde_json::from_str(&fyyd_response)?;

    Ok(fyyd_response)
}

/// Retrieves information about a podcast
///
/// Retrieve episodes by appending `/episodes`
///
/// Arguments:
///
/// - `id` - the podcast's id from feeds database
///
/// - `slug` - the podcast's slug
///
/// - `count` - the page size (default: 50)
///
/// - `page` - the page of the search,
///         WARNING: if the page is overshot it will still return valid pages
///
/// -  `episodes` - bool - whether to query episodes information
pub async fn get_episodes_from_id(
    id: u64,
    slug: Option<String>,
    count: Option<u16>,
    page: Option<u16>,
    _episodes: bool,
) -> Result<FyydResponse<FyydPodcast>, Box<dyn std::error::Error>> {
    let path = FYYD_API_V2.to_owned() + PODCAST + EPISODES;
    let client = reqwest::Client::new();

    let request = client
        .get(path)
        .query(&[("podcast_id", id)])
        .query(&[("podcast_slug", slug.unwrap_or_default().as_str())])
        .query(&[("page", page.unwrap_or_default())])
        .query(&[("count", count.unwrap_or(50))]);

    let response = request.send().await?;
    let fyyd_response = response.text().await?;
    //error!("{:?}", fyyd_response);
    let fyyd_response: FyydResponse<FyydPodcast> = serde_json::from_str(&fyyd_response)?;

    Ok(fyyd_response)
}

/// Retrieves information about the last added podcasts
///
/// Arguments:
/// - `since_id` - the last podcast's id from feeds database (maximum 1000)
///
/// - `count` - the page size (default: 20)
pub(crate) async fn _get_latest_episodes_from_id(
    since_id: Option<u64>,
    count: Option<u16>,
) -> Result<FyydResponse<FyydPodcast>, Box<dyn std::error::Error>> {
    let path = FYYD_API_V2.to_owned() + _PODCAST_LATEST;
    let client = reqwest::Client::new();

    let request = client
        .get(path)
        .query(&[("since_id", since_id)])
        .query(&[("count", count.unwrap_or(20))]);

    let response = request.send().await?;
    let fyyd_response = response.text().await?;
    //error!("{:?}", fyyd_response);
    let fyyd_response: FyydResponse<FyydPodcast> = serde_json::from_str(&fyyd_response)?;

    Ok(fyyd_response)
}
