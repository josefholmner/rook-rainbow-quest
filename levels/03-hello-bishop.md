# 03. Hello, Bishop!

Version 0.6. Stable ID: `lesson-bishop-01`. World 1: The First Adventure. Trail area: Piece Garden. Runtime source: `03-hello-bishop.json`.

Plan a route with at least 1 quiet move(s), then capture every pawn.

## Setup

- Board: 4×4, algebraic coordinates from bottom left.
- Friendly: bishop a1.
- Stationary targets: pawn a3, pawn c3.
- Map icon: Cream bishop, 1 small gold star(s) for its lesson pair, numbered badge and independent completion star.

```text
4 . . . .
3 p . p .
2 . . . .
1 B . . .
  a b c d
```

Uppercase = cream friendly, lowercase = purple opponent. R/r rook, B/b bishop, N/n knight, P/p pawn, Q/q queen.

## Rules and learning goal

Capture all 2 stationary targets with the one friendly piece. Targets neither move nor attack. Empty-square moves are allowed. The exact shortest winning route is 3 moves, including 1 quiet moves. `minimumQuietMoves: 1` requires **at least 1 more moves than targets** on every winning route. A deliberately long example alone does not satisfy the rule; breadth-first search verifies the minimum.

Promotion is disabled in this lesson (`pawnPromotion: false`). Rooks, bishops and queens cannot jump; knights can. No kings, check, checkmate, en passant, timers or move limits.

Hints follow a shortest complete winning route, recalculated and cached after every move, including detours and promotion when enabled.

## Tested winning example

a1 → c3 → b2 → a3

The first square is the starting square; each arrow is one move. Both this authored route and the independently computed shortest route are tested.

## Retry and tuning

There is no undo or lives counter. Failed/stuck levels offer unlimited restart without losing stars or unlocked levels. There is no full game over. A match with no legal player move offers retry. Repeated positions trigger brave opponent play instead of failing the child.

Change `player.square`, `targets`, board and example together. Preserve the pair's minimum of 1 necessary quiet move(s). `minimumQuietMoves` is checked against the exact shortest solution. Bishops need targets on their square color. Pawns move forward into empty squares and capture diagonally; they have no double step.

Keep stable IDs when tuning. `icon` controls the level marker; add title translations in `src/language.rs` when renaming. World membership comes from `worlds/worlds.json`; the twenty levels belong to one world. Earlier designs are preserved in `docs/archive/v0.4-levels/`.
