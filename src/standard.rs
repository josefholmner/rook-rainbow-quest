//! World 3 only: orthodox chess state, special moves, draws and bounded search.
//! Worlds 1 and 2 continue to use their original Battle rules.
use crate::{
    battle::{Action, Move, Outcome, Side, Unit},
    movement,
    rules::{Level, Square},
};
use cozy_chess::{Board, Color, GameStatus, Move as ChessMove, Piece};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Standard {
    pub board: Board,
    repetitions: HashMap<String, u16>,
    pub draw_reason: Option<&'static str>,
}
fn ui_square(s: cozy_chess::Square) -> Square {
    Square {
        x: s.file() as u8,
        y: s.rank() as u8,
    }
}
fn kind(p: Piece) -> &'static str {
    match p {
        Piece::King => "king",
        Piece::Queen => "queen",
        Piece::Rook => "rook",
        Piece::Bishop => "bishop",
        Piece::Knight => "knight",
        Piece::Pawn => "pawn",
    }
}
fn color(side: Side) -> Color {
    if side == Side::Player {
        Color::White
    } else {
        Color::Black
    }
}
fn ui_move(board: &Board, m: ChessMove) -> Move {
    let mut to = ui_square(m.to);
    // cozy-chess encodes castling as king-to-rook; children click c/g instead.
    if board.piece_on(m.from) == Some(Piece::King)
        && board.color_on(m.to) == Some(board.side_to_move())
    {
        to.x = if m.to.file() > m.from.file() { 6 } else { 2 };
    }
    Move {
        from: ui_square(m.from),
        to,
    }
}
fn moves(board: &Board) -> Vec<ChessMove> {
    let mut list = Vec::new();
    board.generate_moves(|batch| {
        list.extend(batch);
        false
    });
    list.sort_by_key(|m| (m.from, m.to, m.promotion));
    list
}
fn key(board: &Board) -> String {
    let fen = board.to_string();
    let mut fields: Vec<_> = fen.split_whitespace().take(4).collect();
    // Repetition identity includes en passant only when it can legally be played.
    if !moves(board).iter().any(|m| {
        board.piece_on(m.from) == Some(Piece::Pawn)
            && m.from.file() != m.to.file()
            && board.color_on(m.to).is_none()
    }) {
        fields[3] = "-";
    }
    fields.join(" ")
}
impl Standard {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let board: Board = fen
            .parse()
            .map_err(|e| format!("Invalid chess position: {e}"))?;
        let white = board.king(Color::White);
        let black = board.king(Color::Black);
        if (white.file() as i16 - black.file() as i16).abs() <= 1
            && (white.rank() as i16 - black.rank() as i16).abs() <= 1
        {
            return Err("Kings may not be adjacent".into());
        }
        let repetitions = HashMap::from([(key(&board), 1)]);
        Ok(Self {
            board,
            repetitions,
            draw_reason: None,
        })
    }
    pub fn from_level(level: &Level) -> Result<Self, String> {
        let mut cells = [[' '; 8]; 8];
        for (pieces, black) in [
            (
                std::iter::once(&level.player)
                    .chain(level.allies.iter())
                    .collect::<Vec<_>>(),
                false,
            ),
            (level.targets.iter().collect::<Vec<_>>(), true),
        ] {
            for p in pieces {
                let s = Square::parse(&p.square)?;
                let c = match p.piece.as_str() {
                    "king" => 'K',
                    "queen" => 'Q',
                    "rook" => 'R',
                    "bishop" => 'B',
                    "knight" => 'N',
                    "pawn" => 'P',
                    _ => return Err("Unknown chess piece".into()),
                };
                cells[s.y as usize][s.x as usize] = if black { c.to_ascii_lowercase() } else { c };
            }
        }
        let mut rows = Vec::new();
        for row in cells.iter().rev() {
            let mut line = String::new();
            let mut empty = 0;
            for c in row {
                if *c == ' ' {
                    empty += 1;
                } else {
                    if empty > 0 {
                        line.push(char::from(b'0' + empty));
                        empty = 0;
                    }
                    line.push(*c);
                }
            }
            if empty > 0 {
                line.push(char::from(b'0' + empty));
            }
            rows.push(line);
        }
        let rights = if level.castling_rights.is_empty() {
            "-"
        } else {
            &level.castling_rights
        };
        Self::from_fen(&format!(
            "{} w {} {} 0 1",
            rows.join("/"),
            rights,
            level.en_passant.as_deref().unwrap_or("-")
        ))
    }
    pub fn units(&self) -> Vec<Unit> {
        self.board
            .occupied()
            .into_iter()
            .map(|s| Unit {
                kind: kind(self.board.piece_on(s).unwrap()).into(),
                square: ui_square(s),
                side: if self.board.color_on(s) == Some(Color::White) {
                    Side::Player
                } else {
                    Side::Opponent
                },
            })
            .collect()
    }
    pub fn in_check(&self) -> bool {
        !self.board.checkers().is_empty()
    }
    pub fn legal_moves(&self, side: Side) -> Vec<Move> {
        if self.board.side_to_move() != color(side) {
            return vec![];
        }
        let mut result: Vec<_> = moves(&self.board)
            .into_iter()
            .map(|m| ui_move(&self.board, m))
            .collect();
        result.sort_by_key(|m| (m.from.y, m.from.x, m.to.y, m.to.x));
        result.dedup();
        result
    }
    pub fn promotion_choices(&self, movement: Move) -> Vec<&'static str> {
        moves(&self.board)
            .into_iter()
            .filter(|m| ui_move(&self.board, *m) == movement)
            .filter_map(|m| m.promotion.map(kind))
            .collect()
    }
    pub fn play(
        &mut self,
        movement: Move,
        promotion: Option<&str>,
    ) -> Result<Action, &'static str> {
        let found = moves(&self.board)
            .into_iter()
            .find(|m| {
                ui_move(&self.board, *m) == movement
                    && m.promotion
                        .is_none_or(|p| kind(p) == promotion.unwrap_or("queen"))
            })
            .ok_or("Try a glowing square")?;
        let before = self.board.occupied().len();
        let moving = kind(self.board.piece_on(found.from).unwrap()).to_owned();
        let side = if self.board.side_to_move() == Color::White {
            Side::Player
        } else {
            Side::Opponent
        };
        self.board.play(found);
        *self.repetitions.entry(key(&self.board)).or_default() += 1;
        Ok(Action {
            movement,
            kind: moving,
            side,
            captured: self.board.occupied().len() < before,
            promoted: found.promotion.is_some(),
        })
    }
    pub fn outcome(&mut self) -> Outcome {
        match self.board.status() {
            GameStatus::Won => {
                return if self.board.side_to_move() == Color::Black {
                    Outcome::Won
                } else {
                    Outcome::Lost
                }
            }
            GameStatus::Drawn => {
                self.draw_reason = Some(if self.board.halfmove_clock() >= 100 {
                    "Fifty-move draw"
                } else {
                    "Stalemate"
                });
                return Outcome::Stuck;
            }
            GameStatus::Ongoing => {}
        }
        let majors = self.board.pieces(Piece::Pawn)
            | self.board.pieces(Piece::Rook)
            | self.board.pieces(Piece::Queen);
        let minors = self.board.pieces(Piece::Bishop) | self.board.pieces(Piece::Knight);
        let bishops_only = self.board.pieces(Piece::Knight).is_empty() && {
            let mut parity = None;
            self.board.pieces(Piece::Bishop).into_iter().all(|s| {
                let p = (s.file() as u8 + s.rank() as u8) % 2;
                *parity.get_or_insert(p) == p
            })
        };
        self.draw_reason = if majors.is_empty() && (minors.len() <= 1 || bishops_only) {
            Some("Insufficient material")
        } else if self
            .repetitions
            .get(&key(&self.board))
            .copied()
            .unwrap_or(0)
            >= 3
        {
            Some("Threefold repetition")
        } else {
            None
        };
        if self.draw_reason.is_some() {
            Outcome::Stuck
        } else {
            Outcome::Playing
        }
    }
    pub fn choose(&self, safety: u8, overlook: bool) -> Option<Move> {
        let depth = if safety >= 2 { 2 } else { 1 };
        let mut budget = 1800;
        let mut candidates = Vec::new();
        for m in moves(&self.board) {
            // Automatic opponent promotion chooses a queen. Player choices stay available.
            if m.promotion.is_some_and(|p| p != Piece::Queen) {
                continue;
            }
            let mut next = self.board.clone();
            next.play(m);
            let score = -search(&next, depth - 1, &mut budget, 1);
            candidates.push((score, m));
        }
        candidates.sort_by_key(|(score, m)| (std::cmp::Reverse(*score), m.from, m.to, m.promotion));
        let index = if overlook && candidates.len() > 2 && candidates[0].0 < 90_000 {
            1
        } else {
            0
        };
        candidates.get(index).map(|(_, m)| ui_move(&self.board, *m))
    }
}
fn evaluate(board: &Board) -> i32 {
    let us = board.side_to_move();
    board
        .occupied()
        .into_iter()
        .map(|s| {
            let piece = board.piece_on(s).unwrap();
            let side = board.color_on(s).unwrap();
            let center = 6 - (s.file() as i32 - 3).abs() - (s.rank() as i32 - 3).abs();
            let advance = if side == Color::White {
                s.rank() as i32
            } else {
                7 - s.rank() as i32
            };
            let v = movement::value(kind(piece)) * 100
                + match piece {
                    Piece::Pawn => advance * 7,
                    Piece::Knight | Piece::Bishop => center * 4,
                    Piece::King => 0,
                    _ => center,
                };
            v * if side == us { 1 } else { -1 }
        })
        .sum()
}
fn search(board: &Board, depth: u8, budget: &mut usize, ply: i32) -> i32 {
    match board.status() {
        GameStatus::Won => return -100_000 + ply,
        GameStatus::Drawn => return 0,
        GameStatus::Ongoing => {}
    }
    if depth == 0 || *budget == 0 {
        return evaluate(board);
    }
    let mut best = i32::MIN;
    for m in moves(board) {
        if *budget == 0 {
            break;
        }
        *budget -= 1;
        let mut next = board.clone();
        next.play(m);
        best = best.max(-search(&next, depth - 1, budget, ply + 1));
    }
    if best == i32::MIN {
        evaluate(board)
    } else {
        best
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn m(from: &str, to: &str) -> Move {
        Move {
            from: Square::parse(from).unwrap(),
            to: Square::parse(to).unwrap(),
        }
    }
    fn perft(board: &Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        moves(board)
            .into_iter()
            .map(|m| {
                let mut b = board.clone();
                b.play(m);
                perft(&b, depth - 1)
            })
            .sum()
    }
    #[test]
    fn orthodox_move_generation_matches_standard_perft_positions() {
        let b = Board::default();
        assert_eq!((perft(&b, 1), perft(&b, 2), perft(&b, 3)), (20, 400, 8902));
        let b: Board = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
            .parse()
            .unwrap();
        assert_eq!((perft(&b, 1), perft(&b, 2)), (48, 2039));
    }
    #[test]
    fn pawn_double_steps_obey_starting_rank_and_both_blockers() {
        let mut game = Standard::from_fen(&Board::default().to_string()).unwrap();
        assert!(game.legal_moves(Side::Player).contains(&m("e2", "e4")));
        game.play(m("e2", "e4"), None).unwrap();
        assert!(game.legal_moves(Side::Opponent).contains(&m("e7", "e5")));
        game.play(m("e7", "e5"), None).unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e4", "e6")));
        let game = Standard::from_fen("4k3/8/8/8/8/4n3/4P3/4K3 w - - 0 1").unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e2", "e4")));
    }
    #[test]
    fn pins_checks_and_king_adjacency_are_respected_on_both_sides() {
        let game = Standard::from_fen("4r1k1/8/8/8/8/8/4R3/4K3 w - - 0 1").unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e2", "d2")));
        assert!(game.legal_moves(Side::Player).contains(&m("e2", "e8")));
        assert!(Standard::from_fen("8/8/8/8/8/8/4k3/4K3 w - - 0 1").is_err());
        let game = Standard::from_fen("4k3/4r3/8/8/8/8/8/4R1K1 b - - 0 1").unwrap();
        assert!(!game.legal_moves(Side::Opponent).contains(&m("e7", "d7")));
    }
    #[test]
    fn castling_moves_both_pieces_and_loses_rights_after_king_moves() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
        let mut game = Standard::from_fen(fen).unwrap();
        assert!(game.legal_moves(Side::Player).contains(&m("e1", "g1")));
        assert!(game.legal_moves(Side::Player).contains(&m("e1", "c1")));
        game.play(m("e1", "g1"), None).unwrap();
        assert_eq!(
            game.board.piece_on(cozy_chess::Square::G1),
            Some(Piece::King)
        );
        assert_eq!(
            game.board.piece_on(cozy_chess::Square::F1),
            Some(Piece::Rook)
        );
        let mut game = Standard::from_fen(fen).unwrap();
        for (from, to) in [("e1", "f1"), ("e8", "f8"), ("f1", "e1"), ("f8", "e8")] {
            game.play(m(from, to), None).unwrap();
        }
        assert!(!game.legal_moves(Side::Player).contains(&m("e1", "g1")));
        assert!(!game.legal_moves(Side::Player).contains(&m("e1", "c1")));
        let game = Standard::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1").unwrap();
        assert!(game.legal_moves(Side::Opponent).contains(&m("e8", "c8")));
    }
    #[test]
    fn castling_cannot_cross_check_or_jump_blockers() {
        let game = Standard::from_fen("4kr2/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e1", "g1")));
        assert!(game.legal_moves(Side::Player).contains(&m("e1", "c1")));
        let game = Standard::from_fen("4r1k1/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e1", "c1")));
        let game = Standard::from_fen(&Board::default().to_string()).unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e1", "g1")));
    }
    #[test]
    fn en_passant_removes_the_passed_pawn_expires_and_cannot_expose_king() {
        let fen = "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1";
        let mut game = Standard::from_fen(fen).unwrap();
        let action = game.play(m("e5", "d6"), None).unwrap();
        assert!(action.captured);
        assert_eq!(game.board.piece_on(cozy_chess::Square::D5), None);
        let mut game = Standard::from_fen(fen).unwrap();
        game.play(m("e1", "f1"), None).unwrap();
        game.play(m("e8", "f8"), None).unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e5", "d6")));
        let game = Standard::from_fen("k3r3/8/8/3pP3/8/8/8/4K3 w - d6 0 1").unwrap();
        assert!(!game.legal_moves(Side::Player).contains(&m("e5", "d6")));
        let other = Standard::from_fen("k3r3/8/8/3pP3/8/8/8/4K3 w - - 0 1").unwrap();
        assert_eq!(key(&game.board), key(&other.board));
    }
    #[test]
    fn promotion_has_four_choices_and_rejects_invalid_choice_without_mutation() {
        for piece in ["queen", "rook", "bishop", "knight"] {
            let mut game = Standard::from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1").unwrap();
            assert_eq!(game.promotion_choices(m("a7", "a8")).len(), 4);
            let before = game.board.clone();
            assert!(game.play(m("a7", "a8"), Some("king")).is_err());
            assert_eq!(game.board, before);
            assert!(game.play(m("a7", "a8"), Some(piece)).unwrap().promoted);
            assert_eq!(
                kind(game.board.piece_on(cozy_chess::Square::A8).unwrap()),
                piece
            );
        }
    }
    #[test]
    fn both_sides_can_be_checkmated_and_draws_never_win() {
        let mut game = Standard::from_fen(&Board::default().to_string()).unwrap();
        for (f, t) in [("f2", "f3"), ("e7", "e5"), ("g2", "g4"), ("d8", "h4")] {
            game.play(m(f, t), None).unwrap();
        }
        assert_eq!(game.outcome(), Outcome::Lost);
        assert_eq!(game.units().iter().filter(|u| u.kind == "king").count(), 2);
        let fen = "7k/5Q2/6K1/8/8/8/8/8 w - - 0 1";
        let mut game = Standard::from_fen(fen).unwrap();
        game.play(m("f7", "g7"), None).unwrap();
        assert_eq!(game.outcome(), Outcome::Won);
        let mut game = Standard::from_fen(fen).unwrap();
        game.play(m("f7", "e6"), None).unwrap();
        assert_eq!(game.outcome(), Outcome::Stuck);
        assert_eq!(game.draw_reason, Some("Stalemate"));
        let mut game = Standard::from_fen("7k/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
        assert_eq!(game.outcome(), Outcome::Stuck);
        assert_eq!(game.draw_reason, Some("Insufficient material"));
    }
    #[test]
    fn opposite_bishops_and_rook_pairs_are_not_automatically_dead_in_regular_chess() {
        let mut game = Standard::from_fen("7k/5K1b/7B/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(game.outcome(), Outcome::Playing);
        game.play(m("h6", "g7"), None).unwrap();
        assert_eq!(game.outcome(), Outcome::Won);
        let mut game = Standard::from_fen("7k/8/8/8/8/8/r7/R5K1 w - - 0 1").unwrap();
        assert_eq!(game.outcome(), Outcome::Playing);
        game.play(m("a1", "a2"), None).unwrap();
        assert_eq!(game.outcome(), Outcome::Playing);
    }
    #[test]
    fn draw_claims_include_initial_position_and_fifty_move_counter() {
        let mut game = Standard::from_fen(&Board::default().to_string()).unwrap();
        for _ in 0..2 {
            for (f, t) in [("g1", "f3"), ("g8", "f6"), ("f3", "g1"), ("f6", "g8")] {
                game.play(m(f, t), None).unwrap();
            }
        }
        assert_eq!(game.outcome(), Outcome::Stuck);
        assert_eq!(game.draw_reason, Some("Threefold repetition"));
        let mut game = Standard::from_fen("7k/8/8/8/8/8/8/KQ6 w - - 99 1").unwrap();
        game.play(m("b1", "b2"), None).unwrap();
        assert_eq!(game.outcome(), Outcome::Stuck);
        assert_eq!(game.draw_reason, Some("Fifty-move draw"));
    }
}
