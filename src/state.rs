use url::Url;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub apprise_url: Url,
    pub apprise_tags: String,
}

impl AppState {
    pub fn new() -> Option<Self> {
        let apprise_url = env::var("APPRISE_URL").ok()?;
        let apprise_url = Url::parse(&apprise_url).ok()?;
        
        // Get tags from environment variable or default to "all"
        let apprise_tags = env::var("APPRISE_TAGS").unwrap_or_else(|_| "all".to_string());
        
        Some(AppState {
            apprise_url,
            apprise_tags,
        })
    }
}
