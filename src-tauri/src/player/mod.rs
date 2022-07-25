use librespot::core::session::Session;
use librespot::playback::player::Player as LibrePlayer;
pub struct Player {
	session: Session,
	player: LibrePlayer
}