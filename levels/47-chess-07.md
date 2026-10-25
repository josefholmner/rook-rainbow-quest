# World 3, level 7: Bishop Buddies

Version: 0.10.0. Stable ID: `chess-07`.

## Teaching goal

Use bishops and your king together to support your other pieces.

## Board and pieces

8 × 8. Player: king e1, bishop b2, bishop g2, pawn a2, pawn b3, pawn c3. Opponent: king e8, pawn a7, pawn b7, pawn c7.

Initial material advantage: 6 points.

Distinct starting formation: f1 → g2, b2 → b3, c1 → b2, c2 → c3. Piece counts and material advantage are unchanged.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `-`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 1, with an occasional second-choice legal move every 5 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: g2 → b7, b7 → c6, a2 → a4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
