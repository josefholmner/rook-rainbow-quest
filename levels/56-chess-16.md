# World 3, level 16: A Fair Fight

Version: 0.10.0. Stable ID: `chess-16`.

## Teaching goal

Both teams have plenty of helpers now. Look before you capture.

## Board and pieces

8 × 8. Player: king e1, bishop c1, bishop f1, knight c3, knight f3, pawn a2, pawn b2, pawn c2, pawn d4, pawn e3, pawn f2, pawn g2, pawn h2. Opponent: king e8, bishop c8, bishop f8, knight c6, pawn a7, pawn b7, pawn c5, pawn d7, pawn e7, pawn f7, pawn g7, pawn h7.

Initial material advantage: 3 points.

Distinct starting formation: b1 → c3, g1 → f3, d2 → d4, e2 → e3. Enemy development: b8 → c6, c7 → c5. Piece counts and material advantage are unchanged.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `-`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 3, with an occasional second-choice legal move every 8 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: d4 → c5, f1 → d3, a2 → a4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
