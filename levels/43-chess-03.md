# World 3, level 3: Castle to Safety

Version: 0.8.0. Stable ID: `chess-03`.

## Teaching goal

Castle by selecting your king, then c1 or g1. Both king and rook must be unmoved and the king must stay safe.

## Board and pieces

8 × 8. Player: king e1, rook a1, rook h1, pawn e2. Opponent: king e8, pawn e7.

Initial material advantage: 10 points.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `KQ`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 0, with an occasional second-choice legal move every 3 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: e2 → e4, a1 → d1, h1 → h4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
