# World 3, level 1: Hello, Your Majesty!

Version: 0.8.0. Stable ID: `chess-01`.

## Teaching goal

Your king is part of the team. Protect it while your queen and rook hunt for checkmate.

## Board and pieces

8 × 8. Player: king e1, queen d1, rook h1. Opponent: king e8.

Initial material advantage: 14 points.

## Rules and tuning

Regular chess: both kings must remain safe; checkmate wins. Pawn double steps from the starting rank, en passant, castling and four promotion choices are supported. Click the king then c/g to castle. Castling rights at this starting position: `K`. Draws offer a free retry; neither draws nor losses award a star. Threefold repetition and fifty-move claims are made automatically for both players. No undo, lives or full game over.

Opponent: safety 0, with an occasional second-choice legal move every 3 replies. Search is bounded; even gentle moves must respect check.

## Tested opening

3 tested player moves against the configured opponent: d1 → d4, h1 → h4, h4 → e4. It is an opening example, not an automatically completed level or a claim of a forced win. The child must play to checkmate to earn a star.

## Icon and future tweaks

Two king-team icons distinguish regular chess from the earlier worlds.

Edit only this level’s JSON and description when tuning. Keep its stable ID to preserve saves. Regenerate openings with `cargo run --release --example author_matches -- --world3`, then run the full tests.
