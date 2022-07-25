use tauri::async_runtime::Mutex;
use tokio::sync::broadcast::{Sender,error::SendError};

use librespot::core::config::SessionConfig;

#[derive(Debug, Clone)]
pub struct ExitSignaller(pub Sender<usize>);

impl ExitSignaller {
	pub fn send(&self, exit_code: usize) -> Result<usize, SendError<usize>> {
		self.0.send(exit_code)
	}
}

pub struct SessionConf(pub Mutex<SessionConfig>);