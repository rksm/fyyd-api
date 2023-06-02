use std::sync::OnceLock;

use fyyd_types::v2::fyyd_response::{FyydPodcast, FyydResponse};
use reqwest::{Client, ClientBuilder};

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

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn get_http_client(client_builder: Option<ClientBuilder>) -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        let builder = client_builder.unwrap_or_else(ClientBuilder::new);
        builder.build().unwrap()
    })
}

/// Client for interacting with the fyyd API.
pub struct FyydClient {
    client: Client,
}

impl FyydClient {
    /// Creates a new instance of the FyydClient.
    ///
    /// Arguments:
    ///
    /// - `client_builder` - Optional `ClientBuilder` to customize the underlying HTTP client.
    ///
    /// Returns:
    ///
    /// A new instance of `FyydClient`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fyyd_api::v2::fyyd_api::FyydClient;
    /// use reqwest::ClientBuilder;
    ///
    /// let client_builder = ClientBuilder::new().timeout(std::time::Duration::from_secs(10));
    /// let client = FyydClient::new(Some(client_builder));
    /// ```
    pub fn new(client_builder: Option<ClientBuilder>) -> Self {
        let client = get_http_client(client_builder).clone();
        FyydClient { client }
    }

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
        &self,
        title: Option<String>,
        url: Option<String>,
        term: Option<String>,
        page: Option<u16>,
    ) -> Result<FyydResponse<Vec<FyydPodcast>>, Box<dyn std::error::Error>> {
        let path = FYYD_API_V2.to_owned() + SEARCH_PODCAST;

        let request = self
            .client
            .get(path)
            .query(&[("title", title.unwrap_or_default().as_str())])
            .query(&[("url", url.unwrap_or_default().as_str())])
            .query(&[("term", term.unwrap_or_default().as_str())])
            .query(&[("page", page.unwrap_or_default())]);

        let response = request.send().await?;
        let body = response.bytes().await?.to_vec();
        let fyyd_response: FyydResponse<Vec<FyydPodcast>> = serde_json::from_slice(&body)?;

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
        &self,
        id: u64,
        slug: Option<String>,
        count: Option<u16>,
        page: Option<u16>,
        _episodes: bool,
    ) -> Result<FyydResponse<FyydPodcast>, Box<dyn std::error::Error>> {
        let path = FYYD_API_V2.to_owned() + PODCAST + EPISODES;

        let request = self
            .client
            .get(path)
            .query(&[("podcast_id", id)])
            .query(&[("podcast_slug", slug.unwrap_or_default().as_str())])
            .query(&[("page", page.unwrap_or_default())])
            .query(&[("count", count.unwrap_or(50))]);

        let response = request.send().await?;
        let body = response.bytes().await?.to_vec();
        let fyyd_response: FyydResponse<FyydPodcast> = serde_json::from_slice(&body)?;

        Ok(fyyd_response)
    }

    /// Retrieves information about the last added podcasts
    ///
    /// Arguments:
    /// - `since_id` - the last podcast's id from feeds database (maximum 1000)
    ///
    /// - `count` - the page size (default: 20)
    pub(crate) async fn _get_latest_episodes_from_id(
        &self,
        since_id: Option<u64>,
        count: Option<u16>,
    ) -> Result<FyydResponse<FyydPodcast>, Box<dyn std::error::Error>> {
        let path = FYYD_API_V2.to_owned() + _PODCAST_LATEST;

        let request = self
            .client
            .get(path)
            .query(&[("since_id", since_id)])
            .query(&[("count", count.unwrap_or(20))]);

        let response = request.send().await?;
        let body = response.bytes().await?.to_vec();
        let fyyd_response: FyydResponse<FyydPodcast> = serde_json::from_slice(&body)?;

        Ok(fyyd_response)
    }
}
