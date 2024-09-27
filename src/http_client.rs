
/// Error type for HTTP client
#[derive(Debug)]
pub enum HttpError {
    NetworkError(reqwest::Error),
    StatusCode(reqwest::StatusCode),
}

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        HttpError::NetworkError(e)
    }
}

impl From<reqwest::StatusCode> for HttpError {
    fn from(e: reqwest::StatusCode) -> Self {
        HttpError::StatusCode(e)
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::NetworkError(e) => write!(f, "NetworkError: {}", e),
            HttpError::StatusCode(e) => write!(f, "StatusCode: {}", e),
        }
    }
}

/// HTTP client for POTD engine
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("Mozilla/5.0 (compatible; potd-rs/{}; +https://github.com/menhera-org/potd-rs)", version);
        HttpClient {
            client: reqwest::Client::builder()
                .user_agent(user_agent)
                .build()
                .unwrap(),
        }
    }

    pub async fn fetch(&self, url: &str, expect_ok: bool) -> Result<String, HttpError> {
        let res = self.client.get(url).send().await?;
        if expect_ok && res.status() != 200 {
            return Err(res.status().into());
        }
        let body = res.text().await?;
        Ok(body)
    }

    pub async fn fetch_bytes(&self, url: &str, expect_ok: bool) -> Result<Vec<u8>, HttpError> {
        let res = self.client.get(url).send().await?;
        if expect_ok && res.status() != 200 {
            return Err(res.status().into());
        }
        let body = res.bytes().await?;
        Ok(body.to_vec())
    }
}