//! This is an example demonstrating the usage of the FyydClient from the fyyd-api crate.
//! It performs a search for podcasts related to the term "rust" and prints their details.
use fyyd_api::v2::fyyd_api::FyydPodcastSearch;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {
    // Create a new instance of a FyydClient
    let client_builder = ClientBuilder::new().timeout(std::time::Duration::from_secs(10));

    // Perform a search for a podcast feed
    let search_result = FyydPodcastSearch::default()
        .client_builder(client_builder)
        .term("rust")
        .run()
        .await;

    match search_result {
        Ok(response) => {
            if let Some(podcasts) = response.data {
                for podcast in podcasts {
                    println!("Title: {}", podcast.title);
                    println!("URL: {}", podcast.xml_url);
                    println!("Description: {}", podcast.description);
                    println!("---");
                }
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
