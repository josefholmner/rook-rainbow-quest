#![cfg_attr(windows, windows_subsystem = "windows")]

use std::{
    ffi::OsString,
    io,
    path::Path,
    process::{Child, Command},
};

fn launch(executable: &Path, arguments: impl Iterator<Item = OsString>) -> io::Result<Child> {
    let folder = executable
        .parent()
        .ok_or_else(|| io::Error::other("Launcher folder is missing"))?;
    let versioned = folder
        .join(concat!("RookRainbowQuest-", env!("CARGO_PKG_VERSION")))
        .join("RookRainbowQuest.exe");
    let adjacent = folder.join("RookRainbowQuest.exe");
    let game = if versioned.is_file() {
        versioned
    } else if adjacent.is_file() {
        adjacent
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!(
            "Spelfilen saknas / Game file is missing:\n{}\n{}\n\nBehåll startprogrammet med spelfilerna.\nKeep the launcher with the game files.", versioned.display(), adjacent.display()
        )));
    };
    Command::new(&game)
        .current_dir(game.parent().unwrap())
        .args(arguments)
        .spawn()
}

#[cfg(windows)]
fn show_error(message: &str) {
    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(
            window: *mut std::ffi::c_void,
            text: *const u16,
            caption: *const u16,
            flags: u32,
        ) -> i32;
    }
    let wide = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
    let text = wide(message);
    let title = wide("Rook Rainbow Quest");
    // Synchronous Windows dialog; both UTF-16 buffers remain alive throughout.
    unsafe {
        MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), 0x10);
    }
}
#[cfg(not(windows))]
fn show_error(message: &str) {
    eprintln!("{message}");
}

fn main() {
    if let Err(error) = std::env::current_exe()
        .and_then(|path| launch(&path, std::env::args_os().skip(1)).map(|_| ()))
    {
        show_error(&format!(
            "Det gick inte att starta spelet.\nCould not start the game.\n\n{error}"
        ));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_game_returns_a_useful_error_without_starting_a_process() {
        let folder = std::env::temp_dir().join(format!(
            "rook-rainbow-quest-launcher-{}",
            std::process::id()
        ));
        let error = launch(
            &folder.join("Play Rook Rainbow Quest.exe"),
            std::iter::empty(),
        )
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        let message = error.to_string();
        assert!(message.contains("Game file is missing"));
        assert!(message.contains(concat!("RookRainbowQuest-", env!("CARGO_PKG_VERSION"))));
    }
}
