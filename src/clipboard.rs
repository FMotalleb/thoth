use anyhow::Result;
use arboard::{Clipboard, Error};
#[cfg(target_os = "linux")]
use std::env;
#[cfg(target_os = "linux")]
use std::process;

#[cfg(target_os = "linux")]
use crate::DAEMONIZE_ARG;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;

pub struct EditorClipboard {
    clipboard: Clipboard,
}

impl EditorClipboard {
    pub fn new() -> Result<EditorClipboard, Error> {
        Clipboard::new().map(|c| EditorClipboard { clipboard: c })
    }

    pub fn try_new() -> Option<EditorClipboard> {
        Self::new().ok()
    }

    pub fn set_contents(&mut self, content: String) -> Result<(), Error> {
        #[cfg(target_os = "linux")]
        {
            if env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG) {
                self.clipboard.set().wait().text(content)?;
            } else {
                process::Command::new(env::current_exe().unwrap())
                    .arg(DAEMONIZE_ARG)
                    .arg(content)
                    .stdin(process::Stdio::null())
                    .stdout(process::Stdio::null())
                    .stderr(process::Stdio::null())
                    .current_dir("/")
                    .spawn()
                    .map_err(|_e| arboard::Error::ContentNotAvailable)?;
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            self.clipboard.set_text(content)?;
        }

        Ok(())
    }

    pub fn get_content(&mut self) -> Result<String, Error> {
        self.clipboard.get_text()
    }

    #[cfg(target_os = "linux")]
    pub fn handle_daemon_args() -> Result<(), Error> {
        if env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG) {
            if let Some(content) = env::args().nth(2) {
                let mut clipboard = Self::new()?;
                clipboard.set_contents(content)?;
                std::process::exit(0);
            }
        }
        Ok(())
    }
}
