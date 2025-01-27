use rspotify::{
    model::{playlist::*, Market, PrivateUser},
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
    // async fn test_playback() {
    //     let client = oauth_client().await;
    //     let uris = [
    //         PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
    //         PlayableId::Track(TrackId::from_uri("spotify:track:2DzSjFQKetFhkFCuDWhioi").unwrap()),
    //         PlayableId::Episode(EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap()),
    //     ];
    //     let devices = client.device().await.unwrap();

    //     // Save current playback data to be restored later
    //     // NOTE: unfortunately it's impossible to revert the entire queue, this will
    //     // just restore the song playing at the moment.
    //     let backup = client.current_playback(None, None::<&[_]>).await.unwrap();

    //     for (i, device) in devices.iter().enumerate() {
    //         let device_id = device.id.as_ref().unwrap();
    //         let next_device_id = devices
    //             .get(i + 1)
    //             .unwrap_or(&devices[0])
    //             .id
    //             .as_ref()
    //             .unwrap();

    //         // Starting playback of some songs
    //         client
    //             .start_uris_playback(
    //                 uris.iter().map(PlayableId::as_ref),
    //                 Some(device_id),
    //                 Some(Offset::Position(chrono::Duration::zero())),
    //                 None,
    //             )
    //             .await
    //             .unwrap();

    //         for i in 0..uris.len() - 1 {
    //             client.next_track(Some(device_id)).await.unwrap();

    //             // Also trying to go to the previous track
    //             if i != 0 {
    //                 client.previous_track(Some(device_id)).await.unwrap();
    //                 client.next_track(Some(device_id)).await.unwrap();
    //             }

    //             // Making sure pause/resume also works
    //             let playback = client.current_playback(None, None::<&[_]>).await.unwrap();
    //             if let Some(playback) = playback {
    //                 if playback.is_playing {
    //                     client.pause_playback(Some(device_id)).await.unwrap();
    //                     client.resume_playback(None, None).await.unwrap();
    //                 } else {
    //                     client.resume_playback(None, None).await.unwrap();
    //                     client.pause_playback(Some(device_id)).await.unwrap();
    //                 }
    //             }
    //         }

    //         client
    //             .transfer_playback(next_device_id, Some(true))
    //             .await
    //             .unwrap();
    //     }

    //     // Restore the original playback data
    //     if let Some(backup) = &backup {
    //         let uri = backup.item.as_ref().map(|item| item.id());
    //         if let Some(uri) = uri {
    //             let offset = None;
    //             let device = backup.device.id.as_deref();
    //             let position = backup.progress;
    //             client
    //                 .start_uris_playback(uri, device, offset, position)
    //                 .await
    //                 .unwrap();
    //         }
    //     }
    //     // Pause the playback by default, unless it was playing before
    //     if !backup.map(|b| b.is_playing).unwrap_or(false) {
    //         client.pause_playback(None).await.unwrap();
    //     }
    // }

    pub async fn play_music(&self) {
        let devices = self.api.device().await.unwrap();

        //     let devices = client.device().await.unwrap();
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
