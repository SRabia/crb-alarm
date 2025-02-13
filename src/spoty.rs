use rspotify::{
    model::{device, playlist::*, Market, PrivateUser},
    prelude::*,
    scopes, AuthCodePkceSpotify, ClientResult, Config, Credentials, OAuth,
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
        //TODO: if this fail we have to somehow say that we need the creds
        //maybe inject the creds instead!
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

    pub async fn get_user_info(&self) -> Option<PrivateUser> {
        let user = self.api.me().await;
        match user {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }
    pub async fn get_user_playlist(&self) -> ClientResult<Vec<SimplifiedPlaylist>> {
        let limit = 50;
        let mut offset = 0;
        let mut playlist = vec![];
        loop {
            let page = self
                .api
                .current_user_playlists_manual(Some(limit), Some(offset))
                .await?;

            for item in page.items {
                playlist.push(item);
                // println!("* {}", item.track.name);
            }

            // The iteration ends when the `next` field is `None`. Otherwise, the
            // Spotify API will keep returning empty lists from then on.
            if page.next.is_none() {
                break;
            }
            offset += limit;
        }
        Ok(playlist)
    }

    pub async fn play_music(&self, playlist: &SimplifiedPlaylist) {
        let devices = self.api.device().await.unwrap();
        //TODO:: requires premium..
        println!("devices : {:?}", devices);
        if let Some(first_dev_avail) = devices.iter().find(|d| d.is_active) {
            println!("hhhhhhhhh    feofe");
            self.api
                .start_context_playback(
                    PlayContextId::Playlist(playlist.id.clone()),
                    first_dev_avail.id.as_deref(),
                    None,
                    // Some(rspotify::model::Offset::Position(Duration::ZERO)),
                    None,
                )
                .await
                .unwrap(); //TODO:can't unwrap here if we get http err
            self.api
                .transfer_playback(first_dev_avail.id.as_deref().unwrap(), Some(true))
                .await
                .unwrap(); //TODO: can't unwrap!
        }
    }

    pub async fn get_playlist_track(
        &self,
        playlist: &SimplifiedPlaylist,
    ) -> ClientResult<Vec<PlaylistItem>> {
        let tracks = self
            .api
            .playlist(playlist.id.clone(), None, Some(Market::FromToken))
            .await?;

        Ok(tracks.tracks.items.into_iter().collect())
    }

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
                // TODO: can't let this since it print directy to stdout!!
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
