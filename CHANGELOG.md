# Releases

## 0.9.0 — Rook Rainbow Quest

- Created an independent Rook Rainbow Quest repository from the complete v0.8.4 game. The original repository and its Git history stay untouched. Sixty level IDs, board layouts, artwork, mechanics and unlimited retries are preserved.
- Renamed the Rust package/crate, executable, native launcher, resource metadata, window title, both-language menu/map branding, packaging, CI artifacts and current documentation. Progress now lives in the separate RookRainbowQuest folder.
- Added Git LFS policies for binary assets/releases, automatic asset checks for builds/tests, CI LFS checkout/pointer checks, and a local prepare-check-in script that stages real LFS pointers without committing or pushing.
- Built a fresh Windows release in dist with native launcher, ZIP, licenses and checksums. Added play/build/LFS/migration documentation.

The entries below describe the predecessor project's releases and retain their historical names and paths.

## 0.8.4 — Let the first pawn finish its adventure

- Enabled promotion in World 1 level 7. After capturing c3, the pawn may advance through c4 to c5, become a queen and return to capture b4. The solvable detour no longer triggers an early retry. The direct three-move solution, board layout and lesson difficulty requirement are unchanged.
- Added session and native UI regressions for the detour, promotion, final capture, completion star timing and restart. Genuine blocked-pawn dead ends still offer a retry.

## 0.8.3 — A friendly native launcher

- Added Play LittleChess.exe: a native Rust/Windows launcher with no console window, a smiling rook icon and a bilingual error dialog when game files are missing. It supports both the stable repository entry point and extracted ZIP layout, resolves paths from its own location, and forwards game arguments.
- Embedded multi-size icons and Windows version metadata in both executables; the game window/taskbar uses the same original icon. Window title now reflects the actual package version.
- Packaging includes the launcher in the ZIP and at the top of dist, verifies it before publishing, and includes its checksum. Added missing-file, publication-failure, GUI-subsystem, icon/version, path and native handoff checks. Existing CMD remains a compatibility fallback.

## 0.8.2 — Queen helpers for the first checkmates

- Added a friendly queen on f1 to World 2 levels 1 and 2, alongside the existing two rooks, against a lone king. Later levels retain their armies.
- Regenerated and verified the hint routes: two and three player moves to checkmate, previously four and five. Updated individual descriptions, world guide and Swedish/English helper text.
- Added campaign assertions for the opening armies, safe starting positions and short teaching routes.
- Added distinct original backgrounds for Worlds 2 and 3: a peach castle courtyard and a lavender/gold palace above the clouds. Both map and match views use the selected world's painting. Textures are embedded, filtered and uploaded once before play; the meadow and galaxy artwork stay in place.

## 0.8.1 — Recognize dead ends

- Added conservative dead-position checks to World 1 capture matches: enemy bishops on an unreachable color when only friendly bishops remain, and both teams completely blocked.
- Added threefold repetition and 100 quiet-ply draw limits, keeping brave play on the second repetition. Captures and pawn moves reset the quiet limit. Rook-versus-rook remains winnable and playable.
- Added Swedish/English draw reasons and unlimited retry, without losing earned stars or awarding a false win. Worlds 2/3 and all sixty layouts remain unchanged.
- Added rule regressions and a native bilingual dead-position/restart test. See docs/DRAW-RULES.md for exact scope and limitations.

## 0.8.0 — Royal Chess Kingdom

- Added World 3 with twenty separate 8×8 regular-chess matches, kings on both teams, growing armies and a shrinking material advantage. The last board is the full standard starting position, marked with a castle icon and FINAL BOSS label.
- Added an isolated cozy-chess adapter for legal checks/pins, pawn double steps, en passant, castling rights and four-choice promotion. Both kings are protected. Player checkmate wins; loss/draw offers unlimited retry. Both sides automatically claim available threefold/fifty-move draws; insufficient material and stalemate also draw.
- Added bounded gentle opponent search, verified opening guides and Swedish/English rule tips, promotion picker and draw text. Three-player-move opening examples are not automatically awarded wins.
- Added perft and special-move/king-safety/promotion/draw tests, World 3 progression/finale checks and native regular-chess scenarios. Expanded the native campaign to sixty boards: forty original winning routes plus twenty new legal openings, preserving earned-star semantics.
- Added matching descriptions for all twenty new levels and docs/WORLD-3.md. Original forty JSON/Markdown files are unchanged, and their previous rule branches remain in use. No save reset is needed.
- Added the MIT-licensed cozy-chess/cozy-chess-types dependencies and matching upstream license fallback; dependencies remain Cargo-managed and the executable self-contained.

## 0.7.2 — Room to checkmate and worlds to celebrate

- Made all twenty World 2 boards 6×6. Regenerated the winning examples and updated every paired level description; the first two lone-king lessons now have four/five-move examples. Added a campaign assertion enforcing the shared size.
- Named the worlds Sunflower Meadows / Solrosängarna and Checkmate Castle / Schackmattslottet; named the universe selector Adventure Galaxy / Äventyrsgalaxen. Stable IDs and existing earned progress are preserved.
- Completing all levels in a world opens a localized celebration naming that world, with a Back to Adventure Galaxy button. Enter and Escape provide the same return. Access overrides do not award completion; replaying a completed finale celebrates again. Added completion/profile/reset tests and native campaign checks of both celebration buttons and screenshots.

## 0.7.1 — A safer opening and a parent unlock shortcut

- Moved World 1 level 14’s rook from a1 to d1, off the enemy bishop’s diagonal and out of its own pieces’ blockade. Regenerated its four-move winning route and updated its description. Tests verify rook mobility, its initially safe position and survival after a pawn-opening reply.
- Added localized Settings → Unlock all levels. Access to every level and both worlds persists for the selected profile, without fake stars or effects on the other profile. Confirmed New Game clears this override; older saves default to normal progression. Added save, bounds, profile-isolation, reset and native settings-button coverage.
- Reviewed the application, dependency integrity and build scripts after the first candidate was quarantined. A fresh optimized build passed pre/post Defender scans, all forty native levels and six native regressions from staging and its final repository location. The previous binary remains explicitly blocked by hash; the historical detection has no Microsoft determination. See docs/RELEASE-REVIEW-v0.7.1.md.
- Packaging now stages and validates before publishing, checks known quarantined hashes and Defender real-time events, and generates executable/ZIP checksums. Added three isolated packaging regressions. The v0.7.1 executable, licenses, ZIP and hashes are eligible for check-in under root ignore exceptions.

## 0.7.0 — The Checkmate Kingdom

- Added World 2 with twenty individually documented boards, distinct king icons and verified winning routes. It unlocks after World 1; each trail numbers its own levels 1–20. Existing IDs, saves and stars are retained.
- Added a protected opponent king, legal check responses, checkmate victory and a distinct stalemate retry. Players never have kings. Enemy teams grow gradually while the player retains a clear material advantage. Pawn promotion, gentle captures/oversights, brave play, free retries and explicit piece selection remain.
- Added king-safety, checkmate/stalemate, promotion-mate, pinned defender, hints, material, board-uniqueness and world-unlock regressions. The native campaign crosses both worlds; a native checkmate regression checks stalemate, retry and a mate win.
- Added high-precision texture coordinates for text so a shared font atlas stays crisp. Prepared glyphs before submission, grouped text draws and reset the atlas on display-scale changes. The final native profiling p95 improved from 240 ms before the performance work to 38 ms, with occasional stalls remaining.
- Retained release debug information for diagnostic traceability after a stripped candidate was quarantined by Defender. Packaging/native test scripts now honor custom Cargo target directories.
- Root .gitignore now excludes archive directories and ZIPs; removed prior checked-in level archives and made unit fixtures independent of them. Git history retains earlier versions.

## 0.6.0 — Brave rivals and a smoother adventure

- Opponents become brave after 2–4 quiet rounds, sooner with their last piece, and offer legal capture opportunities until a capture. Repeated positions trigger bravery rather than retry. Added the two-rooks-versus-one-rook regression and regenerated all match witnesses.
- Added opt-in native before/after profiling. Moved disk saves to a coalescing worker with error reporting and bounded shutdown flushing; cached initial sessions, high-resolution piece/mascot artwork and common text glyphs. Removed unnecessary per-frame cursor-release calls. Kept native-resolution text and full background quality.
- Added an original cheerful universe background specifically for the top-level world selector, using built-in image generation. The inner meadow remains.
- Retained versioned packages and archived v0.5 level descriptions/data. Updated per-level opponent settings and tuning notes.

## 0.5.0 — Quiet moves, promotion and a world of adventures

- Redesigned the ten lessons to require one/two extra moves in each pair, validated against the exact shortest solution. The second pawn lesson requires promotion to return for an otherwise unreachable target.
- Added automatic queen promotion for both sides of matches, at least one friendly pawn in every match, and two/three in later matches. Regenerated all winning examples.
- Match turns start with no selection; hints only highlight the source/destination. Restart and opponent passes also leave selection empty.
- Added localized Continue/New Game/Quit and a Yes/No reset confirmation. Continue availability persists even before the first star; New Game resets only the selected adventure.
- Added an expandable world map, currently with one world containing all twenty levels, plus arrow navigation and return buttons.
- Preserved older version folders and archived v0.4 level designs. Extended unit and native regressions for difficulty, promotion, selection, menu/reset/save behavior and future world expansion.

## 0.4.0 — Five pieces and friendly matches

- Rebuilt the campaign as two lessons each for rooks, bishops, knights, pawns and queen, followed by ten alternating-turn capture matches. Every level has its own board description, icon, tuning data and tested winning example.
- Added selectable friendly teams, distinct cream/purple pieces, turn indicators and a gentle deterministic opponent with gradually stronger capture/threat choices and periodic oversights. No kings, check, checkmate or pawn promotion.
- Removed undo throughout. Failed/stuck levels offer unlimited retry; stars and unlocked levels stay safe, with no lives or full game over.
- Preserved Swedish/English settings, native-resolution rendering and deferred shutdown. Archived version 0.3 level designs; retained the first two IDs and assigned new IDs to redesigned challenges.
- Added turn, opponent, pawn/queen, failure/retry and campaign tests, plus a native failure-screen retry regression.

## 0.3.0 â€” Swedish, clear settings and reliable exit

- Swedish is the default throughout the game. Settings can switch immediately between Svenska and English; the choice persists without changing progress IDs or stars.
- Settings and gameplay render directly to the native window viewport, avoiding the fullscreen image-sampling pass. Font raster size accounts for display scale and Windows DPI once. Translated buttons fit their labels.
- Exit Game requests shutdown after presentation rather than calling process exit inside drawing code. Audio is stopped and preferences are saved before the normal window loop ends.
- Added translation, save-compatibility, font-sizing, camera-orientation and shutdown tests, plus timed native settings/exit regressions in real processes.
- Versioned application directories and ZIP files are retained; the stable launcher points to the current packaged version.

## 0.2.0 â€” Twenty-level adventure

- Added 20 individually described rook, bishop and knight capture levels, sequential unlocking, arrow-key avatar movement and a scrolling world map.
- Added physical-resolution rendering and a 4K rendering regression, per-player map cursors and compatibility with the original saved star.

## 0.1.0 â€” First playable prototype

- Native Windows meadow menu and one 4Ã—4 rook puzzle, with hints, undo, sound, animation and two local player profiles.
