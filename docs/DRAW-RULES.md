# Dead positions and draws — v0.8.1

## Sunflower Meadows (World 1)

The ten alternating capture matches stop with a localized draw/retry dialog when:

1. All surviving friendly pieces are bishops, and at least one enemy bishop lives on a square color that none of those friendly bishops can reach. Bishops cannot change square color, so capturing every opponent is impossible. This also works with several bishops and other enemy pieces still present.
2. Neither team has any legal move, for example two opposing pawns blocked face to face. An immobile enemy still passes normally when the player can continue.
3. The same arrangement, with the player to move, occurs three times. The starting arrangement counts. The second occurrence still activates the existing brave opponent; the third offers a retry.
4. There have been 100 individual moves without a capture or pawn move (50 turns each). Either kind of progress resets this counter. A win or loss takes priority over a draw.

The first two are proofs that progress/victory is impossible. The last two are explicit practical draw limits: they end a stalled match without claiming that no winning sequence exists. Checks run at creation and after moves, rather than every rendered frame. Opponent passes also record repetition.

Rook versus rook remains playable: a rook can capture the other rook, especially with the gentle opponent's existing willingness to offer a capture. A friendly rook, knight, pawn with promotion potential, or bishop on the required color prevents the bishop-color rule from firing. The detector deliberately does not claim to solve every possible blocked arrangement.

The stationary lessons (levels 1–10) retain their existing exact puzzle/dead-end checks. No board layouts changed.

## Checkmate Castle (World 2)

Existing checks and stalemate behavior stay in use: an opponent with no legal reply loses when checked; otherwise the child can retry. World 1's kingless material rule and draw counters do not apply here.

## Royal Chess Kingdom (World 3)

Existing regular-chess draw rules remain: stalemate, insufficient material (bare kings, one minor piece total, or only bishops all confined to one square color), automatically claimed threefold repetition and fifty-move draws. Opposite-colored bishops are not automatically dead when kings are present: an opposing bishop can block its own king's escape and allow mate. A regression demonstrates such a legal mate. Rooks on both teams likewise do not imply a dead position.

The distinction follows [FIDE's dead-position definition, article 5.2.2](https://handbook.fide.com/chapter/e012023): no legal sequence can produce mate, rather than a draw assuming optimal play. The game uses conservative sufficient tests; it does not exhaustively solve all chess positions.

All retries preserve earned stars and allow unlimited restarts. A draw never awards a completion star.
