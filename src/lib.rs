
mod fetcher;
pub mod parser;
pub mod http_client;

use std::fmt::Display;
use std::collections::VecDeque;

pub use fetcher::Fetcher;

/// Error type for the POTD engine
#[derive(Debug)]
pub enum Error {
    FetchError(http_client::HttpError),
    ParseError(parser::ParseError),
}

impl From<http_client::HttpError> for Error {
    fn from(e: http_client::HttpError) -> Self {
        Error::FetchError(e)
    }
}

impl From<parser::ParseError> for Error {
    fn from(e: parser::ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FetchError(e) => write!(f, "FetchError: {}", e),
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
        }
    }
}

/// Extension whitelist for POTD engine
#[derive(Debug)]
pub struct ExtensionWhitelist {
    extensions: Vec<&'static str>,
}

impl ExtensionWhitelist {
    pub fn new_with_default() -> ExtensionWhitelist {
        ExtensionWhitelist {
            extensions: vec![
                ".jpg",
                ".jpeg",
            ],
        }
    }

    pub fn new(extensions: Vec<&'static str>) -> ExtensionWhitelist {
        ExtensionWhitelist {
            extensions,
        }
    }

    pub fn is_whitelisted(&self, filename: &str) -> bool {
        let filename = filename.to_lowercase();
        let mut whitelisted = false;
        for ext in &self.extensions {
            if filename.ends_with(ext) {
                whitelisted = true;
                break;
            }
        }
        whitelisted
    }
}

impl Default for ExtensionWhitelist {
    fn default() -> Self {
        Self::new_with_default()
    }
}

/// Builder for POTD engine
#[derive(Debug)]
pub struct EngineBuilder {
    target_width: usize,
    fetcher: Option<Fetcher>,
    extension_whitelist: Option<ExtensionWhitelist>,
}

impl EngineBuilder {
    /// Create a new builder with the target image width
    pub fn new(mut target_width: usize) -> EngineBuilder {
        if target_width > 3840 {
            log::warn!("Target width is too large, setting to 3840"); // 4K
            target_width = 3840;
        }
        EngineBuilder {
            target_width,
            fetcher: None,
            extension_whitelist: None,
        }
    }

    pub fn fetcher(mut self, fetcher: Fetcher) -> EngineBuilder {
        self.fetcher = Some(fetcher);
        self
    }

    pub fn extension_whitelist(mut self, extension_whitelist: ExtensionWhitelist) -> EngineBuilder {
        self.extension_whitelist = Some(extension_whitelist);
        self
    }

    pub fn build(self) -> Engine {
        let fetcher = self.fetcher.unwrap_or_else(|| Fetcher::new());
        Engine {
            target_width: self.target_width,
            fetcher,
            extension_whitelist: self.extension_whitelist.unwrap_or_default(),
        }
    }
}

/// Engine for POTD
pub struct Engine {
    target_width: usize,
    fetcher: Fetcher,
    extension_whitelist: ExtensionWhitelist,
}

impl Engine {
    /// Shortcut to create a new engine with the target image width
    pub fn new(target_width: usize) -> Engine {
        EngineBuilder::new(target_width).build()
    }

    pub fn fetcher(&self) -> &Fetcher {
        &self.fetcher
    }

    /// Run the POTD engine in blocking mode, returning a list of URLs for the POTD images
    pub fn run_blocking(&self) -> Result<Vec<String>, Error> {
        let body = self.fetcher.fetch_blocking()?;
        log::debug!("Fetched body ({} bytes)", body.len());
        let raw_urls = parser::parse(&body)?;
        Ok(raw_urls
            .iter()
            .filter_map(|url| {
                let mut parts = url.split("/").collect::<Vec<&str>>();
                let filename = parts.pop().unwrap();
                if !self.extension_whitelist.is_whitelisted(filename) {
                    log::info!("Skipping non-whitelisted file: {}", filename);
                    return None;
                }
                let mut filename_parts = filename.split("px-").collect::<VecDeque<&str>>();
                if filename_parts.len() < 2 {
                    log::warn!("Invalid filename: {}", filename);
                    return None;
                }
                filename_parts.pop_front();
                let filename = format!("{}px-{}", self.target_width, filename_parts.iter().map(|s| *s).collect::<Vec<&str>>().join("px-"));
                parts.push(&filename);
                Some(parts.join("/"))
            })
            .collect())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use env_logger;

    #[test]
    fn fetch_and_parse() {
        env_logger::init();

        let engine = Engine::new(640);
        let urls = engine.run_blocking().unwrap();
        let http_client = engine.fetcher().http_client();
        log::info!("Fetched {} URLs", urls.len());

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                for url in urls {
                    log::info!("Fetching {}", url);
                    let _ = http_client.fetch_bytes(&url, true).await.unwrap();
                }
            });
    }
}
