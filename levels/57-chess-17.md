# World 3, level 17: Opening Practice

Version: 0.10.0. Stable ID: `chess-17`.

## Teaching goal

Develop your pieces and keep your king safe.

## Board and pieces

8 × 8. Player: king e1, rook a1, bishop c1, bishop e2, knight c3, knight g1, pawn a2, pawn b2, pawn c2, pawn d4, pawn e3, pawn f2, pawn g2, pawn h2. Opponent: king e8, rook a8, rook h8, bishop c8, bishop f8, pawn a6, pawn b7, pawn c7, pawn d7, pawn e7, pawn f7, pawn g7, pawn h7.

Initial material advantage: 1 points.

Distinct starting formation: e2 → e3, f1 → e2, b1 → c3, d2 → d4. Enemy development: a7 → a6. Piece counts and material advantage are unchanged.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `Qkq`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 3, with an occasional second-choice legal move every 8 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: a2 → a4, f2 → f4, h2 → h4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
