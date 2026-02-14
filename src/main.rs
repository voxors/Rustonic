#![cfg(any(target_os = "linux", target_os = "windows", target_arch = "wasm32",))]

mod app_main;
mod music_player;
mod subsonic;
pub mod ui {
    slint::include_modules!();
}
use crate::app_main::AppHandler;

#[cfg(any(target_os = "linux", target_os = "windows"))]
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), slint::PlatformError> {
    env_logger::Builder::default()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    let mut app_handler = AppHandler::new();
    app_handler.initialize_ui()?;

    app_handler.run()
}

#[cfg(target_arch = "wasm32")]
#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init_with_level(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    })
    .ok();

    let mut app_handler = AppHandler::new();
    app_handler.initialize_ui().unwrap();

    let res = app_handler.run();

    match res {
        Ok(()) => {}
        Err(e) => {
            log::error!("Runtime error: {}", e);
        }
    }
}
