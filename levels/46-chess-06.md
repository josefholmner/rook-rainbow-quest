# World 3, level 6: Knights Join In

Version: 0.8.0. Stable ID: `chess-06`.

## Teaching goal

Bring your knight into the game without leaving your king in check.

## Board and pieces

8 × 8. Player: king e1, bishop c1, knight b1, pawn a2, pawn b2, pawn c2. Opponent: king e8, pawn a7, pawn b7, pawn c7.

Initial material advantage: 6 points.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `-`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 1, with an occasional second-choice legal move every 4 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: a2 → a4, c2 → c4, a4 → b5. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
