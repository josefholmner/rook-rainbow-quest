# 19. A Clever Little Team

Version 0.6. Stable ID: `match-09`. World 1: The First Adventure. Trail area: Team Adventures. Runtime source: `19-a-clever-little-team.json`.

Your queen and rook still have the advantage; check the opposing knight and bishop before choosing a capture.

## Setup

- Board: 6×6, algebraic coordinates from bottom left.
- Friendly: queen c1, rook f1, pawn b2, pawn a2, pawn c2.
- Opponent: rook a6, knight e6, bishop d5.
- Map icon: Cream queen beside purple rook, numbered badge and independent completion star.

```text
6 r . . . n .
5 . . . b . .
4 . . . . . .
3 . . . . . .
2 P P P . . .
1 . . Q . . R
  a b c d e f
```

Uppercase = cream friendly, lowercase = purple opponent. R/r rook, B/b bishop, N/n knight, P/p pawn, Q/q queen.

## Rules and learning goal

Alternate one player move and one opponent reply. Every player turn begins with no piece selected, including the first turn, after a reply/pass, and after restart. Click a cream piece, then its destination. Hints highlight a source and destination without selecting a piece for you.

Friendly pawns move up and enemy pawns move down. On the far rank a pawn automatically becomes a queen, keeping its color. Promotion can accompany a capture and grants no extra turn.

Automatic queen promotion is enabled (`pawnPromotion: true`). Rooks, bishops and queens cannot jump; knights can. No kings, check, checkmate, en passant, timers or move limits.

Opponent: `overlookEvery: 6`, `safety: 2`. Captures are favored; every Nth reply overlooks them when a quiet move is available. Threat awareness rises gradually. After 4 quiet rounds (`braveAfter: 4`), the opponent stays brave until a capture: it offers a legal capture when possible, otherwise approaches/trades without a safety penalty. A repeated position also triggers bravery. With one opponent left the threshold is at most two rounds. Ties are deterministic. An immobile opponent passes; the player's next turn still starts unselected.

## Tested winning example

1. c2 → c3
2. c1 → a1
3. a1 → a6
4. a6 → c4
5. f1 → b1
6. c4 → d4

Only player moves are listed; the actual updated opponent replies between them. All 6 player moves are validated against this exact policy.

## Retry and tuning

There is no undo or lives counter. Failed/stuck levels offer unlimited restart without losing stars or unlocked levels. There is no full game over. A match with no legal player move offers retry. Repeated positions trigger brave opponent play instead of failing the child.

Tune `opponent.braveAfter` (2–4 quiet rounds), `opponent.overlookEvery` (smaller means more missed captures), `opponent.safety` (0–3), or either army. Keep a friendly material advantage and at least one friendly pawn; later lessons contain two or three. Regenerate `exampleTurns` with `cargo run --example author_matches` after changing a board or policy, then update this description and run the tests.

Keep stable IDs when tuning. `icon` controls the level marker; add title translations in `src/language.rs` when renaming. World membership comes from `worlds/worlds.json`; the twenty levels belong to one world. Earlier designs are preserved in `docs/archive/v0.4-levels/`.
