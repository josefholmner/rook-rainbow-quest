//! Deterministic, gentle capture-only opponent and alternating turn state.
use crate::{
    movement,
    rules::{Level, Square},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Side {
    Player,
    Opponent,
}
impl Side {
    pub fn other(self) -> Self {
        match self {
            Self::Player => Self::Opponent,
            Self::Opponent => Self::Player,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit {
    pub kind: String,
    pub square: Square,
    pub side: Side,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Playing,
    Won,
    Lost,
    Stuck,
}
#[derive(Clone, Debug)]
pub struct Action {
    pub movement: Move,
    pub kind: String,
    pub side: Side,
    pub captured: bool,
    pub promoted: bool,
}
#[derive(Clone, Debug)]
pub struct Battle {
    pub level: Level,
    pub units: Vec<Unit>,
    pub turn: Side,
    pub outcome: Outcome,
    pub opponent_turns: u32,
    repetitions: HashMap<Vec<Unit>, u8>,
    pub quiet_rounds: u32,
    pub brave: bool,
    pub draw_reason: Option<&'static str>,
    quiet_plies: u16,
    pub standard: Option<crate::standard::Standard>,
}
impl Battle {
    pub fn new(level: Level) -> Self {
        let mut units = vec![Unit {
            kind: level.player.piece.clone(),
            square: Square::parse(&level.player.square).unwrap(),
            side: Side::Player,
        }];
        units.extend(level.allies.iter().map(|p| Unit {
            kind: p.piece.clone(),
            square: Square::parse(&p.square).unwrap(),
            side: Side::Player,
        }));
        units.extend(level.targets.iter().map(|p| Unit {
            kind: p.piece.clone(),
            square: Square::parse(&p.square).unwrap(),
            side: Side::Opponent,
        }));
        let mut battle = Self {
            standard: if level.is_standard() {
                Some(
                    crate::standard::Standard::from_level(&level)
                        .expect("Validated chess position"),
                )
            } else {
                None
            },
            level,
            units,
            turn: Side::Player,
            outcome: Outcome::Playing,
            opponent_turns: 0,
            repetitions: HashMap::new(),
            quiet_rounds: 0,
            brave: false,
            draw_reason: None,
            quiet_plies: 0,
        };
        if !battle.level.is_royal() {
            battle.repetitions.insert(battle.repetition_key(), 1);
        }
        battle.detect_capture_draw();
        battle
    }
    fn repetition_key(&self) -> Vec<Unit> {
        let mut key = self.units.clone();
        if self.level.is_royal() {
            return key;
        }
        key.sort_by(|a, b| {
            (a.side == Side::Player, &a.kind, a.square.x, a.square.y).cmp(&(
                b.side == Side::Player,
                &b.kind,
                b.square.x,
                b.square.y,
            ))
        });
        key
    }
    /// Sound material proofs for kingless capture matches, plus explicit draw
    /// limits. These limits end a stalled game; they do not prove perfect play.
    fn detect_capture_draw(&mut self) {
        if self.standard.is_some() || self.level.is_checkmate() || self.outcome != Outcome::Playing
        {
            return;
        }
        let players: Vec<_> = self
            .units
            .iter()
            .filter(|u| u.side == Side::Player)
            .collect();
        let bishop_color_dead = !players.is_empty()
            && players.iter().all(|u| u.kind == "bishop")
            && self.units.iter().any(|target| {
                target.side == Side::Opponent
                    && target.kind == "bishop"
                    && !players.iter().any(|p| {
                        (p.square.x + p.square.y) % 2 == (target.square.x + target.square.y) % 2
                    })
            });
        self.draw_reason = if bishop_color_dead {
            Some("A bishop is out of reach.")
        } else if self.legal_moves(Side::Player).is_empty()
            && self.legal_moves(Side::Opponent).is_empty()
        {
            Some("Neither team can move.")
        } else if self
            .repetitions
            .get(&self.repetition_key())
            .copied()
            .unwrap_or(0)
            >= 3
            && self.turn == Side::Player
        {
            Some("Threefold repetition")
        } else if self.quiet_plies >= 100 {
            Some("Fifty-move draw")
        } else {
            None
        };
        if self.draw_reason.is_some() {
            self.outcome = Outcome::Stuck;
        }
    }
    pub fn in_check(&self) -> bool {
        if let Some(game) = &self.standard {
            return game.in_check();
        }
        self.level.is_checkmate() && king_in_check(&self.level.board, &self.units)
    }
    pub fn king_in_check(&self, side: Side) -> bool {
        if self.standard.is_some() {
            self.turn == side && self.in_check()
        } else {
            side == Side::Opponent && self.in_check()
        }
    }
    pub fn promotion_choices(&self, m: Move) -> Vec<&'static str> {
        self.standard
            .as_ref()
            .map_or_else(Vec::new, |s| s.promotion_choices(m))
    }
    pub fn play_promoted(&mut self, m: Move, promotion: &str) -> Result<Action, &'static str> {
        self.apply_standard(m, Side::Player, Some(promotion))
    }
    fn apply_standard(
        &mut self,
        m: Move,
        side: Side,
        promotion: Option<&str>,
    ) -> Result<Action, &'static str> {
        if self.outcome != Outcome::Playing || self.turn != side {
            return Err("Wait for your turn");
        }
        let game = self.standard.as_mut().ok_or("Not a chess match")?;
        let action = game.play(m, promotion)?;
        self.units = game.units();
        self.turn = side.other();
        self.outcome = game.outcome();
        Ok(action)
    }
    pub fn count(&self, side: Side) -> usize {
        self.units.iter().filter(|u| u.side == side).count()
    }
    pub fn legal_moves(&self, side: Side) -> Vec<Move> {
        if let Some(game) = &self.standard {
            return game.legal_moves(side);
        }
        let mut result = vec![];
        for unit in self.units.iter().filter(|u| u.side == side) {
            for y in 0..self.level.board.height {
                for x in 0..self.level.board.width {
                    let to = Square { x, y };
                    if self.legal_for(unit, to) {
                        result.push(Move {
                            from: unit.square,
                            to,
                        });
                    }
                }
            }
        }
        result
    }
    fn legal_for(&self, unit: &Unit, to: Square) -> bool {
        if self
            .units
            .iter()
            .any(|u| u.square == to && u.kind == "king")
        {
            return false;
        }
        if !legal_on(&self.level.board, &self.units, unit, to) {
            return false;
        }
        if self.level.is_checkmate() && unit.side == Side::Opponent {
            let (after, _) = self.after_move(
                Move {
                    from: unit.square,
                    to,
                },
                unit.side,
            );
            return !king_in_check(&self.level.board, &after);
        }
        true
    }
    pub fn legal_move(&self, m: Move, side: Side) -> bool {
        if self.standard.is_some() {
            return self.legal_moves(side).contains(&m);
        }
        self.units
            .iter()
            .find(|u| u.side == side && u.square == m.from)
            .is_some_and(|u| self.legal_for(u, m.to))
    }
    pub fn brave_after(&self) -> u32 {
        if self.count(Side::Opponent) == 1 {
            self.level.opponent.brave_after.min(2)
        } else {
            self.level.opponent.brave_after
        }
    }
    fn after_move(&self, m: Move, side: Side) -> (Vec<Unit>, bool) {
        let mut units = self.units.clone();
        units.retain(|u| !(u.square == m.to && u.side != side));
        let promoted = relocate(&mut units, m, side, self.level.board.height);
        (units, promoted)
    }
    fn offered_capture(&self, m: Move) -> bool {
        let (units, _) = self.after_move(m, Side::Opponent);
        units
            .iter()
            .filter(|u| u.side == Side::Player)
            .any(|u| legal_on(&self.level.board, &units, u, m.to))
    }

    fn apply(&mut self, m: Move, side: Side) -> Result<Action, &'static str> {
        if self.standard.is_some() {
            return self.apply_standard(m, side, None);
        }
        if self.outcome != Outcome::Playing || self.turn != side {
            return Err("Wait for your turn");
        }
        let unit = self
            .units
            .iter()
            .find(|u| u.square == m.from && u.side == side)
            .cloned()
            .ok_or("Choose one of your pieces")?;
        if !self.legal_for(&unit, m.to) {
            return Err("Try a glowing square");
        }
        let captured = self
            .units
            .iter()
            .any(|u| u.square == m.to && u.side != side);
        self.units.retain(|u| !(u.square == m.to && u.side != side));
        let promoted = relocate(&mut self.units, m, side, self.level.board.height);
        self.turn = side.other();
        self.quiet_plies = if captured || unit.kind == "pawn" {
            0
        } else {
            self.quiet_plies.saturating_add(1)
        };
        self.outcome = if self.count(Side::Opponent) == 0 && !self.level.is_checkmate() {
            Outcome::Won
        } else if self.count(Side::Player) == 0 {
            Outcome::Lost
        } else {
            Outcome::Playing
        };
        if self.level.is_checkmate()
            && self.outcome == Outcome::Playing
            && self.turn == Side::Opponent
            && self.legal_moves(Side::Opponent).is_empty()
        {
            self.outcome = if self.in_check() {
                Outcome::Won
            } else {
                Outcome::Stuck
            };
        }
        if captured {
            self.quiet_rounds = 0;
            self.brave = false;
            self.repetitions.clear();
        } else if side == Side::Opponent {
            self.quiet_rounds += 1;
        }
        if self.quiet_rounds >= self.brave_after() {
            self.brave = true;
        }
        if self.outcome == Outcome::Playing && self.turn == Side::Player {
            // Check a bilateral blockade before the existing player-no-move retry.
            self.detect_capture_draw();
            if self.legal_moves(Side::Player).is_empty() {
                self.outcome = Outcome::Stuck;
            }
            let count = self.repetitions.entry(self.repetition_key()).or_default();
            *count += 1;
            if *count >= 2 {
                self.brave = true;
            }
        }
        self.detect_capture_draw();
        Ok(Action {
            movement: m,
            kind: unit.kind,
            side,
            captured,
            promoted,
        })
    }
    pub fn play(&mut self, m: Move) -> Result<Action, &'static str> {
        self.apply(m, Side::Player)
    }
    pub fn opponent_reply(&mut self) -> Option<Action> {
        if self.turn != Side::Opponent || self.outcome != Outcome::Playing {
            return None;
        }
        let Some(m) = self.choose_opponent() else {
            if self.level.is_checkmate() {
                self.outcome = if self.in_check() {
                    Outcome::Won
                } else {
                    Outcome::Stuck
                };
                return None;
            }
            self.turn = Side::Player;
            *self.repetitions.entry(self.repetition_key()).or_default() += 1;
            self.detect_capture_draw();
            if self.legal_moves(Side::Player).is_empty() {
                self.outcome = Outcome::Stuck;
            }
            return None;
        };
        self.opponent_turns += 1;
        self.apply(m, Side::Opponent).ok()
    }
    pub fn choose_opponent(&self) -> Option<Move> {
        if let Some(game) = &self.standard {
            let miss = self.level.opponent.overlook_every > 0
                && (self.opponent_turns + 1).is_multiple_of(self.level.opponent.overlook_every);
            return game.choose(self.level.opponent.safety, miss);
        }
        let moves = self.legal_moves(Side::Opponent);
        if moves.is_empty() {
            return None;
        }
        if self.brave || self.quiet_rounds >= self.brave_after() {
            let offered: Vec<_> = moves
                .iter()
                .copied()
                .filter(|m| self.offered_capture(*m))
                .collect();
            let candidates = if offered.is_empty() { moves } else { offered };
            // Keep offering a capture until somebody takes it. Otherwise approach
            // and trade, with no safety penalty or periodic avoidance of captures.
            return candidates.into_iter().max_by_key(|m| {
                (
                    self.move_score(*m, Side::Opponent, 0),
                    std::cmp::Reverse((m.from.y, m.from.x, m.to.y, m.to.x)),
                )
            });
        }
        let overlook = self.level.opponent.overlook_every;
        let miss = overlook > 0 && (self.opponent_turns + 1).is_multiple_of(overlook);
        let quiet: Vec<_> = moves
            .iter()
            .copied()
            .filter(|m| {
                !self
                    .units
                    .iter()
                    .any(|u| u.square == m.to && u.side == Side::Player)
            })
            .collect();
        let candidates = if miss && !quiet.is_empty() {
            quiet
        } else {
            moves
        };
        candidates.into_iter().max_by_key(|m| {
            (
                self.move_score(*m, Side::Opponent, self.level.opponent.safety),
                std::cmp::Reverse((m.from.y, m.from.x, m.to.y, m.to.x)),
            )
        })
    }
    fn move_score(&self, m: Move, side: Side, safety: u8) -> i32 {
        let unit = self
            .units
            .iter()
            .find(|u| u.square == m.from && u.side == side)
            .unwrap();
        let captured = self
            .units
            .iter()
            .find(|u| u.square == m.to && u.side != side)
            .map_or(0, |u| movement::value(&u.kind));
        let (after, promoted) = self.after_move(m, side);
        let threatened = after
            .iter()
            .filter(|u| u.side != side)
            .any(|u| legal_on(&self.level.board, &after, u, m.to));
        let distance = after
            .iter()
            .filter(|u| u.side != side)
            .map(|u| m.to.x.abs_diff(u.square.x) as i32 + m.to.y.abs_diff(u.square.y) as i32)
            .min()
            .unwrap_or(0);
        captured * 100 + if promoted { 96 } else { 0 }
            - if threatened {
                movement::value(if promoted { "queen" } else { &unit.kind }) * safety as i32 * 22
            } else {
                0
            }
            - distance * 3
    }
    pub fn hint(&self) -> Option<Move> {
        if self.outcome != Outcome::Playing || self.turn != Side::Player {
            return None;
        }
        if self.level.is_royal() {
            // Follow the authored teaching route while it still matches the board.
            // This only compares bounded, prevalidated moves; no live tree search.
            let mut replay = Self::new(self.level.clone());
            for step in &self.level.example_turns {
                let movement = Move {
                    from: Square::parse(&step.from).ok()?,
                    to: Square::parse(&step.to).ok()?,
                };
                if replay.units == self.units
                    && replay.opponent_turns == self.opponent_turns
                    && replay
                        .standard
                        .as_ref()
                        .zip(self.standard.as_ref())
                        .is_none_or(|(a, b)| a.board.same_position(&b.board))
                {
                    return Some(movement);
                }
                replay.play(movement).ok()?;
                replay.opponent_reply();
            }
        }
        if let Some(game) = &self.standard {
            return game.choose(3, false);
        }
        self.legal_moves(Side::Player).into_iter().max_by_key(|m| {
            (
                if self.level.is_checkmate() {
                    let mut next = self.clone();
                    let _ = next.play(*m);
                    next.position_score()
                } else {
                    self.move_score(*m, Side::Player, 3)
                },
                std::cmp::Reverse((m.from.y, m.from.x, m.to.y, m.to.x)),
            )
        })
    }
    fn position_score(&self) -> i32 {
        if self.outcome == Outcome::Won {
            return 1_000_000;
        }
        if matches!(self.outcome, Outcome::Lost | Outcome::Stuck) {
            return -1_000_000;
        }
        let material = self
            .units
            .iter()
            .map(|u| movement::value(&u.kind) * if u.side == Side::Player { 1 } else { -1 })
            .sum::<i32>();
        let proximity = self
            .units
            .iter()
            .filter(|u| u.side == Side::Opponent)
            .map(|e| {
                self.units
                    .iter()
                    .filter(|u| u.side == Side::Player)
                    .map(|p| {
                        p.square.x.abs_diff(e.square.x) as i32
                            + p.square.y.abs_diff(e.square.y) as i32
                    })
                    .min()
                    .unwrap_or(20)
            })
            .sum::<i32>();
        let mate_pressure = if self.level.is_checkmate() {
            let king = self.units.iter().find(|u| u.kind == "king").unwrap();
            let edge = king
                .square
                .x
                .min(self.level.board.width - 1 - king.square.x)
                .min(
                    king.square
                        .y
                        .min(self.level.board.height - 1 - king.square.y),
                );
            -(self.legal_moves(Side::Opponent).len() as i32) * 90 - edge as i32 * 60
                + if self.in_check() { 80 } else { 0 }
        } else {
            0
        };
        material * 1000 - self.count(Side::Opponent) as i32 * 150 - proximity * 10 + mate_pressure
    }
    /// Offline level authoring, not the live UI: find a bounded winning witness
    /// against this exact opponent policy so each match can be regression tested.
    pub fn winning_line(&self, max_rounds: usize, beam_width: usize) -> Option<Vec<Move>> {
        let mut frontier = vec![(self.clone(), Vec::new())];
        let mut seen = HashSet::new();
        for _ in 0..max_rounds {
            let mut next = vec![];
            for (game, path) in frontier {
                for m in game.legal_moves(Side::Player) {
                    let mut n = game.clone();
                    let mut path = path.clone();
                    n.play(m).ok()?;
                    path.push(m);
                    if n.outcome == Outcome::Won {
                        return Some(path);
                    }
                    n.opponent_reply();
                    if n.outcome == Outcome::Won {
                        return Some(path);
                    }
                    if n.outcome != Outcome::Playing {
                        continue;
                    }
                    if seen.insert((
                        n.units.clone(),
                        n.opponent_turns % n.level.opponent.overlook_every.max(1),
                        n.quiet_rounds.min(n.brave_after()),
                        n.brave,
                    )) {
                        next.push((n, path));
                    }
                }
            }
            next.sort_by_key(|(g, _)| std::cmp::Reverse(g.position_score()));
            next.truncate(beam_width);
            frontier = next;
        }
        None
    }
}

/// Attacks include defended friendly squares and pawn diagonals, never pawn pushes.
fn attacked(board: &crate::rules::Board, units: &[Unit], square: Square, side: Side) -> bool {
    units.iter().filter(|u| u.side == side).any(|u| {
        if u.kind == "pawn" {
            let forward = if side == Side::Player { 1 } else { -1 };
            return u.square.x.abs_diff(square.x) == 1
                && square.y as i16 - u.square.y as i16 == forward;
        }
        movement::legal(&u.kind, u.square, square, board, 1, |q| {
            if q == square {
                Some(false)
            } else {
                units.iter().find(|u| u.square == q).map(|_| false)
            }
        })
    })
}
fn king_in_check(board: &crate::rules::Board, units: &[Unit]) -> bool {
    units
        .iter()
        .find(|u| u.kind == "king" && u.side == Side::Opponent)
        .is_some_and(|u| attacked(board, units, u.square, Side::Player))
}

fn legal_on(board: &crate::rules::Board, units: &[Unit], unit: &Unit, to: Square) -> bool {
    movement::legal(
        &unit.kind,
        unit.square,
        to,
        board,
        if unit.side == Side::Player { 1 } else { -1 },
        |q| {
            units
                .iter()
                .find(|u| u.square == q)
                .map(|u| u.side == unit.side)
        },
    )
}

fn relocate(units: &mut [Unit], m: Move, side: Side, height: u8) -> bool {
    let unit = units
        .iter_mut()
        .find(|u| u.square == m.from && u.side == side)
        .unwrap();
    unit.square = m.to;
    let promoted = unit.kind == "pawn"
        && if side == Side::Player {
            m.to.y + 1 == height
        } else {
            m.to.y == 0
        };
    if promoted {
        unit.kind = "queen".into();
    }
    promoted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dead_capture_position_is_detected_when_a_match_is_created() {
        let mut level = crate::campaign::levels()[10].clone();
        level.player = crate::rules::Piece {
            piece: "bishop".into(),
            square: "a1".into(),
        };
        level.allies.clear();
        level.targets = vec![crate::rules::Piece {
            piece: "bishop".into(),
            square: "b1".into(),
        }];
        let b = Battle::new(level);
        assert_eq!(b.outcome, Outcome::Stuck);
        assert_eq!(b.draw_reason, Some("A bishop is out of reach."));
    }
    #[test]
    fn bishops_cannot_capture_an_enemy_bishop_on_an_uncovered_color() {
        let mut b = game(&[
            ("bishop", "a1", Side::Player),
            ("bishop", "c3", Side::Player),
            ("bishop", "b1", Side::Opponent),
            ("rook", "d4", Side::Opponent),
        ]);
        b.detect_capture_draw();
        assert_eq!(b.outcome, Outcome::Stuck);
        assert_eq!(b.draw_reason, Some("A bishop is out of reach."));
        assert!(b.play(mv("a1", "b2")).is_err());
        assert!(b.opponent_reply().is_none());
    }
    #[test]
    fn matching_bishops_rooks_and_promotion_chances_remain_playable() {
        for (kind, square) in [
            ("rook", "d1"),
            ("knight", "d1"),
            ("pawn", "d3"),
            ("bishop", "d1"),
        ] {
            let mut b = game(&[
                ("bishop", "a1", Side::Player),
                (kind, square, Side::Player),
                ("bishop", "b1", Side::Opponent),
            ]);
            b.detect_capture_draw();
            assert_eq!(b.outcome, Outcome::Playing, "{kind}");
        }
        let mut b = game(&[
            ("bishop", "a1", Side::Player),
            ("bishop", "c3", Side::Opponent),
        ]);
        b.play(mv("a1", "c3")).unwrap();
        assert_eq!(b.outcome, Outcome::Won);
        let mut b = game(&[("rook", "a1", Side::Player), ("rook", "a4", Side::Opponent)]);
        b.detect_capture_draw();
        b.play(mv("a1", "a4")).unwrap();
        assert_eq!(b.outcome, Outcome::Won);
    }
    #[test]
    fn losing_the_last_color_changing_piece_detects_dead_material_immediately() {
        let mut b = game(&[
            ("bishop", "a1", Side::Player),
            ("rook", "d1", Side::Player),
            ("bishop", "b1", Side::Opponent),
            ("rook", "d4", Side::Opponent),
        ]);
        b.turn = Side::Opponent;
        b.apply(mv("d4", "d1"), Side::Opponent).unwrap();
        assert_eq!(b.outcome, Outcome::Stuck);
        assert_eq!(b.draw_reason, Some("A bishop is out of reach."));
    }
    #[test]
    fn blocked_pawns_are_dead_but_a_free_pawn_can_still_promote() {
        let mut b = game(&[("pawn", "a2", Side::Player), ("pawn", "a3", Side::Opponent)]);
        b.detect_capture_draw();
        assert_eq!(b.outcome, Outcome::Stuck);
        assert_eq!(b.draw_reason, Some("Neither team can move."));
        let mut b = game(&[("pawn", "a2", Side::Player), ("pawn", "c3", Side::Opponent)]);
        b.detect_capture_draw();
        assert_eq!(b.outcome, Outcome::Playing);
    }
    #[test]
    fn repetition_compares_positions_without_tracking_identical_piece_identity() {
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("rook", "b1", Side::Player),
            ("rook", "d4", Side::Opponent),
        ]);
        let key = b.repetition_key();
        b.units.swap(0, 1);
        assert_eq!(b.repetition_key(), key);
        b.repetitions.insert(key, 3);
        b.detect_capture_draw();
        assert_eq!(b.draw_reason, Some("Threefold repetition"));
    }
    #[test]
    fn repetition_draw_counts_initial_position_and_gives_brave_play_a_chance() {
        let mut b = game(&[("rook", "a1", Side::Player), ("rook", "d4", Side::Opponent)]);
        b.repetitions.clear();
        b.repetitions.insert(b.repetition_key(), 1);
        for round in 0..2 {
            for (from, to, side) in [
                ("a1", "a2", Side::Player),
                ("d4", "d3", Side::Opponent),
                ("a2", "a1", Side::Player),
                ("d3", "d4", Side::Opponent),
            ] {
                b.apply(mv(from, to), side).unwrap();
            }
            assert_eq!(
                b.outcome,
                if round == 0 {
                    Outcome::Playing
                } else {
                    Outcome::Stuck
                }
            );
            assert!(b.brave);
        }
        assert_eq!(b.draw_reason, Some("Threefold repetition"));
    }
    #[test]
    fn quiet_limit_resets_on_capture_and_pawn_moves_and_never_overrides_a_win() {
        for (player, from, to, enemy, enemy_square, expected) in [
            ("rook", "a1", "a2", "rook", "d4", Outcome::Stuck),
            ("pawn", "a2", "a3", "rook", "d4", Outcome::Playing),
            ("rook", "a1", "a4", "rook", "a4", Outcome::Won),
        ] {
            let mut b = game(&[
                (player, from, Side::Player),
                (enemy, enemy_square, Side::Opponent),
            ]);
            b.quiet_plies = 99;
            b.play(mv(from, to)).unwrap();
            assert_eq!(b.outcome, expected);
            assert_eq!(
                b.quiet_plies,
                if expected == Outcome::Stuck { 100 } else { 0 }
            );
        }
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("bishop", "a3", Side::Opponent),
            ("rook", "d4", Side::Opponent),
        ]);
        b.quiet_plies = 99;
        b.play(mv("a1", "a3")).unwrap();
        assert_eq!(b.outcome, Outcome::Playing);
        assert_eq!(b.quiet_plies, 0);
    }
    fn sq(s: &str) -> Square {
        Square::parse(s).unwrap()
    }
    fn mv(a: &str, b: &str) -> Move {
        Move {
            from: sq(a),
            to: sq(b),
        }
    }
    fn game(units: &[(&str, &str, Side)]) -> Battle {
        let mut b = Battle::new(crate::campaign::levels()[10].clone());
        b.units = units
            .iter()
            .map(|(kind, s, side)| Unit {
                kind: (*kind).into(),
                square: sq(s),
                side: *side,
            })
            .collect();
        b
    }
    fn mate_game(units: &[(&str, &str, Side)]) -> Battle {
        let mut level = Level::embedded();
        level.rules.mode = "checkmate".into();
        level.board.width = 4;
        level.board.height = 4;
        let mut b = Battle::new(level);
        b.units = units
            .iter()
            .map(|(kind, square, side)| Unit {
                kind: (*kind).into(),
                square: sq(square),
                side: *side,
            })
            .collect();
        b
    }
    #[test]
    fn promotion_can_give_checkmate_and_a_defender_can_block_check() {
        let mut b = mate_game(&[
            ("pawn", "b3", Side::Player),
            ("rook", "c3", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        assert!(!b.in_check());
        let action = b.play(mv("b3", "b4")).unwrap();
        assert!(action.promoted);
        assert_eq!(b.outcome, Outcome::Won);
        let b = mate_game(&[
            ("rook", "a4", Side::Player),
            ("bishop", "c3", Side::Opponent),
            ("king", "d4", Side::Opponent),
        ]);
        assert!(b.in_check());
        assert!(b.legal_move(mv("c3", "b4"), Side::Opponent));
        assert!(!b.legal_move(mv("c3", "b2"), Side::Opponent));
    }
    #[test]
    fn king_is_never_a_capture_target_and_mate_leaves_it_on_board() {
        let mut b = mate_game(&[
            ("rook", "a1", Side::Player),
            ("rook", "b3", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        b.play(mv("a1", "a4")).unwrap();
        assert!(b.in_check());
        assert_eq!(b.outcome, Outcome::Won);
        assert_eq!(b.count(Side::Opponent), 1);
        assert!(!b.legal_move(mv("a4", "d4"), Side::Player));
        assert!(b.opponent_reply().is_none());
    }
    #[test]
    fn stalemate_is_a_retry_not_a_win_and_normal_check_allows_a_reply() {
        let mut b = mate_game(&[
            ("queen", "b1", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        b.play(mv("b1", "c2")).unwrap();
        assert!(!b.in_check());
        assert_eq!(b.outcome, Outcome::Stuck);
        let mut b = mate_game(&[
            ("rook", "a1", Side::Player),
            ("rook", "b2", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        b.play(mv("a1", "a4")).unwrap();
        assert!(b.in_check());
        assert_eq!(b.outcome, Outcome::Playing);
        b.opponent_reply().unwrap();
        assert!(!b.in_check());
    }
    #[test]
    fn king_captures_only_undefended_pieces_including_the_checking_piece() {
        let b = mate_game(&[
            ("rook", "c3", Side::Player),
            ("rook", "c1", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        assert!(!b.legal_move(mv("d4", "c3"), Side::Opponent));
        let b = mate_game(&[
            ("rook", "d3", Side::Player),
            ("pawn", "a1", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        assert!(b.in_check());
        assert!(b.legal_move(mv("d4", "d3"), Side::Opponent));
        assert!(!b.legal_move(mv("d4", "b4"), Side::Opponent));
    }
    #[test]
    fn pawn_attack_squares_and_blockers_are_respected_by_the_king() {
        let b = mate_game(&[
            ("pawn", "b2", Side::Player),
            ("rook", "a1", Side::Player),
            ("king", "d4", Side::Opponent),
        ]);
        assert!(!b.legal_move(mv("d4", "c3"), Side::Opponent));
        assert!(attacked(&b.level.board, &b.units, sq("c3"), Side::Player));
        assert!(!attacked(&b.level.board, &b.units, sq("b3"), Side::Player));
        let b = mate_game(&[
            ("rook", "d1", Side::Player),
            ("rook", "d3", Side::Opponent),
            ("king", "d4", Side::Opponent),
        ]);
        assert!(!b.in_check());
        assert!(!b.legal_move(mv("d3", "c3"), Side::Opponent));
        assert!(b.legal_move(mv("d3", "d2"), Side::Opponent));
        assert!(b.legal_move(mv("d3", "d1"), Side::Opponent));
    }
    #[test]
    fn every_check_response_is_legal_even_when_brave_or_overlooking_captures() {
        let mut b = mate_game(&[
            ("rook", "a4", Side::Player),
            ("rook", "b2", Side::Player),
            ("bishop", "c2", Side::Opponent),
            ("king", "d4", Side::Opponent),
        ]);
        b.turn = Side::Opponent;
        b.brave = true;
        b.level.opponent.overlook_every = 2;
        b.opponent_turns = 1;
        assert!(b.in_check());
        for m in b.legal_moves(Side::Opponent) {
            let (after, _) = b.after_move(m, Side::Opponent);
            assert!(!king_in_check(&b.level.board, &after));
        }
        b.opponent_reply().unwrap();
        assert!(!b.in_check());
    }
    #[test]
    fn actual_quiet_play_offers_a_last_rook_capture_within_a_few_rounds() {
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("rook", "d1", Side::Player),
            ("rook", "c4", Side::Opponent),
        ]);
        b.level.opponent.safety = 3;
        let mut saw_offer = false;
        for _ in 0..8 {
            if b.outcome == Outcome::Won {
                break;
            }
            let capture = b.legal_moves(Side::Player).into_iter().find(|m| {
                b.units
                    .iter()
                    .any(|u| u.side == Side::Opponent && u.square == m.to)
            });
            if let Some(m) = capture {
                saw_offer = true;
                b.play(m).unwrap();
                break;
            }
            let from = b
                .units
                .iter()
                .find(|u| u.side == Side::Player && u.square.x == 0)
                .unwrap()
                .square;
            b.play(Move {
                from,
                to: Square {
                    x: 0,
                    y: if from.y == 0 { 1 } else { 0 },
                },
            })
            .unwrap();
            b.opponent_reply();
        }
        assert!(
            saw_offer && b.outcome == Outcome::Won,
            "No endless rook chase"
        );
        assert!(b.opponent_turns <= 4);
    }
    #[test]
    fn two_rooks_can_capture_the_last_rook_after_it_becomes_brave() {
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("rook", "d1", Side::Player),
            ("rook", "c4", Side::Opponent),
        ]);
        b.turn = Side::Opponent;
        b.level.opponent.safety = 3;
        b.quiet_rounds = b.brave_after();
        let m = b.choose_opponent().unwrap();
        assert!(b.legal_moves(Side::Opponent).contains(&m));
        assert!(b.offered_capture(m));
        b.opponent_reply().unwrap();
        let capture = b
            .legal_moves(Side::Player)
            .into_iter()
            .find(|m| {
                b.units
                    .iter()
                    .any(|u| u.side == Side::Opponent && u.square == m.to)
            })
            .unwrap();
        b.play(capture).unwrap();
        assert_eq!(b.outcome, Outcome::Won);
    }
    #[test]
    fn bravery_stays_until_a_capture_then_quiet_rounds_reset() {
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("rook", "d1", Side::Player),
            ("rook", "c4", Side::Opponent),
        ]);
        b.turn = Side::Opponent;
        b.brave = true;
        b.apply(mv("c4", "c3"), Side::Opponent).unwrap();
        assert!(b.brave);
        b.apply(mv("a1", "a2"), Side::Player).unwrap();
        assert!(b.brave);
        b.apply(mv("c3", "d3"), Side::Opponent).unwrap();
        assert!(b.brave);
        b.play(mv("d1", "d3")).unwrap();
        assert_eq!(b.quiet_rounds, 0);
        assert!(!b.brave);
    }
    #[test]
    fn quiet_round_threshold_and_last_piece_generosity_are_explicit() {
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("rook", "d4", Side::Opponent),
            ("pawn", "c4", Side::Opponent),
        ]);
        b.level.opponent.brave_after = 4;
        assert_eq!(b.brave_after(), 4);
        b.units.pop();
        assert_eq!(b.brave_after(), 2);
    }
    #[test]
    fn turns_reject_double_moves_and_wrong_side_without_mutation() {
        let mut b = game(&[("rook", "a1", Side::Player), ("rook", "d4", Side::Opponent)]);
        assert!(b.opponent_reply().is_none());
        assert!(b.play(mv("d4", "d3")).is_err());
        b.play(mv("a1", "a2")).unwrap();
        let before = b.units.clone();
        assert!(b.play(mv("a2", "b2")).is_err());
        assert_eq!(b.units, before);
        assert!(b.opponent_reply().is_some());
        assert_eq!(b.turn, Side::Player);
        assert!(b.opponent_reply().is_none());
    }
    #[test]
    fn captures_remove_only_the_enemy_and_finish_the_level() {
        let mut b = game(&[
            ("rook", "a1", Side::Player),
            ("bishop", "b1", Side::Player),
            ("pawn", "a3", Side::Opponent),
        ]);
        assert!(!b.legal_moves(Side::Player).contains(&mv("a1", "b1")));
        assert!(!b.legal_moves(Side::Player).contains(&mv("a1", "c1")));
        assert!(b.play(mv("a1", "a3")).unwrap().captured);
        assert_eq!(b.outcome, Outcome::Won);
        assert_eq!(b.count(Side::Player), 2);
        assert!(b.opponent_reply().is_none());
        assert!(b.play(mv("a3", "a4")).is_err());
    }
    #[test]
    fn opponent_takes_obvious_captures_but_periodically_overlooks_them() {
        let mut b = game(&[
            ("pawn", "a2", Side::Player),
            ("rook", "a4", Side::Opponent),
            ("rook", "d1", Side::Player),
        ]);
        assert_eq!(b.choose_opponent(), Some(mv("a4", "a2")));
        b.opponent_turns = b.level.opponent.overlook_every - 1;
        let m = b.choose_opponent().unwrap();
        assert_ne!(m.to, sq("a2"));
        assert!(b.legal_moves(Side::Opponent).contains(&m));
        assert_eq!(b.choose_opponent(), Some(m));
    }
    #[test]
    fn losing_all_pieces_is_a_local_failure() {
        let mut b = game(&[("pawn", "a2", Side::Player), ("rook", "a4", Side::Opponent)]);
        b.turn = Side::Opponent;
        b.opponent_reply().unwrap();
        assert_eq!(b.outcome, Outcome::Lost);
        assert!(b.hint().is_none());
        assert!(b.play(mv("a2", "a3")).is_err());
    }
    #[test]
    fn pawns_promote_to_queens_on_both_sides_without_changing_team() {
        let mut b = game(&[("pawn", "a3", Side::Player), ("pawn", "d2", Side::Opponent)]);
        let action = b.play(mv("a3", "a4")).unwrap();
        assert!(action.promoted);
        assert_eq!(b.units[0].kind, "queen");
        assert_eq!(b.units[0].side, Side::Player);
        let reply = b.opponent_reply().unwrap();
        assert!(reply.promoted);
        assert_eq!(reply.movement, mv("d2", "d1"));
        assert_eq!(b.units[1].kind, "queen");
        assert_eq!(b.units[1].side, Side::Opponent);
        assert_eq!(b.outcome, Outcome::Playing);
        assert!(b.legal_moves(Side::Player).contains(&mv("a4", "c2")));
    }
    #[test]
    fn capture_promotion_removes_target_and_non_edge_pawn_stays_a_pawn() {
        let mut b = game(&[
            ("pawn", "b3", Side::Player),
            ("rook", "c4", Side::Opponent),
            ("rook", "d1", Side::Opponent),
        ]);
        let action = b.play(mv("b3", "c4")).unwrap();
        assert!(action.promoted && action.captured);
        assert_eq!(b.count(Side::Opponent), 1);
        assert_eq!(b.units[0].kind, "queen");
        let mut b = game(&[("pawn", "b1", Side::Player), ("rook", "d4", Side::Opponent)]);
        assert!(!b.play(mv("b1", "b2")).unwrap().promoted);
        assert_eq!(b.units[0].kind, "pawn");
    }
    #[test]
    fn immobile_opponent_passes_without_a_fake_move() {
        let mut b = game(&[("rook", "a1", Side::Player), ("pawn", "d1", Side::Opponent)]);
        b.play(mv("a1", "a2")).unwrap();
        assert!(b.opponent_reply().is_none());
        assert_eq!(b.turn, Side::Player);
        assert_eq!(b.outcome, Outcome::Playing);
        assert_eq!(b.opponent_turns, 0);
        b.play(mv("a2", "d2")).unwrap();
        b.opponent_reply();
        b.play(mv("d2", "d1")).unwrap();
        assert_eq!(b.outcome, Outcome::Won);
    }
    #[test]
    fn second_position_visit_triggers_bravery_before_the_draw_limit() {
        let mut b = game(&[("rook", "a1", Side::Player), ("rook", "d4", Side::Opponent)]);
        for _ in 0..2 {
            for (side, m) in [
                (Side::Player, mv("a1", "a2")),
                (Side::Opponent, mv("d4", "d3")),
                (Side::Player, mv("a2", "a1")),
                (Side::Opponent, mv("d3", "d4")),
            ] {
                if b.outcome == Outcome::Playing {
                    b.apply(m, side).unwrap();
                }
            }
        }
        assert_eq!(b.outcome, Outcome::Playing);
        assert!(b.brave);
    }
}
