use crate::{
    music_player::MusicPlayer,
    subsonic::SubsonicApi,
    ui::{self, *},
};
use slint::ComponentHandle;
use std::error::Error;
use tokio::sync::Mutex;

#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
type MusicPlayerHandle = Rc<Mutex<MusicPlayer>>;
#[cfg(not(target_arch = "wasm32"))]
type MusicPlayerHandle = Arc<Mutex<MusicPlayer>>;

pub struct AppHandler {
    window: Option<AppWindow>,
    music_player: MusicPlayerHandle,
}

impl AppHandler {
    pub fn new() -> Self {
        Self {
            window: None,
            music_player: MusicPlayerHandle::new(Mutex::new(MusicPlayer::new())),
        }
    }

    pub fn initialize_ui(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let main_window = ui::AppWindow::new()?;

        let api = SubsonicApi::new(
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "Rustonic Desktop".to_owned(),
        )?;

        let music_play_clone = self.music_player.clone();

        main_window.on_request_song_list(move || {
            let api_clone = api.clone();
            let music_player_play_thread_clone = music_play_clone.clone();

            slint::spawn_local(async move {
                if let Err(err) = api_clone.ping().await {
                    log::error!("Failed to ping Subsonic API: {}", err);
                }
                if let Ok(stream) = api_clone.stream().await {
                    let _ = music_player_play_thread_clone
                        .lock()
                        .await
                        .play(stream)
                        .await;
                }
            })
            .ok();
        });

        self.window = Some(main_window);

        Ok(())
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        let window = self.window.as_ref().expect("Cannot access main window!");
        window.run()
    }
}
