# 08. Pawn's Garden

Version 0.6. Stable ID: `lesson-pawn-02`. World 1: The First Adventure. Trail area: Piece Garden. Runtime source: `08-pawn-s-garden.json`.

One target is behind you. Reach the far edge, become a queen, and return to capture it.

## Setup

- Board: 5×5, algebraic coordinates from bottom left.
- Friendly: pawn b1.
- Stationary targets: pawn c3, pawn b4, pawn a1.
- Map icon: Cream pawn, 2 small gold star(s) for its lesson pair, numbered badge and independent completion star.

```text
5 . . . . .
4 . p . . .
3 . . p . .
2 . . . . .
1 p P . . .
  a b c d e
```

Uppercase = cream friendly, lowercase = purple opponent. R/r rook, B/b bishop, N/n knight, P/p pawn, Q/q queen.

## Rules and learning goal

Capture all 3 stationary targets with the one friendly piece. Targets neither move nor attack. Empty-square moves are allowed. The exact shortest winning route is 6 moves, including 3 quiet moves. `minimumQuietMoves: 2` requires **at least 2 more moves than targets** on every winning route. A deliberately long example alone does not satisfy the rule; breadth-first search verifies the minimum.

**Promotion discovery:** target a1 is behind the starting pawn at b1. A pawn can never capture it directly. Capture c3 and b4, reach b5 to become a queen, then return for a1. This setup is unsolvable with promotion disabled, which has its own regression test. The shortest solution takes six moves for three opponents (three extra moves, exceeding the required two).

Automatic queen promotion is enabled (`pawnPromotion: true`). Rooks, bishops and queens cannot jump; knights can. No kings, check, checkmate, en passant, timers or move limits.

Hints follow a shortest complete winning route, recalculated and cached after every move, including detours and promotion when enabled.

## Tested winning example

b1 → b2 → c3 → b4 → b5 → b1 → a1

The first square is the starting square; each arrow is one move. Both this authored route and the independently computed shortest route are tested.

## Retry and tuning

There is no undo or lives counter. Failed/stuck levels offer unlimited restart without losing stars or unlocked levels. There is no full game over. A match with no legal player move offers retry. Repeated positions trigger brave opponent play instead of failing the child.

Change `player.square`, `targets`, board and example together. Preserve the pair's minimum of 2 necessary quiet move(s). `minimumQuietMoves` is checked against the exact shortest solution. Bishops need targets on their square color. Pawns move forward into empty squares and capture diagonally; they have no double step.

Keep stable IDs when tuning. `icon` controls the level marker; add title translations in `src/language.rs` when renaming. World membership comes from `worlds/worlds.json`; the twenty levels belong to one world. Earlier designs are preserved in `docs/archive/v0.4-levels/`.
