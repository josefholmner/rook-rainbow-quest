# Rook Rainbow Quest — first playable design

Status: design proposal, 23 October 2026. This delivery contains a plan and visual concept; it is not yet a playable application.

## Scope

Build a native Windows desktop game for children aged 6–7. The first version contains one world-map main menu and exactly one playable level. Aim for a warm, colorful game that feels like a small adventure. Future level locations can appear as scenery, but must clearly say “Coming later”; there is no second level to unlock in this prototype.

Use an original world and characters, with the connected paths and sense of progress familiar from classic platform-game maps. Do not use Mario characters, music, or artwork.

## Language and delivery

Recommend **Rust + Macroquad**, a small 2D game framework. Native window, native rendering and audio; no browser, webview, web server, account, or online service is needed to play. This scope does not need Bevy's larger engine architecture.

Use Cargo-managed dependencies with pinned versions and a checked-in Cargo.lock. Start with Macroquad (graphics, window, input, audio) and serde/serde_json (editable levels and local progress). Avoid an external chess engine for the first stationary capture puzzle.

The family-facing deliverable is a Windows ZIP containing `RookRainbowQuest.exe`, with graphics, level data and sounds embedded in the executable. Extract and double-click. No Rust installation or dependency downloads for players. The first release should target this PC's architecture; verify that architecture before building.

The developer workflow is `cargo run` and `cargo build --release` once the Rust toolchain is available. Provide a checked-in PowerShell setup script that detects the environment and handles prerequisite installation, with elevation only if Windows requires it. Cargo downloads library dependencies automatically. A clean-PC build and launch check is a release gate; do not call setup automatic until it is verified. Rust and Cargo were not found on PATH during planning; the toolchain still needs provisioning.

Framework references: [Macroquad](https://macroquad.rs/), [audio API](https://docs.rs/macroquad/latest/macroquad/audio/index.html). Bevy remains a possible future option, but its [Windows setup](https://bevy.org/learn/quick-start/getting-started/setup/) includes additional build tools.

## Visual direction

Working title: **Rook Rainbow Quest**. Use a sunny storybook landscape: mint hills, sky blue, warm cream, coral, lavender and golden yellow. Chunky silhouettes, rounded panels, soft shadows and clear dark text. The chess pieces should be immediately recognizable, with small friendly faces that do not obscure their shapes.

Use a 1280×800 reference canvas with proportional scaling and letterboxing where needed. Support resizable windows and Windows display scaling without clipping or tiny controls. Fullscreen is optional through the settings panel or F11. Keep essential buttons at least 56 reference pixels tall. Child-facing text is short; icons and demonstrations carry most of the instructions.

Visual concept artwork illustrates mood and layout. The implementation should draw the actual board, buttons and text separately so controls remain crisp and responsive. Never bake game state into a background image.

## Main menu: Sunny Meadow

On launch, show the world map directly. The title sits at the top. A winding sandy path runs through green islands, flowers and tiny flags. The first stop is a large colorful rook-shaped clubhouse with a golden ring and the label “1”. Clicking it or the prominent **Play** button opens the level. Enter also starts it.

Two small future stops can sit farther along the path, labeled “Coming later”. They are decorative and do not react like available buttons. Keep the playable stop visually dominant. Do not display 24 empty nodes or promise an unavailable level.

Top corner controls: sound on/off, settings, and exit with a clear icon and label. Settings contain effects/music volume, reduced motion, fullscreen and local player selection. Start with two simple local profiles so siblings can keep their own completion star. No names, login, or reading-heavy setup required: choose an avatar/color, with the first profile selected by default.

After a win, the first node earns a permanent star for the active profile. **Play** becomes **Play again**. Map decorations celebrate, while the next stops remain “Coming later”. Future versions will unlock an explicitly configured successor only after completing its prerequisite.

## Level screen

Place the large 4×4 board centrally, with a short instruction above: **Capture all 3 pawns!** Show a rook portrait beside “Rooks move straight.” Display progress as three pawn-shaped tokens that turn into stars. Do not show a score, countdown, or move-efficiency grade.

Below the board, use four large controls: **Hint**, **Undo**, **Try again**, and **Map**. Keep Sound available in the corner. Escape returns to the map and preserves the active attempt in memory. Relaunching restores completion and preferences; unfinished moves may restart in this first version.

Start with the rook already selected and its legal destinations marked. Support click-piece/click-destination and optionally dragging once click input works reliably. Show the selected piece with a thick ring. Mark legal empty squares with dots and capturable pawns with rings, so color is not the only cue. A hovered destination previews the straight-line route.

An invalid move leaves the board unchanged. Gently wiggle the rook and say “Try a glowing square.” No red failure screen, life counter, buzzer, or lost progress. Legal non-capture moves are allowed, and pawns never move or attack. This is explicitly a movement-practice puzzle, not standard chess.

When the last pawn is captured, finish the movement animation, play a short happy flourish, and show **You did it!** with a large star, **Play again**, and **Back to map**. Celebrate every completion equally, including completions with hints and undo.

See `levels/01-rook-rainbow-trail.md` for the authoritative level description, exact board and tuning controls. Its JSON companion defines the proposed runtime data contract.

## Animation and sound

| Moment | Animation | Sound |
| --- | --- | --- |
| Map idle | Clouds drift; flowers sway; the first flag waves | Optional quiet, original meadow loop |
| Button hover/focus | Gentle lift, clear focus ring | None, to avoid repeated noise |
| Button press | 100 ms squash and release | Soft wooden tap |
| Rook selected | Ring pulses slowly | Small bell |
| Legal move | Slide for about 220 ms, tiny landing bounce | Soft wooden click |
| Pawn captured | Pawn pops into a star, 400 ms sparkle | Bright two-note chime |
| Hint | Route pulses twice | Quiet sparkle |
| Completion | Star bounce and 1–2 seconds of confetti | Original 1.5-second victory phrase |

Do not shake the whole screen or flash the background. Block additional board moves during the short move animation, but keep menu and mute responsive. Commit each move exactly once. Undo restores the entire previous logical state and cancels leftover effects.

Reduced motion replaces bouncing and moving scenery with static highlights and short fades. Sound can be muted at any time, and the preference persists. Never make audio necessary to understand a move or win. Use bundled original or suitably licensed assets; record attribution where required. Generate the few initial effects if that keeps delivery simple. Audio initialization failure must not prevent silent play.

## Proposed project layout

```text
Cargo.toml / Cargo.lock
src/main.rs                 native startup and main loop
src/game.rs                 map, playing and celebration states
src/rules.rs                board state and pure rook move validation
src/render.rs               drawing and responsive layout
src/audio.rs                sound loading, volumes and mute
src/save.rs                 per-profile progress and preferences
levels/01-rook-rainbow-trail.json
levels/01-rook-rainbow-trail.md
assets/                    bundled visuals, font and sounds
docs/DESIGN.md
scripts/setup.ps1
```

Load the embedded JSON through a typed schema. Keep level ID stable forever; never key progress by filename order or display title. Validate coordinates, overlaps, board size, supported piece types and win conditions at startup/build time. A later level gets its own JSON plus Markdown description. Keep rules separate from presentation so the first puzzle can be tested without opening a window.

Save preferences and profile completion under the user's local application data directory, outside the executable folder. Write through a temporary file then replace; handle missing/corrupt saves by recovering to defaults without crashing. Version the save format and show a small nonblocking message if saving fails.

## Build sequence and acceptance

1. **Native shell and map:** provision the toolchain; open a resizable window; implement original map art, Play, sound control, settings, profiles and exit. Verify scaling at 100%, 150% and 200%.
2. **One complete puzzle:** load level 01, support legal movement, captures, progress, hint, undo and restart. Verify that all three targets are reachable and invalid input never changes state.
3. **Game feel:** add animation timing, sound effects, reduced motion and completion celebration. Repeated clicks must not produce extra moves or duplicate captures.
4. **Local delivery:** preserve stars and preferences, embed assets, produce a release executable, and launch it outside the repository with networking unavailable.
5. **Kid playtest:** watch whether a child can start without help, understand the rook after one demonstration and finish in roughly 1–3 minutes. Record confusion and revise this single level before planning more content.

Automated checks should cover rook paths and blocking, capture removal, win detection, hint legality from arbitrary reachable positions, undo/restart, data validation and save round-tripping. Manually verify actual sound, window resize, fullscreen, mute, reduced motion, focus behavior, map return and release packaging. No checkmate engine, moving enemies, level editor or additional playable levels in this milestone.

## Next design decision after playtesting

Decide whether to add a second rook puzzle or introduce a bishop. The next difficulty step should follow observed comfort with this level. Retain stable individual descriptions and data files when adding content.
