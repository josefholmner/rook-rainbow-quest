# World 3 — Royal Chess Kingdom, Rook Rainbow Quest v0.10.0

Twenty individually described levels introduce regular chess on an 8×8 board. Both teams have exactly one king. The first five focus on king safety, pawn double moves, castling, en passant and promotion choice. Armies then grow from reduced positions to equal teams; level 20 is the complete standard starting position, marked with a castle icon and FINAL BOSS / SLUTBOSS label.

The player is White, moves first and must checkmate Black to earn a star. The first level gives a 14-point material advantage. That advantage never increases and reaches zero by level 18. Both armies have sixteen pieces in the finale. World 1 retains its original level data and routes. World 2 has its own reviewed formations; all worlds keep their stable IDs and rules.

## Regular rules

- Both kings must stay out of attack. Checks must be answered; kings cannot be captured or stand next to one another. Pinned pieces cannot expose their king.
- Pawns may move one or two squares from their starting rank if both squares are clear. They capture diagonally. En passant is available only immediately after the corresponding double step, and cannot expose the king.
- Castling requires retained rights, a clear path and a king that is not in check and crosses no attacked square. Select the king and click c1/g1 (Black uses c8/g8). Moving or capturing the relevant rook, or moving the king, removes rights permanently.
- Promotion offers queen, rook, bishop or knight. The board move waits for the child's choice. Escape cancels the choice without moving. The opponent normally promotes to a queen.
- Checkmate of White offers a free retry; checkmate of Black earns one star. Stalemate, insufficient material, threefold repetition and fifty-move draws offer retry, preserving earned stars. The application automatically makes available repetition/fifty-move claims for both sides; those are not wins. No undo, lives, clock or full game over.

World 3 uses the MIT-licensed cozy-chess move generator behind a separate state adapter. The earlier puzzle engines do not acquire these additional rules.

## Opponent and examples

The deterministic opponent chooses only legal moves. Early levels use one-ply material/position scores; later levels look one reply further, with a fixed node budget. Periodic second-choice moves make it gentler; their frequency decreases with level number. The opening guide and live hints use the same legal state, with a bounded hint search. Neither side gets illegal immunity from check.

Each JSON contains three verified player opening moves with actual opponent replies. These are teaching openings, not proofs of forced wins or automatic level completion. The native campaign tours those openings without inventing stars. Full-game legal operation, special moves, checkmates for both teams and draws have separate rule/native regressions. Difficulty still benefits from family playtesting.

| Level | Individual design | Pieces: White vs Black | Material edge | Icon |
|---|---|---|---|---|
| 1 | [Hello, Your Majesty!](../levels/41-chess-01.md) | 3 vs 1 | +14 | queen |
| 2 | [Two Steps Forward](../levels/42-chess-02.md) | 4 vs 2 | +14 | pawn |
| 3 | [Castle to Safety](../levels/43-chess-03.md) | 4 vs 2 | +10 | king |
| 4 | [A Passing Pawn](../levels/44-chess-04.md) | 4 vs 3 | +9 | pawn |
| 5 | [Choose Your Promotion](../levels/45-chess-05.md) | 5 vs 3 | +6 | pawn |
| 6 | [Knights Join In](../levels/46-chess-06.md) | 6 vs 4 | +6 | knight |
| 7 | [Bishop Buddies](../levels/47-chess-07.md) | 6 vs 4 | +6 | bishop |
| 8 | [Keep Your King Safe](../levels/48-chess-08.md) | 7 vs 5 | +4 | king |
| 9 | [Trade and Protect](../levels/49-chess-09.md) | 8 vs 6 | +4 | king |
| 10 | [A Growing Army](../levels/50-chess-10.md) | 9 vs 7 | +4 | king |
| 11 | [Find the Check](../levels/51-chess-11.md) | 9 vs 8 | +3 | king |
| 12 | [Open a File](../levels/52-chess-12.md) | 10 vs 9 | +3 | king |
| 13 | [The Royal Guard](../levels/53-chess-13.md) | 11 vs 10 | +3 | king |
| 14 | [Pieces Work Together](../levels/54-chess-14.md) | 12 vs 11 | +3 | king |
| 15 | [Almost Even](../levels/55-chess-15.md) | 12 vs 11 | +3 | king |
| 16 | [A Fair Fight](../levels/56-chess-16.md) | 13 vs 12 | +3 | king |
| 17 | [Opening Practice](../levels/57-chess-17.md) | 14 vs 13 | +1 | king |
| 18 | [Both Wings Ready](../levels/58-chess-18.md) | 14 vs 14 | +0 | king |
| 19 | [One More Adventure](../levels/59-chess-19.md) | 15 vs 15 | +0 | king |
| 20 | [Grand Castle](../levels/60-chess-20.md) | 16 vs 16 | +0 | Castle / FINAL BOSS |

## Formation variety in v0.10.0

All twenty friendly formations and complete positions are distinct. The first five special-rule lessons and full starting-position finale are retained. Levels 6–19 vary developed bishops/knights and pawn structures on both wings and in the center, with a few developed enemy formations later on. Piece counts and the material-edge curve in the table remain unchanged. Each paired description records its specific changes and verified opening; those openings remain examples, not forced-win claims.

Draw, retry and victory banners now sit below the board with a translucent cream background, leaving the final position visible.

## Per-level edits

Every JSON has a matching Markdown description listing the teaching focus, exact armies, material edge, castling rights, en-passant context, tested opening, opponent policy and icon. Keep IDs `chess-01` through `chess-20` stable for saves. `castlingRights` uses KQkq; `enPassant` is a target square and is present only for the passing-pawn lesson's stated previous move. `openingOnly` distinguishes the teaching route from the required real checkmate. `finalBoss` and `icon.castle` mark the finale.

After changing positions, regenerate with `cargo run --locked --release --example author_matches -- --world3` and update the paired descriptions. Use `--level chess-15` to target one design. Run `scripts/test.ps1` and native validation before packaging. `worlds/worlds.json` adds the third consecutive range without changing either existing world's endpoints or the old final JSON link.
