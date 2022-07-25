#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate tauri;
extern crate tokio;

use librespot::core::config::SessionConfig;
use tauri::{async_runtime::Mutex, Manager};
/// # Managed State
/// Controls how tauri will manage the state of the application
mod managed_state;

/// # Player
/// This will contain the code used to actually play music
///
/// - [ ] Make it exist
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
mod player;

/// # Scrobbler
/// allows the use of the Last.fm scrobbler
/// as of yet unimplemented
///
/// - [ ] Scrobble
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
mod scrobbler;

/// # MPRIS
/// control and display the current song using the MPRIS protocol
///
/// - [ ] Display Current Song
/// - [ ] Pause/Play
/// - [ ] Next Song
/// - [ ] Previous Song
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
mod mpris;

/// # Config
/// handles how config is written
/// needs its own module since it's going to be cross platform
///
/// - [ ] Windows (registry)
/// - [ ] MacOS (defaults)
/// - [ ] Linux (gsettings)
/// - [ ] Fallback (a ron file)
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
mod config;

/// # Commands
/// handles talking to the front-end and managing the state of the rest of the application
mod commands;

/// # API
/// handles connecting to spotify and doing all of the UI stuff
/// a similar tauri command will be added for most of these function in order to separate the data fetching from the UI and make it asynchronous
mod api;

/// # Entrypoint
/// Run the program
///
/// - [x] Spawn Window
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness

#[tokio::main]
async fn main() {
    // init exit signaller
    let (exit_sender, exit_receiver) = tokio::sync::broadcast::channel::<usize>(1);

    let exit_signaller = exit_sender;
    let exit_signaller_managed = managed_state::ExitSignaller(exit_signaller.clone());
    let exit_sig = exit_signaller.clone();

    let session_config = Mutex::new(SessionConfig::default());
    let session_config_managed = managed_state::SessionConf(session_config);

    // can now use tokio::select to manage the rest of these

    tauri::Builder::default()
        .manage(exit_signaller_managed)
        .manage(session_config_managed)
        .invoke_handler(generate_handler![commands::window::shutdown_window])
        .setup(move |app| {
            let ex = exit_sig;

            app.listen_global("quit", move |_event| {
                ex.send(0).ok();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
