# 07. Hello, Pawn!

Version 0.8.4. Stable ID: `lesson-pawn-01`. World 1: The First Adventure. Trail area: Piece Garden. Runtime source: `07-hello-pawn.json`.

Plan a route with at least 1 quiet move(s), then capture every pawn.

## Setup

- Board: 5×5, algebraic coordinates from bottom left.
- Friendly: pawn b1.
- Stationary targets: pawn c3, pawn b4.
- Map icon: Cream pawn, 1 small gold star(s) for its lesson pair, numbered badge and independent completion star.

```text
5 . . . . .
4 . p . . .
3 . . p . .
2 . . . . .
1 . P . . .
  a b c d e
```

Uppercase = cream friendly, lowercase = purple opponent. R/r rook, B/b bishop, N/n knight, P/p pawn, Q/q queen.

## Rules and learning goal

Capture all 2 stationary targets with the one friendly piece. Targets neither move nor attack. Empty-square moves are allowed. The exact shortest winning route is 3 moves, including 1 quiet moves. `minimumQuietMoves: 1` requires **at least 1 more moves than targets** on every winning route. A deliberately long example alone does not satisfy the rule; breadth-first search verifies the minimum.

Promotion is enabled (`pawnPromotion: true`). Reaching rank 5 turns the friendly pawn into a queen, allowing it to return for a target it passed. Rooks, bishops and queens cannot jump; knights can. No kings, check, checkmate, en passant, timers or move limits.

Hints follow a shortest complete winning route, recalculated and cached after every move, including detours and promotion when enabled.

## Tested winning example

b1 → b2 → c3 → b4

Also tested: b1 → b2 → c3 → c4 → c5 (promote to queen) → b4. This optional detour must stay playable and must not trigger a premature failure or completion. Promotion is optional here and remains required by the second pawn lesson. The shortest route remains three moves, with one necessary quiet move.

The first square is the starting square; each arrow is one move. Both this authored route and the independently computed shortest route are tested.

## Retry and tuning

There is no undo or lives counter. Failed/stuck levels offer unlimited restart without losing stars or unlocked levels. There is no full game over. A match with no legal player move offers retry. Repeated positions trigger brave opponent play instead of failing the child.

Change `player.square`, `targets`, board and example together. Preserve the pair's minimum of 1 necessary quiet move(s). `minimumQuietMoves` is checked against the exact shortest solution. Bishops need targets on their square color. Pawns move forward into empty squares and capture diagonally; they have no double step.

Keep stable IDs when tuning. `icon` controls the level marker; add title translations in `src/language.rs` when renaming. World membership comes from `worlds/worlds.json`; the twenty levels belong to one world. Earlier designs are preserved in `docs/archive/v0.4-levels/`.
