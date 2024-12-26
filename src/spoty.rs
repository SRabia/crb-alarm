use rspotify::{
    model::PrivateUser, prelude::*, scopes, AuthCodePkceSpotify, Config, Credentials, OAuth,
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
        let mut spotify = AuthCodePkceSpotify::with_config(creds.clone(), oauth.clone(), config);

        // Obtaining the access token
        let url = spotify.get_authorize_url(None).unwrap();
        // This function requires the `cli` feature enabled.
        spotify.prompt_for_token(&url).unwrap();
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
}

impl Default for SpotiApi {
    fn default() -> Self {
        Self::new()
    }
}
