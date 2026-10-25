# Independent renamed repository

Rook Rainbow Quest v0.9.0 derives from the complete v0.8.4 game in `C:\Users\Admin\git\chessKids`. It lives in the separate `C:\Users\Admin\git\rook-rainbow-quest` repository, whose existing origin is `https://github.com/josefholmner/rook-rainbow-quest.git`. The original repository, history, working files and releases are preserved.

Copied: all Rust code/build configuration, sixty level JSON files and their individual descriptions, three-world data, original artwork/icons/fonts, licenses/notices, build/test/packaging helpers and current design documentation. The original `.git`, build cache, installed toolchains, old release folders and archives were not copied. The new repository keeps its own initial commit and remote.

The product name is **Rook Rainbow Quest** in both Swedish and English. Rust uses `rook-rainbow-quest` / `rook_rainbow_quest`; packaged executables use `RookRainbowQuest.exe` and `Play Rook Rainbow Quest.exe`. The stable native launcher, its Windows metadata and the game's window/menu/map branding all use the new identity. The original smiling-rook icon remains appropriate and is retained.

New progress is stored at `%LOCALAPPDATA%\RookRainbowQuest\progress.json`. The predecessor's `%LOCALAPPDATA%\LittleChessAdventure\progress.json` is not read or overwritten automatically. To reuse previous progress, close both games and copy that JSON to the new folder; save schema and stable level/world IDs are unchanged. Doing so is optional and separate from the source migration.

Older changelog entries and the original concept-art prompt retain historical naming as provenance. Current usage/build documentation and runtime code use the new name. Latest local verification is recorded in TEST-RESULTS.md. Git LFS policy and check-in preparation are documented in LFS.md; no commit or push is performed by the preparation script.
