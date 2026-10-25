# Rook Rainbow Quest v0.9.0 validation — 2026-10-25

- The new independent repository is C:\Users\Admin\git\rook-rainbow-quest, retaining its existing origin and initial commit. No original Git metadata, old releases, caches or toolchains were copied. No commit or push was made.
- Verified all 190 original core files against the pre-copy SHA-256 baseline. The original chessKids repository's HEAD is unchanged and its working tree is clean. All sixty new level JSON files match the original game byte for byte.
- Renamed Rust package/crate, game and launcher filenames, Windows icon/version resources, window title, both-language menu/map branding, package outputs, CI artifact names, test prefixes and build-tool override. A source/script/config scan found no old product naming. Historical changelog/concept provenance and migration references are explicitly labeled.
- Added a regression confirming progress is isolated under RookRainbowQuest/progress.json. The predecessor's family save is not read or overwritten automatically; optional reuse is documented. Stable level/world IDs and save schema remain unchanged.
- Formatting, 90 library tests, one renderer test and one launcher test passed (92 total), plus clippy with warnings denied and four packaging regressions.
- Final native campaign passed sixty boards: forty complete winning routes and twenty regular-chess opening guides. All eight game regression cases and three launcher handoff scenarios passed. This includes World 1 level 7's optional promotion detour and Worlds 2/3's distinct backgrounds. Tests use isolated saves.
- Visually inspected the renamed main menu in the running release. Native launcher metadata/icon tests passed. Original artwork, icons, fonts, mechanics and 60 level definitions are retained.
- Git LFS attribute/content audit passed for all 17 binary files, including artwork, concept image, fonts, raw RGBA textures, icons, game, both launcher copies and versioned ZIP. Source and license text remain normal Git files. The audit also detects unknown binary types rather than relying only on filename extensions.
- After the user staged the files with their own Git client, the strict read-only LFS audit passed for all 17 binary files: every staged pointer's SHA-256 and size matches its actual content. The LFS pre-push hook is present. Git management remains with the user. prepare-check-in.ps1 is an optional helper; CI independently checks out LFS content and enforces the pointer audit.
- Game and launcher passed pre/post Microsoft Defender scans and matching real-time detection checks. Published game, launcher and ZIP also explicitly scanned with no threats. Both ZIP executables match the published files. This is local validation, not a Microsoft analysis verdict; no security settings were changed.
- The versioned release folder, Windows ZIP, SHA-256 manifest, licenses and stable native launcher are eligible for check-in. Build/play/LFS instructions are in README.md, PLAY.txt, docs/BUILD.md, docs/LAUNCHER.md and docs/LFS.md. No remote CI run or upload is claimed.

Game SHA-256: F3B9AF7EB45C45F900BA2E106748222F458DC89C094200AF994D026A68594014

Launcher SHA-256: E5E72B1A34808087DD74CD03896C216749A939BEF89700FFF658567B323ED70E

ZIP SHA-256: C1BDEF18FEDF5D507B94C8DD052E90FD13D24F7CFEA32487115366A370666BD7

Raw evidence: target/package-staging/8a9ff18c46db4e389c01ff79fa2c0592/validation/ and launcher-validation/ (ignored). Original-file baseline and presentation copies are retained in the task's outputs/Rook-Rainbow-Quest folder, outside both repositories.
