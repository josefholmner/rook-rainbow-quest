# Rook Rainbow Quest v0.10.1 validation — 2026-10-25

- Renamed the animation setting to Move animation: On / Off (Draganimation: på / av). On maps to the existing full-animation preference; Off maps to reduced motion. Existing save values keep their behavior. No levels, opponent strength or progress data were changed.
- All 94 Rust tests passed: 91 library, two rendering/layout, one launcher. Formatting, clippy with warnings denied, and all four packaging regressions passed. Both new labels are covered by localization checks.
- The optimized native campaign passed all sixty levels, comprising forty winning routes and twenty legal regular-chess opening tours. All eight native regressions passed.
- The settings regression now clicks the real animation toggle four times, checks reduced/full states in both languages, and captures both labels. Visually inspected Swedish Off and English On/Off screenshots for clarity and fit at fullscreen/windowed sizes. High-DPI/4K settings rendering remains covered.
- Native launcher validation passed GUI subsystem, icon/version metadata, adjacent release folder, extracted layout and Unicode/spaced path handoff. Game and launcher passed pre/post Defender scans and real-time event checks. No security configuration changed.
- ZIP executables and all manifest checksums match the published files. Current release files are in dist; the superseded v0.10.0 package is under ignored target/superseded-release-files. Existing LFS rules cover the new release files and binary assets. Git staging/configuration/commits/pushing remain with the user.
- Native checks use isolated temporary saves; the active player's World 2 level 6 progress is untouched.

Game SHA-256: CBA4F8969F9137BF72A85A5EA129A755B6ACA3408F59AD779EE39EF9498C14C8

Launcher SHA-256: 288DA75AE9D8C07C80F4957345CE3083A875AA196736C73A712BBC1EF8D774BC

ZIP SHA-256: 0EFA51AFDAE73074C1A81FE5D44DF300DBBEFAA1766C4A3198FCDD8DCA18A2E1

Raw evidence: target/package-staging/18a8ff87813e4088a143aaacd29ba37f/validation/ and launcher-validation/ (ignored).
