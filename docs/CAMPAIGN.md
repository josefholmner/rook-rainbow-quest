# Rook Rainbow Quest v0.10.0 — Three worlds, sixty levels

The larger world map contains three worlds with twenty levels each. World 3 adds Royal Chess Kingdom; see [the regular-chess guide](WORLD-3.md). World 2 unlocks when World 1 is complete; see [the checkmate guide](WORLD-2.md). World 1 is Sunflower Meadows (Solrosängarna). World 2 is Checkmate Castle (Schackmattslottet), with 6×6 boards throughout. The top-level map is Adventure Galaxy (Äventyrsgalaxen). Its inner trail keeps the three areas (Piece Garden, Friendly Matches, Team Adventures). Each row links to an individual description with exact board, icon, example and tuning notes.

| Level | Design | Mode / piece | Board | Shortest lesson moves | Required extra moves | Friendly team | Opposing team |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 01 | [Rook's Rainbow Trail](../levels/01-rook-rainbow-trail.md) | Rook | 4×4 | 3 | 1 | rook a1 | pawn b1, pawn a2 |
| 02 | [Up to the Clouds](../levels/02-up-to-the-clouds.md) | Rook | 4×4 | 5 | 2 | rook a1 | pawn c1, pawn b2, pawn d3 |
| 03 | [Hello, Bishop!](../levels/03-hello-bishop.md) | Bishop | 4×4 | 3 | 1 | bishop a1 | pawn a3, pawn c3 |
| 04 | [Butterfly Diagonals](../levels/04-butterfly-diagonals.md) | Bishop | 4×4 | 5 | 2 | bishop a1 | pawn c1, pawn b4, pawn d4 |
| 05 | [Hello, Knight!](../levels/05-hello-knight.md) | Knight | 4×4 | 3 | 1 | knight a1 | pawn c2, pawn c4 |
| 06 | [Two Little Hops](../levels/06-two-little-hops.md) | Knight | 4×4 | 5 | 2 | knight a1 | pawn d3, pawn b4, pawn c4 |
| 07 | [Hello, Pawn!](../levels/07-hello-pawn.md) | Pawn | 5×5 | 3 | 1 | pawn b1 | pawn c3, pawn b4 |
| 08 | [Pawn's Garden](../levels/08-pawn-s-garden.md) | Pawn | 5×5 | 6 | 2 | pawn b1 | pawn c3, pawn b4, pawn a1 |
| 09 | [Hello, Queen!](../levels/09-hello-queen.md) | Queen | 4×4 | 3 | 1 | queen a1 | pawn b1, pawn c3 |
| 10 | [Queen's Rainbow](../levels/10-queen-s-rainbow.md) | Queen | 5×5 | 5 | 2 | queen a1 | pawn d2, pawn a4, pawn e5 |
| 11 | [Our First Match](../levels/11-our-first-match.md) | Match | 4×4 | — | — | rook a1, rook d1, pawn a2 | pawn a3, pawn c4 |
| 12 | [A Little Rook Rival](../levels/12-a-little-rook-rival.md) | Match | 4×4 | — | — | rook a1, bishop b1, bishop c1, pawn a2 | rook d4, pawn c3 |
| 13 | [Four Friendly Helpers](../levels/13-four-friendly-helpers.md) | Match | 5×5 | — | — | rook a1, rook e1, bishop b1, bishop d1, pawn a2 | rook a5, pawn d4 |
| 14 | [A Diagonal Rival](../levels/14-a-diagonal-rival.md) | Match | 5×5 | — | — | rook d1, knight b1, bishop c1, pawn a2 | bishop e5, pawn a4 |
| 15 | [The Queen Helps](../levels/15-the-queen-helps.md) | Match | 5×5 | — | — | queen c1, rook a1, pawn b2 | rook e5, pawn d4 |
| 16 | [Hops Across the River](../levels/16-hops-across-the-river.md) | Match | 5×5 | — | — | rook a1, bishop c1, knight d1, pawn b2, pawn a2 | rook e5, knight c4 |
| 17 | [Teamwork in the Woods](../levels/17-teamwork-in-the-woods.md) | Match | 6×6 | — | — | queen d1, bishop b1, pawn c2, pawn a2 | bishop f6, rook a6, pawn e4 |
| 18 | [Two Rooks Together](../levels/18-two-rooks-together.md) | Match | 6×6 | — | — | rook a1, rook f1, knight c1, pawn d2, pawn a2 | rook f6, bishop b6, pawn e5 |
| 19 | [A Clever Little Team](../levels/19-a-clever-little-team.md) | Match | 6×6 | — | — | queen c1, rook f1, pawn b2, pawn a2, pawn c2 | rook a6, knight e6, bishop d5 |
| 20 | [The Friendly Final](../levels/20-the-friendly-final.md) | Match | 6×6 | — | — | queen c1, rook a1, bishop f1, pawn d2, pawn a2, pawn b2 | queen f6, rook a6, pawn c5 |

## Lesson requirements

The first lesson for each piece needs at least one quiet move beyond the number of targets; the second needs at least two. These are requirements on the exact shortest winning route, computed independently by breadth-first search, not restrictions on how the child may move. All lessons remain untimed, with free hints. Level 8 goes further: its pawn must promote to reach a target behind it, and takes six moves for three targets. A regression proves the level is impossible with promotion disabled.

## Matches and promotion

Levels 11–15 have one friendly pawn; 16–18 have two; 19–20 have three. All player armies retain a material advantage. Both sides automatically promote pawns to queens on the far rank. Promotion can capture a piece, never changes team and does not grant an extra turn. No promotion choice dialog, double pawn move or en passant is used. Kings, check and checkmate remain absent.

Each player turn begins with no selected piece, even after a pass. Click a cream piece, then a glowing destination. Hints highlight a recommended source/destination without selecting automatically. The opponent becomes brave after 2 quiet rounds in levels 11–13, 3 in 14–17, and 4 in 18–20. When only one opponent remains, the wait is at most two rounds. A repeated position also activates bravery, which stays active until a capture. The opponent offers a legally capturable destination whenever possible; otherwise it approaches and trades without a safety penalty. This creates opportunities the child must still notice and take. Repetition no longer forces a child to restart.

The gentle deterministic opponent overlooks captures every Nth reply when a quiet move exists; N rises from 2 to 6, with threat awareness rising from 0 to 3. Each match has a tested winning example against the actual policy. Family playtesting should guide difficulty tuning.

No undo, lives or full game over. Failed/stuck levels offer unlimited retry; existing stars and unlocked levels remain safe.

## Main menu, saves and world navigation

The app opens with Continue, New Game and Quit. Continue is disabled for a player with no adventure. New Game always asks Yes/No before resetting that player's stars and cursor. No changes nothing. Other profiles and settings are kept. Confirming New Game creates an adventure even before the first star is earned, so Continue works after closing and reopening. Old saves with earned stars can Continue. Continue resumes the world map and saved level cursor; unfinished board moves are not checkpoints.

Arrow keys move between neighboring unlocked worlds just like the inner level trail; Enter enters the selected world. Adventure Galaxy contains Sunflower Meadows and Checkmate Castle. Escape returns from level → level map → world map → main menu. Buttons also return to the world map/main menu. Each world owns a consecutive level-ID range in `worlds/worlds.json`; adding another world does not require changing menu layouts. It unlocks after the previous worlds' levels are complete. Tests exercise two-world expansion, locked traversal and profile isolation.

## Authoring

Each JSON is embedded at build time. Update its Markdown alongside any changes. `minimumQuietMoves` documents/validates lesson requirements; `pawnPromotion` enables the second pawn lesson's transformation and documents match rules. `allies`, `targets`, `opponent.overlookEvery`, `opponent.braveAfter` and `opponent.safety` tune matches independently. After edits, update examples, run `cargo run --example author_matches` for match witnesses, and run `scripts/test.ps1 -Native`. Keep IDs stable for tuning; new identities need new IDs. Prior designs are retained in Git history; archives and ZIP files are ignored.

## Completing a world

Earning every level star in the active world opens a celebration naming that world, with a Back to Adventure Galaxy button. It appears on the transition to complete, including when unlocked levels were earned out of order. The access-unlock setting alone does not complete a world. Replaying an already completed finale celebrates again; other replays use ordinary level victory. Other profiles retain their own completion state.
