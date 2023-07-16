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

/// Search for podcasts inside fyyd's database.
///
/// # Example
///
/// ```rust
/// use fyyd_api::v2::fyyd_api::FyydPodcastSearch;
/// use reqwest::ClientBuilder;
///
/// # async fn doctest() {
/// let client_builder = ClientBuilder::new().timeout(std::time::Duration::from_secs(10));
/// let result = FyydPodcastSearch::default()
///     .client_builder(client_builder)
///     .term("rust")
///     .run()
///     .await
///     .expect("Failed to search");
/// # }
/// ```

#[derive(Default)]
pub struct FyydPodcastSearch {
    term: Option<String>,
    title: Option<String>,
    language: Option<String>,
    url: Option<String>,
    page: Option<u16>,
    client_builder: Option<ClientBuilder>,
}

impl FyydPodcastSearch {
    /// `term` - a search term inside the podcast
    pub fn term(mut self, term: impl ToString) -> Self {
        self.term = Some(term.to_string());
        self
    }

    /// `title` - the podcast's title
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// The language of the podcast
    pub fn language(mut self, language: impl ToString) -> Self {
        self.language = Some(language.to_string());
        self
    }

    /// `url` - the podcast's url
    pub fn url(mut self, url: impl ToString) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// `page` - the page of the search,
    ///       WARNING: if the page is overshot it will still return valid pages
    pub fn page(mut self, page: u16) -> Self {
        self.page = Some(page);
        self
    }

    pub fn client_builder(mut self, client_builder: ClientBuilder) -> Self {
        self.client_builder = Some(client_builder);
        self
    }

    /// Search for a podcast feed inside fyyd's database,
    /// matches any - or some set of a given critetia.
    pub async fn run(self) -> Result<FyydResponse<Vec<FyydPodcast>>, Box<dyn std::error::Error>> {
        let Self {
            title,
            url,
            term,
            language,
            page,
            client_builder,
        } = self;

        let client = client_builder.unwrap_or_else(ClientBuilder::new).build()?;

        let path = FYYD_API_V2.to_owned() + SEARCH_PODCAST;
        let request = client
            .get(path)
            .query(&[("title", title.unwrap_or_default())])
            .query(&[("url", url.unwrap_or_default())])
            .query(&[("language", language.unwrap_or_default())])
            .query(&[("term", term.unwrap_or_default())])
            .query(&[("page", page.as_ref().cloned().unwrap_or_default())]);

        let response = request.send().await?;
        let body = response.bytes().await?.to_vec();
        let fyyd_response: FyydResponse<Vec<FyydPodcast>> = serde_json::from_slice(&body)?;

        Ok(fyyd_response)
    }
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
    pub fn new(client_builder: Option<ClientBuilder>) -> Self {
        let client = client_builder
            .unwrap_or_else(ClientBuilder::new)
            .build()
            .unwrap();
        FyydClient { client }
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
