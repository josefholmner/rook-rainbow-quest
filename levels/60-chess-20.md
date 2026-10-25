# World 3, level 20: Grand Castle

Version: 0.8.0. Stable ID: `chess-20`.

## Teaching goal

Final boss! Both sides start with all sixteen pieces. Play a complete game of chess.

## Board and pieces

8 × 8. Player: king e1, queen d1, rook a1, rook h1, bishop c1, bishop f1, knight b1, knight g1, pawn a2, pawn b2, pawn c2, pawn d2, pawn e2, pawn f2, pawn g2, pawn h2. Opponent: king e8, queen d8, rook a8, rook h8, bishop c8, bishop f8, knight b8, knight g8, pawn a7, pawn b7, pawn c7, pawn d7, pawn e7, pawn f7, pawn g7, pawn h7.

Initial material advantage: 0 points.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `KQkq`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 3, with an occasional second-choice legal move every 9 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: a2 → a4, c2 → c4, e2 → e4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

A castle and FINAL BOSS label mark the full standard starting-position finale.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
