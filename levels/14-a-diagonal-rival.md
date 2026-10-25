# 14. A Diagonal Rival

Version 0.7.1. Stable ID: `match-04`. World 1: The First Adventure. Trail area: Friendly Matches. Runtime source: `14-a-diagonal-rival.json`.

The rook starts on d1, away from the enemy bishop’s diagonal, with room to move. It cannot be captured on the first reply after any opening move by another friendly piece. The player can still put it in danger by moving onto an attacked square.

## Setup

- Board: 5×5, algebraic coordinates from bottom left.
- Friendly: rook d1, knight b1, bishop c1, pawn a2.
- Opponent: bishop e5, pawn a4.
- Map icon: Cream rook beside purple bishop, numbered badge and independent completion star.

```text
5 . . . . b
4 p . . . .
3 . . . . .
2 P . . . .
1 . N B R .
  a b c d e
```

Uppercase = cream friendly, lowercase = purple opponent. R/r rook, B/b bishop, N/n knight, P/p pawn, Q/q queen.

## Rules and learning goal

Alternate one player move and one opponent reply. Every player turn begins with no piece selected, including the first turn, after a reply/pass, and after restart. Click a cream piece, then its destination. Hints highlight a source and destination without selecting a piece for you.

Friendly pawns move up and enemy pawns move down. On the far rank a pawn automatically becomes a queen, keeping its color. Promotion can accompany a capture and grants no extra turn.

Automatic queen promotion is enabled (`pawnPromotion: true`). Rooks, bishops and queens cannot jump; knights can. No kings, check, checkmate, en passant, timers or move limits.

Opponent: `overlookEvery: 3`, `safety: 1`. Captures are favored; every Nth reply overlooks them when a quiet move is available. Threat awareness rises gradually. After 3 quiet rounds (`braveAfter: 3`), the opponent stays brave until a capture: it offers a legal capture when possible, otherwise approaches/trades without a safety penalty. A repeated position also triggers bravery. With one opponent left the threshold is at most two rounds. Ties are deterministic. An immobile opponent passes; the player's next turn still starts unselected.

## Tested winning example

1. c1 → a3
2. d1 → d5
3. a3 → b2
4. b1 → a3

The actual opponent replies between these 4 player moves. A regression checks opening rook mobility, safety before the first move and after every non-rook opening, and an actual pawn-opening reply.

## Retry and tuning

There is no undo or lives counter. Failed/stuck levels offer unlimited restart without losing stars or unlocked levels. There is no full game over. A match with no legal player move offers retry. Repeated positions trigger brave opponent play instead of failing the child.

Tune `opponent.braveAfter` (2–4 quiet rounds), `opponent.overlookEvery` (smaller means more missed captures), `opponent.safety` (0–3), or either army. Keep a friendly material advantage and at least one friendly pawn; later lessons contain two or three. Regenerate only this level with `cargo run --release --example author_matches -- --level match-04` after changing a board or policy, then update this description and run the tests.

Keep stable IDs when tuning. `icon` controls the level marker; add title translations in `src/language.rs` when renaming. World membership comes from `worlds/worlds.json`; World 1 contains twenty levels. Earlier designs are preserved in Git history.
