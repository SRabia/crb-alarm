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

        // Running the requests
        // let history = spotify.current_playback(None, None::<Vec<_>>).await;
        // println!("Response: {history:?}");

        // let res_user = spotify.me().await;
        // match res_user {
        //     Ok(me) => {
        //         println!("me: {:?}", me);
        //     }
        //     Err(er) => {
        //         eprintln!("erro retrieving user info {:?}", er);
        //     }
        // }
        // let mut playlist: Vec<SimplifiedPlaylist> = Vec::new();

        // let mut playlists: Vec<SimplifiedPlaylist> = Vec::new();
        // let mut offset = 0;
        // let limit = 50; // Maximum playlists per request

        // loop {
        //     match spotify
        //         .current_user_playlists_manual(Some(limit), Some(offset))
        //         .await
        //     {
        //         Ok(page) => {
        //             playlists.extend(page.items);
        //             if playlists.len() >= page.total as usize {
        //                 break;
        //             }
        //             offset += limit;
        //         }
        //         Err(err) => {
        //             eprintln!("Error fetching playlists: {:?}", err);
        //             break;
        //         }
        //     }
        // }
        // for p in playlists {
        //     println!("Playlist: {}", p.name);
    }

    pub fn get_user_info(&self) -> PrivateUser {
        self.api.me().unwrap()
    }

    //TODO: remove, this is just for testing
    pub fn testing_shit(&self) -> String {
        "shit shit shit".to_string()
    }

    //TOTO: WIP this won't work need a tcp listener and tokio to make it non-blocking
    //while waiting for input from user
    //
    pub fn try_auth(&mut self) -> ClientResult<()> {
        let url = self.api.get_authorize_url(None).unwrap();
        match self.api.read_token_cache(true) {
            Ok(Some(new_token)) => {
                let expired = new_token.is_expired();

                *self.api.get_token().lock().unwrap() = Some(new_token);
                if expired {
                    match self.api.refetch_token()? {
                        Some(refreshed_token) => {
                            *self.api.get_token().lock().unwrap() = Some(refreshed_token)
                        }
                        None => {
                            let code = self.api.get_code_from_user(&url)?;
                            self.api.request_token(&code)?;
                        }
                    }
                }
            }
            _ => {
                let code = self.api.get_code_from_user(&url)?;
                self.api.request_token(&code)?;
            }
        }
        self.api.write_token_cache()
    }
}

impl Default for SpotiApi {
    fn default() -> Self {
        Self::new()
    }
}
