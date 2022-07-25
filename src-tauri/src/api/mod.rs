pub mod artist;

pub struct SpotifyAPIClient {
    token: Option<String>
}

impl SpotifyAPIClient {
    pub fn new() -> Self {
        SpotifyAPIClient { token: None }
    }

    pub fn new_init_with_auth_token(token: String) -> Self {
        SpotifyAPIClient { token: Some(token) }
    }
}
