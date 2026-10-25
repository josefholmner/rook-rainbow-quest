# Implementation — Rook Rainbow Quest v0.10.0

## Campaign and world navigation

Rust with Macroquad provides a native Windows application. `build.rs` embeds filename-ordered level JSON and requires a neighboring Markdown description. Sixty stable IDs belong to three consecutive world ranges in `worlds/worlds.json`. Worlds unlock after all earlier levels are completed; each inner trail displays local numbers 1–20. Both map layers move the avatar with arrows and follow it with a scrolling camera.

Main → Worlds → level Map → Level are explicit scenes. Continue requires an adventure or legacy earned stars. New Game confirms Yes/No, resets only the selected profile and retains preferences and the other profile. Continue remembers map cursors rather than unfinished board moves. Existing saves keep World 1 stars and unlock World 2 when World 1 is complete.

## Rules

`movement.rs` handles geometry, blockers, one-step pawns, knights and kings. `rules.rs` validates authored metadata and practice routes; first/second lessons require exact shortest paths with one/two quiet moves beyond target count. The second pawn lesson needs automatic queen promotion.

`Battle` handles alternating capture matches and checkmate matches. The player never has a king. Only checkmate opponents have exactly one king. Capture matches win when all opponents are gone; checkmate matches win only when the enemy king is attacked and no enemy move resolves it. King captures are rejected before movement. Enemy moves simulate the resulting board and reject self-check, including discovered attacks and defended king captures. Pawn attack geometry is separate from forward movement. No-check immobility is stalemate and produces a local retry, never a star. There is no castling, en passant, double pawn step, player check or player checkmate. Both sides autoqueen at the far edge.

The deterministic opponent favors captures and approach, periodically overlooks captures when legal quiet moves exist, and becomes brave after 2–4 quiet rounds/repetition. Check legality is mandatory in all policy branches. World 1's lone enemy becomes brave after at most two quiet rounds. Bravery persists until a capture. It creates opportunities rather than guaranteeing automatic wins.

`Session` adapts lessons and battles. Match selection is cleared on every turn transition, including promotion, a pass and retry. Hints highlight without selecting. World 2 hints replay the bounded authored route when board and reply count match; elsewhere a one-move score considers material, checks and king mobility. No expensive tree search runs in the live interface. Offline `author_matches` uses bounded beam search to write validated witnesses. Those witnesses prove wins against the configured opponent, not optimal play or forced mates.

## Rendering and performance

Scenes render directly to the physical letterboxed window with four-sample antialiasing. Text rasterizes for actual viewport/DPI into a shared font atlas. A custom text shader uses high-precision texture coordinates: the engine default uses low-precision UVs, which damaged glyphs in a large atlas. Glyphs are prepared before text submission, and modal boundaries finish older quads before atlas growth. The font atlas resets on display-scale changes to bound growth during repeated resizing. Grouped text draws share one texture while keeping settings crisp. Modal boundaries flush preceding layers. Original piece/mascot artwork is baked once at 3× into one 1536×1920 atlas, including the new crowned king. Runtime animation moves texture quads. Panels use non-overlapping rounded meshes with subpixel curve quality. Result dialogs share a translucent cream banner below the board with a light 8% backdrop tint, so all squares remain visible after a draw, failure, victory or world completion. Promotion retains a translucent chooser card. Settings keeps its readable opaque panel. A layout regression protects board visibility on every supported board size. The meadow is unchanged; the top-level selector uses the original space painting described in UNIVERSE-ART.md. Offscreen 4K rendering is for export tests.

Initial sessions/routes are cached. A coalescing save worker handles atomic writes away from the rendering thread and reports failures to the interface. Quit requests deferred shutdown through the normal frame loop and flushes queued saves with a bounded wait. `--performance-test --performance-report PATH` measures native optimized traversal with isolated real saves. See PERFORMANCE.md for measured controls and remaining limitations.

## Verification and versions

Pure tests exercise geometry, legality, check responses, checkmate/stalemate, promotion, every authored route, hints, world unlocking, profile/save isolation, localization, DPI and deferred exit. Native regressions use isolated state and application handlers, not OS input automation. The full campaign crosses three worlds, plays forty winning routes and twenty legal regular-chess openings, and exports screenshots. Separate regressions cover main menu, retries, checkmate/stalemate, settings in both languages at multiple sizes, and process exit in both window modes.

Each level has separate editable JSON/Markdown. Git history is the version archive. Root ignore rules exclude `target`, local toolchains, older `dist` packages, archive directories and ZIP files generally. Explicit exceptions check in the latest v0.10.0 executable folder with licenses, its Windows ZIP, SHA-256 checksums and the stable launcher. Update these version-specific exceptions when replacing the checked-in release.

Release profiles retain debug information without shipping PDB files. A stripped v0.7 candidate was quarantined by Defender; the fresh diagnostic release was scanned before execution. See TEST-RESULTS.md for the exact validation and any remaining release limits.

The settings unlock shortcut uses `Save.all_levels_unlocked`, a per-profile boolean array with false defaults for legacy saves. It overrides access checks in campaign/world navigation without changing completed IDs or stars. `unlock_all_levels` starts an adventure so Continue works; confirmed New Game clears only the active profile’s override.

The v0.7.1 source was reviewed and rebuilt from an empty target directory with the existing locked dependencies and release settings. The newly packaged binary passed native campaign/regressions and pre/post Defender scans; the launcher targets the latest validated release. The earlier quarantined binary is rejected by hash. Packaging stages under the ignored target directory and validates before publishing; scripts/test-packaging.ps1 tests the publication boundary with isolated inert fixtures. See RELEASE-REVIEW-v0.7.1.md for evidence and the limits of the local review.

## v0.7.2 world completion

Every checkmate level uses a 6×6 board; authored player routes are regenerated against the actual opponent. World IDs and level IDs remain stable. World titles are Sunflower Meadows and Checkmate Castle; Adventure Galaxy names the top-level selector. Swedish/English titles and celebration controls use the normal localization catalog.

`World::completed` requires all earned IDs in that world range and never treats the access override as earned stars. On the transition from incomplete to complete, or a repeat win on a completed finale, `App::settle` records the completed world and the victory modal presents its title with button 33 returning to Adventure Galaxy. Enter defaults to that button, and Escape also returns there. Returning, retrying or changing profile clears the transient celebration; no new save schema is needed. The native campaign verifies both celebrations and activates their real return buttons before continuing. Celebration labels reuse the already warmed 48/38/28/27 text sizes; the level-trail return control uses the short localized Galaxy label. This avoids introducing font-atlas growth during the celebration. Screenshots of both popups, a 6×6 board and the renamed galaxy were visually checked.

## World 3 standard chess

`standard.rs` owns a cozy-chess Board, castling/en-passant/move-counter state and repetition identities. Its adapter maps standard king destinations c/g to the library's king-to-rook castling encoding. Battle delegates only `standard-chess` levels to this state and synchronizes units/turn/outcome for the existing renderer. Original rule and opponent branches remain for Worlds 1/2. Pawn double moves and en passant therefore cannot leak into those levels.

Standard promotion is pending UI state until a role is chosen, with no premature board mutation; Escape cancels it. Search is deterministic, one or two plies with a 1,800-node budget. Opponent promotions choose queen; White may underpromote. Threefold identities normalize unavailable en-passant targets and include castling rights/turn. Available fifty-move/threefold claims are automatic on behalf of both teams. Static dead-material detection is conservative; draws do not award stars.

World 3 examples validate a legal opening against actual replies (`openingOnly`), not a forced win from the full-game starting position. The native campaign wins the old forty routes then tours the new twenty opening guides with an isolated access override, explicitly rejecting fake stars. A separate regular-chess native case checks actual castling, en passant, the promotion picker/underpromotion, final-board setup, real checkmate awarding a star, and a draw/retry preserving stars. [cozy-chess documentation](https://docs.rs/cozy-chess/0.3.4/cozy_chess/) describes the underlying move generator; both crates are MIT licensed.
