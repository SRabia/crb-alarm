use rspotify::{
    model::PrivateUser, prelude::*, scopes, AuthCodePkceSpotify, ClientResult, Config, Credentials,
    OAuth,
};

#[derive(Debug)]
pub struct SpotiApi {
    pub api: AuthCodePkceSpotify,
}

impl SpotiApi {
    /// Creates a new [`SpotiApi`].
    pub fn new() -> Self {
        let config = Config {
            token_cached: true,
            cache_path: "./token_cache.json".into(),
            ..Default::default()
        };
        let creds = Credentials::from_env().unwrap();

        let scopes = scopes!(
            "playlist-read-collaborative",
            "playlist-read-private",
            // "playlist-modify-private",
            // "playlist-modify-public",
            "user-follow-read",
            // "user-follow-modify",
            // "user-library-modify",
            "user-library-read",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-playback-position",
            "user-read-private",
            "user-read-recently-played"
        );
        let oauth = OAuth::from_env(scopes).unwrap();
        let spotify = AuthCodePkceSpotify::with_config(creds.clone(), oauth.clone(), config);

        Self { api: spotify }
    }

    pub async fn get_user_info(&self) -> PrivateUser {
        self.api.me().await.unwrap()
    }

    //TODO: remove, this is just for testing
    pub fn testing_shit(&self) -> String {
        "shit shit shit".to_string()
    }

    //TOTO: WIP this won't work need a tcp listener and tokio to make it non-blocking
    //while waiting for input from user
    //
    pub async fn try_auth(&mut self) -> ClientResult<()> {
        let url = self.api.get_authorize_url(None).unwrap();
        match self.api.read_token_cache(true).await {
            Ok(Some(new_token)) => {
                let expired = new_token.is_expired();

                *self.api.get_token().lock().await.unwrap() = Some(new_token);
                if expired {
                    match self.api.refetch_token().await? {
                        Some(refreshed_token) => {
                            *self.api.get_token().lock().await.unwrap() = Some(refreshed_token)
                        }
                        None => {
                            let code = self.api.get_code_from_user(&url)?;
                            self.api.request_token(&code).await?;
                        }
                    }
                }
            }
            _ => {
                // this can potential prompt with stdin!
                let code = self.api.get_code_from_user(&url)?;
                self.api.request_token(&code).await?;
            }
        }
        self.api.write_token_cache().await
    }

    // fn get_code_from_user(&self, url: &str) -> ClientResult<String> {
    //     use rspotify::ClientError;

    //     // log::info!("Opening brower with auth URL");
    //     match webbrowser::open(url) {
    //         Ok(_) => println!("Opened {} in your browser.", url),
    //         Err(why) => eprintln!(
    //             "Error when trying to open an URL in your browser: {:?}. \
    //              Please navigate here manually: {}",
    //             why, url
    //         ),
    //     }

    //     // log::info!("Prompting user for code");
    //     println!("Please enter the URL you were redirected to: ");
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input)?;
    //     let code = self
    //         .api
    //         .parse_response_code(&input)
    //         .ok_or_else(|| ClientError::Cli("unable to parse the response code".to_string()))?;

    //     Ok(code)
    // }
}

impl Default for SpotiApi {
    fn default() -> Self {
        Self::new()
    }
}
