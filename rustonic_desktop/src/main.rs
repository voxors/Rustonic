#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rustonic_shared::SubsonicApi;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let api = SubsonicApi::new(
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "Rustonic Desktop".to_owned(),
    )?;

    ui.on_request_song_list(move || {
        let api_clone = api.clone();

        slint::spawn_local(async_compat::Compat::new(
            async move { api_clone.ping().await },
        ))
        .unwrap();
    });

    ui.run()?;

    Ok(())
}
