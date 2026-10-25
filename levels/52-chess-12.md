# World 3, level 12: Open a File

Version: 0.10.0. Stable ID: `chess-12`.

## Teaching goal

Use open files to bring rooks into the game.

## Board and pieces

8 × 8. Player: king e1, bishop g2, pawn a2, pawn b2, pawn c2, pawn d2, pawn e4, pawn f2, pawn g3, pawn h2. Opponent: king e8, pawn a7, pawn b7, pawn c7, pawn d7, pawn e7, pawn f7, pawn g7, pawn h7.

Initial material advantage: 3 points.

Distinct starting formation: g2 → g3, c1 → g2, e2 → e4. Piece counts and material advantage are unchanged.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `-`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 2, with an occasional second-choice legal move every 6 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: a2 → a4, c2 → c4, f2 → f4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
