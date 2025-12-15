#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rustonic_shared::SubsonicApi;
use std::{error::Error, sync::Arc};
use tokio::runtime::Builder;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let tokio_runtime = Arc::new(
        Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime"),
    );

    let api = SubsonicApi::new(
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "Rustonic Desktop".to_owned(),
    )?;

    ui.on_request_song_list(move || {
        let api_clone = api.clone();
        let runtime_clone = tokio_runtime.clone();

        slint::spawn_local(async move {
            let handle = runtime_clone.spawn(async move {
                println!("Starting async ping...");
                api_clone.ping().await
            });

            match handle.await {
                Ok(Ok(())) => println!("Ping successful!"),
                Ok(Err(e)) => eprintln!("Ping failed: {}", e),
                Err(e) => eprintln!("Tokio task panicked: {}", e),
            }
        })
        .unwrap();
    });

    ui.run()?;

    Ok(())
}
