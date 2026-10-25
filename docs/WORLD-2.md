# World 2 — Checkmate Castle, Rook Rainbow Quest v0.10.0

Twenty separate checkmate levels follow the original capture world. The first two opponents have only a king; the child has two rooks and an extra queen in each of those lessons. Enemy teams then grow from one pawn to six pieces including the king; all twenty boards are 6×6, giving the king and attacking pieces room to maneuver. Players always start with at least five points of material advantage (queen 9, rook 5, bishop/knight 3, pawn 1; the opposing king has no material score). No player king is used.

The player moves first. The opponent must resolve check: move its king to safety, block the attacking line, or capture the attacker legally. A king cannot enter an attacked square or capture a defended piece. Pawns attack diagonals even when those squares are empty. Enemy helpers cannot expose their own king. These restrictions also apply during periodic oversights and brave play.

Checkmate wins: the enemy king is attacked and its whole team has no legal reply. The king stays on the board, and surviving helpers need not be captured. No check plus no enemy legal moves is stalemate, which offers an encouraging retry and never awards a win. Losing the player's team also allows unlimited retry. Existing stars remain safe. There is no undo or full game over. Both sides promote to queens at the far rank.

The early opponent misses a capture every third reply when a legal quiet move exists. That interval rises to six, with safety rising from 0 to 3 and bravery after 2–4 quiet rounds. It never overlooks check. Repeated positions encourage bravery without ending the level. The player must still build the mating net; the game never invents a checkmate or captures the king.

The move counts below are tested winning examples against the exact deterministic policy, not shortest-route or forced-mate claims. Family playtesting should guide the next tuning pass. Each description lists its specific armies, teaching focus, route, policy, stable ID and icon.

| Local level | Design | Board | Enemy team | Witness player moves |
|---|---|---|---|---|
| 1 | [Hello, King!](../levels/21-mate-01.md) | 6×6 | king d4 | 2 |
| 2 | [Two Rooks Take Turns](../levels/22-mate-02.md) | 6×6 | king d4 | 3 |
| 3 | [The Pawn Guard](../levels/23-mate-03.md) | 6×6 | king d4, pawn c3 | 3 |
| 4 | [The Little Gate](../levels/24-mate-04.md) | 6×6 | king d4, pawn c2 | 3 |
| 5 | [Queen and Rook](../levels/25-mate-05.md) | 6×6 | king d4, pawn a3 | 3 |
| 6 | [A Knight Visits](../levels/26-mate-06.md) | 6×6 | king d4, pawn c3, knight a4 | 3 |
| 7 | [The Bishop Bridge](../levels/27-mate-07.md) | 6×6 | king e5, pawn d4, bishop c4 | 3 |
| 8 | [Two Pawn Guards](../levels/28-mate-08.md) | 6×6 | king e5, pawn d4, bishop c4, pawn c3 | 3 |
| 9 | [Open the File](../levels/29-mate-09.md) | 6×6 | king e5, pawn d4, bishop c4, pawn c3 | 3 |
| 10 | [Rooks Make Room](../levels/30-mate-10.md) | 6×6 | king e5, pawn d4, knight c4, pawn c3 | 3 |
| 11 | [Watch the Knight](../levels/31-mate-11.md) | 6×6 | king e5, pawn d4, knight c4, pawn c3 | 3 |
| 12 | [Meet the Bishop](../levels/32-mate-12.md) | 6×6 | king e5, pawn d4, bishop c4, pawn c3 | 3 |
| 13 | [The Guarded Corner](../levels/33-mate-13.md) | 6×6 | king f6, pawn e5, pawn d5, knight c4 | 3 |
| 14 | [Three Pawn Steps](../levels/34-mate-14.md) | 6×6 | king f6, pawn e5, pawn d5, knight c4, pawn b5 | 2 |
| 15 | [The Royal Rook](../levels/35-mate-15.md) | 6×6 | king f6, pawn e5, pawn d5, rook c5, pawn b5 | 4 |
| 16 | [The Queen Team](../levels/36-mate-16.md) | 6×6 | king f6, pawn e5, pawn d5, rook c5, pawn b5 | 2 |
| 17 | [Clear the Diagonal](../levels/37-mate-17.md) | 6×6 | king f6, pawn e5, pawn d5, rook c5, pawn b5 | 3 |
| 18 | [Castle Helpers](../levels/38-mate-18.md) | 6×6 | king f6, pawn e5, pawn d5, rook c5, pawn b5, knight f4 | 3 |
| 19 | [The Royal Puzzle](../levels/39-mate-19.md) | 6×6 | king f6, pawn e5, pawn d5, rook c5, pawn b5, bishop f4 | 2 |
| 20 | [Checkmate Celebration](../levels/40-mate-20.md) | 6×6 | king f6, pawn e5, pawn d5, rook c5, pawn b5, knight f4 | 4 |

## Formation variety in v0.10.0

Levels 3–6 have four different friendly teams: queen/two rooks/pawn; queen/rook/bishop/pawn; queen/rook/knight/pawn; and queen/two rooks/two pawns. Every friendly starting formation from level 3 onward is distinct. Later lessons shift a supporting bishop, knight or pawn rather than introducing a sudden difficulty jump. The first two queen-assisted lessons are unchanged. Winning routes were regenerated against the actual opponent.

Result banners are translucent and sit below the board. On stalemate the child can inspect every square and read why no legal reply exists.

## Tuning a level

Edit its JSON and neighboring Markdown. Keep `id` stable to preserve stars. `mode` and `goal` are `checkmate`; the enemy needs exactly one king and the player none. Change `allies`, `targets`, board dimensions and opponent parameters as needed. `exampleTurns` is validated by replaying actual opponent replies. Regenerate only this world's routes with `cargo run --release --example author_matches -- --world2`, update descriptions and run `scripts/test.ps1 -Native`. The default authoring helper can regenerate both worlds' match routes. Check in only the latest validated release package; use Git history for earlier designs and releases.

The universe background was created with the built-in imagegen tool. Its complete generation prompt, method and production file are in [UNIVERSE-ART.md](UNIVERSE-ART.md).
