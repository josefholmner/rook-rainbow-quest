# World 3, level 18: Both Wings Ready

Version: 0.10.0. Stable ID: `chess-18`.

## Teaching goal

Use both sides of the board, and castle when it is safe.

## Board and pieces

8 × 8. Player: king e1, rook a1, bishop f4, bishop f1, knight b1, knight f3, pawn a2, pawn b2, pawn c2, pawn d4, pawn e3, pawn f2, pawn g2, pawn h2. Opponent: king e8, rook a8, bishop c8, bishop f8, knight b8, knight f6, pawn a7, pawn b7, pawn c7, pawn d5, pawn e7, pawn f7, pawn g7, pawn h7.

Initial material advantage: 0 points.

Distinct starting formation: g1 → f3, c1 → f4, e2 → e3, d2 → d4. Enemy development: g8 → f6, d7 → d5. Piece counts and material advantage are unchanged.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `Qq`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 3, with an occasional second-choice legal move every 8 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: f4 → c7, f1 → b5, a2 → a4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
