# Rook Rainbow Quest v0.10.0 validation — 2026-10-25

- All 94 Rust tests passed (91 library, two rendering/layout, one launcher). Formatting and clippy with warnings denied passed. All four packaging regressions passed.
- Added regressions ensure every board size remains unobstructed by result banners, friendly formations differ across World 2 levels 3–20 and all twenty World 3 levels, and the four early castle teams have distinct compositions with pawn helpers.
- All twenty World 2 winning routes were replayed against the real deterministic opponent. Changed positions have regenerated routes. All twenty World 3 guides remain legal three-player-move openings; these are not proofs of forced wins. The existing tests still verify army growth, material advantage, special rules, and the full standard finale.
- The optimized native campaign passed all sixty levels: forty winning routes and twenty regular-chess opening tours. All eight native regressions passed: dead positions, regular chess/promotion, checkmate/stalemate, main menu, retry, settings and both exit modes. Tests use isolated saves.
- Visually checked Swedish and English stalemate banners, victory and world completion, and a revised World 3 formation. Every board square stays visible; old underlying controls are hidden. Draw explanations also cover World 3 stalemate. Promotion retains a translucent chooser and settings retains its clear readable panel.
- Verified SHA-256 for all forty World 1 JSON/description files against the pre-change baseline: unchanged. World 2 retains its first two queen-assisted lesson designs; World 3 retains its first five special-rule designs, material progression and full-board finale. All stable IDs, saves and rules are retained.
- Updated each reviewed level description and the World 2/3 design documentation with exact armies, formation notes and verified routes/openings.
- Game and native launcher passed pre/post Microsoft Defender scans and native execution checks. Launcher tests passed GUI subsystem, icon/version, adjacent release folder, extracted layout and Unicode/spaced path handoff. No security configuration changed.
- ZIP executables match the published game/launcher; all three SHA-256 manifest entries match. The current release folder, launcher, ZIP and manifest are prepared in dist. The former v0.9.0 package is outside dist under ignored target/superseded-release-files, so the prepared release files contain the latest version only.
- Existing LFS policy covers all binary assets and new release files. Git staging, configuration, commits and pushing remain with the user; this work changed files only.

Game SHA-256: 1516ACADDC1E32CC758BF35477DD1A7F6915F0A8F0CC55602BA01CD15DA50775

Launcher SHA-256: 7721C50D4941D4A2529D7D8F4FCF31ADBB0C03615955EFFA0BD9262E68356850

ZIP SHA-256: B0B79667AF5B833C4BD4D9F1833253F05CA82A62A2BABD71A6E569E8AA960EAF

Raw native evidence: target/package-staging/9ee8c3057faa4c5d8ea560362a1a0dc1/validation/ and launcher-validation/ (ignored). Preview images and World 1 baseline: C:/Users/Admin/Documents/Codex/2026-10-23/i/outputs/Rook-Rainbow-Quest/v0.10.0/.
