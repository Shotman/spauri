#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

/// # Player
/// This will contain the code used to actually play music
/// 
/// - [ ] Make it exist
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
mod player;

/// # Runtime
/// Will wrap around the tauri runtime so i can in future wrap it around the Tokio runtime maybe
/// 
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
mod runtime;

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

/// # Entrypoint
/// Run the program
/// 
/// - [x] Spawn Window
/// - [ ] Wrap in Tokio Runtime for multithreaded goodness
fn main() {
  runtime::execute();
}
