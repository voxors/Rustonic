use crate::{
    subsonic::SubsonicApi,
    ui::{self, *},
};
use slint::ComponentHandle;

pub struct AppHandler {
    window: Option<AppWindow>,
}

impl AppHandler {
    pub fn new() -> Self {
        Self { window: None }
    }

    pub fn initialize_ui(&mut self) {
        let main_window = ui::AppWindow::new().unwrap();

        let api = SubsonicApi::new(
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "Rustonic Desktop".to_owned(),
        )
        .unwrap();

        main_window.on_request_song_list(move || {
            let api_clone = api.clone();

            slint::spawn_local(async move {
                let _ = api_clone.ping().await;
            })
            .unwrap();
        });

        self.window = Some(main_window);
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        let window = self.window.as_ref().expect("Cannot access main window!");
        window.run()
    }
}
