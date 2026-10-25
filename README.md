# Rook Rainbow Quest

A colorful native Windows chess adventure for children aged 6–7. Version **0.9.0** has sixty individually described levels across three worlds, a Mario-style level trail, a smiling-rook mascot, hints, sound, animation and unlimited retries. The game starts in Swedish; Settings switches all UI text to English. The product name stays **Rook Rainbow Quest** in both languages.

## Play

Double-click **`dist/Play Rook Rainbow Quest.exe`**. It opens the current versioned release without a console window. Keep the launcher beside the release folder; create a desktop shortcut instead of moving it by itself.

For a portable copy, extract **`dist/RookRainbowQuest-0.9.0-Windows-x64.zip`** and open **`Play Rook Rainbow Quest.exe`** inside. The entire game, levels, fonts, artwork, icons and sounds are embedded. Windows 10/11 x64 is required; players need no installer, browser, Rust or internet connection. The ZIP includes license notices. `Play-RookRainbowQuest.cmd` is a compatibility fallback.

Arrows move between worlds and levels; Enter enters the selected stop. Use the mouse to select a piece and its destination. Tab/Shift+Tab and Enter activate buttons. M toggles sound; F11 toggles fullscreen. Continue becomes available after starting an adventure. New Game confirms before clearing the selected player's progress. Two player profiles have independent stars and settings include an Unlock all levels option that does not award stars.

There is no undo, lives counter or full game over. Draws, losses and genuine dead ends offer a free retry. Promotion detours remain playable, including World 1 level 7. Progress is saved in `%LOCALAPPDATA%/RookRainbowQuest/progress.json`.

## Three worlds

- **Sunflower Meadows:** ten single-piece capture lessons, then ten gentle turn-based capture matches without kings. Pawns promote; repetition, quiet-move and conservative dead-position rules prevent stalled matches.
- **Checkmate Castle:** twenty 6×6 checkmate matches. Only the opponent has a king. The first two lessons give the child two rooks and a queen against a lone king. The opponent must respect checks; stalemate permits retry.
- **Royal Chess Kingdom:** twenty 8×8 regular-chess matches with both kings, castling, pawn double moves, en passant and four-choice promotion. Armies become more even; the castle finale is a full starting position.

Worlds have distinct happy backgrounds; Adventure Galaxy has its own space painting. Each level's editable setup and design notes are paired in `levels/*.json` and `levels/*.md`. See [the campaign](docs/CAMPAIGN.md), [World 2](docs/WORLD-2.md), [World 3](docs/WORLD-3.md) and [draw rules](docs/DRAW-RULES.md).

## Build and test

Develop on Windows with Git for Windows/Git LFS and PowerShell 7. Setup installs missing Rust and Windows C++/SDK tools and downloads the locked dependencies and LFS assets:

```powershell
.\scripts\setup.ps1
.\scripts\run.ps1
.\scripts\test.ps1
.\scripts\package.ps1
```

`test.ps1 -Native` adds native game checks. Packaging always validates the actual optimized game and launcher before publishing. See [complete build instructions](docs/BUILD.md), [launcher details](docs/LAUNCHER.md), [asset provenance](docs/ASSETS.md) and [latest validation](docs/TEST-RESULTS.md).

## Git LFS and check-in

Binary assets and releases use Git LFS; source, level data and documentation remain normal Git text. The latest release folder, native launcher, ZIP, SHA-256 file and licenses are eligible for check-in. Old releases, archives, target and toolchains are excluded.

Use your usual Git client to stage, commit and push. The prepared `.gitattributes` applies LFS to binary files. The initial staged set was verified: all 17 binary files, including the release, contain matching LFS pointers, and the pre-push hook is present.

For an independent read-only audit:

```powershell
.\scripts\verify-lfs.ps1 -Staged
```

`prepare-check-in.ps1` is an optional helper for developers who want local LFS setup and staging automated; it does not commit or push. See [the LFS policy](docs/LFS.md). CI downloads LFS content and checks pointers before building.

This is an independent renamed repository. The original project is preserved; its saves are not changed. See [migration details](docs/MIGRATION.md), including optional progress-file reuse.
