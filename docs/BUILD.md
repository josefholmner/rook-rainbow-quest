# Build Rook Rainbow Quest

The game is a native 64-bit Windows application built in Rust with Macroquad. It embeds all level data, artwork, fonts, icons and synthesized audio. Players do not need a compiler, browser, installer or network connection.

## Developer setup

Use Windows 10/11 x64, Git for Windows including Git LFS, and PowerShell 7. From a clone of this repository:

```powershell
.\scripts\setup.ps1
```

Setup configures Git LFS locally and downloads the real binary assets, installs missing Windows C++/SDK build tools and an isolated Rust toolchain, adds rustfmt/clippy, and fetches the locked dependencies. Installing missing C++ tools may require Windows administrator consent or a restart. Existing compatible tools are reused. No manual library downloads are required. Tooling is stored in ignored `.tools/`; `ROOK_RAINBOW_QUEST_TOOLCHAIN` optionally selects an existing isolated toolchain.

## Run, test and package

```powershell
.\scripts\run.ps1
.\scripts\test.ps1
.\scripts\package.ps1
```

Tests run the LFS asset audit, formatting, pure Rust/renderer/launcher tests, clippy and packaging regressions. `test.ps1 -Native` additionally runs the actual game campaign and native regressions. Packaging always performs a release build and native validation before replacing the launcher or publishing a ZIP.

Release output:

- `dist/Play Rook Rainbow Quest.exe`: stable entry point above the versioned folder.
- `dist/RookRainbowQuest-0.10.1/`: game, adjacent launcher, play instructions and licenses.
- `dist/RookRainbowQuest-0.10.1-Windows-x64.zip`: portable player download.
- `dist/RookRainbowQuest-0.10.1.sha256`: executable, launcher and ZIP hashes.

Packaging rejects known quarantined hashes, checks Microsoft Defender before/after actual native runs, and verifies the launcher GUI subsystem, icon/version resources and path handling. Validation logs/screenshots stay under ignored `target/package-staging/`. It does not weaken Windows protection or sign the executables. See TEST-RESULTS.md for the latest local evidence; local checks do not imply a remote GitHub Actions run.

When releasing a new version, update Cargo.toml/Cargo.lock and the latest-release exceptions in `.gitignore`, rebuild with package.ps1, then prepare the check-in using LFS.md. Existing release filenames are generated from Cargo's version. Do not check in target, toolchains, older releases or source archives.
