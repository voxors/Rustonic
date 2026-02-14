#![cfg(target_os = "android")]

mod app_main;
mod music_player;
mod subsonic;
pub mod ui {
    slint::include_modules!();
}
use crate::app_main::AppHandler;
use core::cell::RefCell;
use log::error;
use std::rc::Rc;

#[unsafe(no_mangle)]
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn android_main(android_app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("Rustonic")
            .with_max_level(if cfg!(debug_assertions) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            }),
    );

    let app_handler = Rc::new(RefCell::new(AppHandler::new()));

    slint::android::init(android_app).unwrap();

    {
        let mut app_handler = app_handler.borrow_mut();
        app_handler
            .initialize_ui()
            .inspect_err(|err| error!("{}", err))
            .unwrap();
    }

    let app_handler = app_handler.borrow();
    app_handler.run().unwrap();
}
