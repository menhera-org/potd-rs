
use crate::http_client::{
    HttpClient,
    HttpError,
};

/// Fetcher for POTD engine
#[derive(Debug)]
pub struct Fetcher {
    feed_url: String,
    client: HttpClient,
}

impl Fetcher {
    pub const DEFAULT_FEED: &'static str = "https://catfood.toolforge.org/catfood.php?category=Featured_pictures_on_Wikimedia_Commons";

    pub fn new() -> Fetcher {
        Fetcher {
            feed_url: Fetcher::DEFAULT_FEED.to_string(),
            client: HttpClient::new(),
        }
    }

    pub fn new_with_url(feed_url: &str) -> Fetcher {
        Fetcher {
            feed_url: feed_url.to_string(),
            client: HttpClient::new(),
        }
    }

    pub async fn fetch(&self) -> Result<String, HttpError> {
        self.client.fetch(&self.feed_url, true).await
    }

    pub fn fetch_blocking(&self) -> Result<String, HttpError> {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(self.fetch())
    }

    pub fn http_client(&self) -> HttpClient {
        self.client.clone()
    }
}
