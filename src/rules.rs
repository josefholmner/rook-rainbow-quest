//! Pure puzzle rules. No window, clock, sound, or filesystem is required.
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn parse(s: &str) -> Result<Self, String> {
        let b = s.as_bytes();
        if b.len() != 2 || !(b'a'..=b'h').contains(&b[0]) || !(b'1'..=b'8').contains(&b[1]) {
            return Err(format!("Invalid square: {s}"));
        }
        Ok(Self {
            x: b[0] - b'a',
            y: b[1] - b'1',
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub schema_version: u32,
    pub id: String,
    pub title: String,
    pub world: String,
    pub board: Board,
    pub coordinates: String,
    pub player: Piece,
    pub targets: Vec<Piece>,
    pub rules: Rules,
    pub goal: String,
    pub intro: String,
    pub movement_reminder: String,
    pub initially_selected: bool,
    pub show_legal_moves: bool,
    pub tutorial_path: Vec<String>,
    pub example_solution: Vec<String>,
    pub completion_stars: u8,
    pub next_level_id: Option<String>,
    #[serde(default)]
    pub allies: Vec<Piece>,
    #[serde(default)]
    pub opponent: OpponentConfig,
    #[serde(default)]
    pub example_turns: Vec<Step>,
    pub icon: LevelIcon,
    #[serde(default)]
    pub minimum_quiet_moves: u8,
    #[serde(default)]
    pub pawn_promotion: bool,
    #[serde(default)]
    pub castling_rights: String,
    #[serde(default)]
    pub en_passant: Option<String>,
    #[serde(default)]
    pub opening_only: bool,
    #[serde(default)]
    pub final_boss: bool,
}
#[derive(Clone, Debug, Deserialize)]
pub struct LevelIcon {
    pub piece: String,
    pub opponent: Option<String>,
    pub badge: u8,
    #[serde(default)]
    pub castle: bool,
}
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpponentConfig {
    pub overlook_every: u32,
    pub safety: u8,
    #[serde(default = "default_brave_after")]
    pub brave_after: u32,
}
fn default_brave_after() -> u32 {
    3
}
#[derive(Clone, Debug, Deserialize)]
pub struct Step {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Board {
    pub width: u8,
    pub height: u8,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Piece {
    pub piece: String,
    pub square: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rules {
    pub mode: String,
    pub targets_move: bool,
    pub targets_attack: bool,
    pub allow_non_capture_moves: bool,
    pub allow_jumping: bool,
    pub move_limit: Option<u32>,
    pub time_limit_seconds: Option<u32>,
}

impl Level {
    pub fn is_match(&self) -> bool {
        matches!(
            self.rules.mode.as_str(),
            "turn-based-capture" | "checkmate" | "standard-chess"
        )
    }
    pub fn is_checkmate(&self) -> bool {
        self.rules.mode == "checkmate"
    }
    pub fn is_standard(&self) -> bool {
        self.rules.mode == "standard-chess"
    }
    pub fn is_royal(&self) -> bool {
        self.is_checkmate() || self.is_standard()
    }
    pub fn from_json(s: &str) -> Result<Self, String> {
        let level: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        level.validate()?;
        Ok(level)
    }
    pub fn embedded() -> Self {
        Self::from_json(include_str!("../levels/01-rook-rainbow-trail.json"))
            .expect("Bundled level must pass validation and tests")
    }
    pub fn validate(&self) -> Result<(), String> {
        if !crate::movement::KINDS.contains(&self.icon.piece.as_str())
            || !(1..=2).contains(&self.icon.badge)
            || self
                .icon
                .opponent
                .as_ref()
                .is_some_and(|p| !crate::movement::KINDS.contains(&p.as_str()))
            || self.icon.opponent.is_some() != self.is_match()
        {
            return Err("Invalid level icon".into());
        }
        if self.schema_version != 1 || self.id.trim().is_empty() || self.title.trim().is_empty() {
            return Err("Unsupported schema or missing identity".into());
        }
        if !(2..=8).contains(&self.board.width) || !(2..=8).contains(&self.board.height) {
            return Err("Board dimensions must be between 2 and 8".into());
        }
        if self.coordinates != "algebraic-bottom-left"
            || !crate::movement::KINDS.contains(&self.player.piece.as_str())
            || self.goal
                != if self.is_royal() {
                    "checkmate"
                } else {
                    "capture-all-targets"
                }
            || ![
                "stationary-capture",
                "turn-based-capture",
                "checkmate",
                "standard-chess",
            ]
            .contains(&self.rules.mode.as_str())
            || self.rules.targets_move != self.is_match()
            || self.rules.targets_attack != self.is_match()
            || self.rules.allow_jumping != (self.is_match() || self.player.piece == "knight")
            || !self.rules.allow_non_capture_moves
            || self.rules.move_limit.is_some()
            || self.rules.time_limit_seconds.is_some()
            || self.completion_stars != 1
        {
            return Err("Unsupported puzzle rules in this milestone".into());
        }
        if self.targets.is_empty() || self.targets.len() > 16 {
            return Err("Expected 1 to 16 targets".into());
        }
        let mut occupied = HashSet::new();
        for p in std::iter::once(&self.player)
            .chain(self.allies.iter())
            .chain(self.targets.iter())
        {
            let sq = Square::parse(&p.square)?;
            if sq.x >= self.board.width || sq.y >= self.board.height || !occupied.insert(sq) {
                return Err("Pieces overlap or lie outside the board".into());
            }
            if !crate::movement::KINDS.contains(&p.piece.as_str()) {
                return Err("Unsupported piece".into());
            }
        }
        let friendly = std::iter::once(&self.player).chain(self.allies.iter());
        if !self.is_standard()
            && (friendly.clone().any(|p| p.piece == "king")
                || self.targets.iter().filter(|p| p.piece == "king").count()
                    != usize::from(self.is_checkmate()))
        {
            return Err(
                "Only checkmate opponents have exactly one king; players never have kings".into(),
            );
        }
        if self.is_standard() {
            if self.board.width != 8
                || self.board.height != 8
                || friendly.clone().filter(|p| p.piece == "king").count() != 1
                || self.targets.iter().filter(|p| p.piece == "king").count() != 1
                || self.allies.len() > 15
                || !self.pawn_promotion
            {
                return Err("Regular chess needs an 8x8 board and one king per side".into());
            }
            let mut game = crate::standard::Standard::from_level(self)?;
            if game.outcome() != crate::battle::Outcome::Playing {
                return Err("Chess levels must start playable".into());
            }
        } else if !self.castling_rights.is_empty()
            || self.en_passant.is_some()
            || self.opening_only
            || self.final_boss
            || self.icon.castle
        {
            return Err("Special chess metadata belongs to World 3".into());
        }
        if self.is_checkmate() {
            let advantage = friendly
                .map(|p| crate::movement::value(&p.piece))
                .sum::<i32>()
                - self
                    .targets
                    .iter()
                    .map(|p| crate::movement::value(&p.piece))
                    .sum::<i32>();
            if advantage < 5 {
                return Err("Checkmate lessons need a clear material advantage".into());
            }
            let initial = crate::battle::Battle::new(self.clone());
            if initial.in_check()
                || initial
                    .legal_moves(crate::battle::Side::Opponent)
                    .is_empty()
            {
                return Err("Start with an unchecked, mobile king".into());
            }
        }
        if self.is_match() {
            if self.opponent.overlook_every < 2
                || self.opponent.safety > 3
                || !(2..=4).contains(&self.opponent.brave_after)
            {
                return Err("Opponent must have a gentle valid policy".into());
            }
            if self.example_turns.is_empty() {
                return Err("Matches need a tested winning example".into());
            }
            let mut b = crate::battle::Battle::new(self.clone());
            for step in &self.example_turns {
                b.play(crate::battle::Move {
                    from: Square::parse(&step.from)?,
                    to: Square::parse(&step.to)?,
                })
                .map_err(|e| format!("Invalid match witness: {e}"))?;
                b.opponent_reply();
            }
            if b.outcome != crate::battle::Outcome::Won
                && !(self.is_standard()
                    && self.opening_only
                    && b.outcome == crate::battle::Outcome::Playing)
            {
                return Err("Match witness must win against its opponent".into());
            }
            return Ok(());
        }
        if !self.allies.is_empty() {
            return Err("Practice lessons use exactly one friendly piece".into());
        }
        if self.targets.iter().any(|p| p.piece != "pawn") {
            return Err("Only stationary pawn targets are supported".into());
        }
        if self.player.piece == "bishop" {
            let from = Square::parse(&self.player.square)?;
            if self.targets.iter().any(|p| {
                let s = Square::parse(&p.square).unwrap();
                (s.x + s.y) % 2 != (from.x + from.y) % 2
            }) {
                return Err("Bishop targets must stay on the starting color".into());
            }
        }
        if self.example_solution.is_empty() {
            return Err("Each level needs a validated example solution".into());
        }
        for square in &self.tutorial_path {
            let sq = Square::parse(square)?;
            if sq.x >= self.board.width || sq.y >= self.board.height {
                return Err("Tutorial outside board".into());
            }
        }
        if !self.example_solution.is_empty() {
            if self.example_solution[0] != self.player.square {
                return Err("Solution must start at the rook".into());
            }
            let mut game = Puzzle::new_unchecked(self.clone());
            for s in self.example_solution.iter().skip(1) {
                game.move_to(Square::parse(s)?)
                    .map_err(|_| "Illegal example solution")?;
            }
            if !game.won() {
                return Err("Example solution does not finish the puzzle".into());
            }
        }
        if self.minimum_quiet_moves > 0 {
            let route = Puzzle::new_unchecked(self.clone())
                .shortest_solution()
                .ok_or("Unsolvable lesson")?;
            if route.len() < self.targets.len() + self.minimum_quiet_moves as usize {
                return Err("Lesson can be solved with too few quiet moves".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub kind: String,
    pub player: Square,
    pub targets: Vec<Square>,
}
#[derive(Clone, Debug)]
pub struct Puzzle {
    pub level: Level,
    pub position: Position,
    solution: Option<Vec<Square>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveOutcome {
    pub captured: bool,
    pub completed: bool,
    pub promoted: bool,
}

impl Puzzle {
    pub fn new(level: Level) -> Result<Self, String> {
        level.validate()?;
        Ok(Self::new_unchecked(level))
    }
    fn new_unchecked(level: Level) -> Self {
        let position = Position {
            kind: level.player.piece.clone(),
            player: Square::parse(&level.player.square).unwrap(),
            targets: level
                .targets
                .iter()
                .map(|p| Square::parse(&p.square).unwrap())
                .collect(),
        };
        let mut game = Self {
            level,
            position,
            solution: None,
        };
        game.solution = game.shortest_solution();
        game
    }
    pub fn won(&self) -> bool {
        self.position.targets.is_empty()
    }
    pub fn legal(&self, to: Square) -> bool {
        !self.won() && legal(&self.position, to, &self.level.board, &self.position.kind)
    }
    pub fn move_to(&mut self, to: Square) -> Result<MoveOutcome, &'static str> {
        if !self.legal(to) {
            return Err("Try a glowing square");
        }
        let before = self.position.targets.len();
        self.position.targets.retain(|s| *s != to);
        self.position.player = to;
        let promoted = self.level.pawn_promotion
            && self.position.kind == "pawn"
            && to.y + 1 == self.level.board.height;
        if promoted {
            self.position.kind = "queen".into();
        }
        self.solution = self.shortest_solution();
        Ok(MoveOutcome {
            captured: self.position.targets.len() < before,
            completed: self.won(),
            promoted,
        })
    }
    pub fn restart(&mut self) {
        *self = Self::new_unchecked(self.level.clone());
    }
    /// Exact shortest winning route, including every remaining target.
    /// Used for authoring/validation; quiet moves must be necessary on every route.
    pub fn shortest_solution(&self) -> Option<Vec<Square>> {
        let mut queue = VecDeque::from([(self.position.clone(), Vec::new())]);
        let mut seen = HashSet::new();
        while let Some((p, path)) = queue.pop_front() {
            if p.targets.is_empty() {
                return Some(path);
            }
            if !seen.insert((p.player, p.targets.clone(), p.kind.clone())) {
                continue;
            }
            for y in 0..self.level.board.height {
                for x in 0..self.level.board.width {
                    let to = Square { x, y };
                    if legal(&p, to, &self.level.board, &p.kind) {
                        let mut next = p.clone();
                        next.player = to;
                        if self.level.pawn_promotion
                            && next.kind == "pawn"
                            && to.y + 1 == self.level.board.height
                        {
                            next.kind = "queen".into();
                        }
                        next.targets.retain(|q| *q != to);
                        let mut route = path.clone();
                        route.push(to);
                        queue.push_back((next, route));
                    }
                }
            }
        }
        None
    }
    /// The cached exact winning route is refreshed after each move.
    /// This includes quiet moves needed to promote before returning for a target.
    pub fn hint(&self) -> Option<Square> {
        self.solution.as_ref()?.first().copied()
    }
}

fn legal(position: &Position, to: Square, board: &Board, piece: &str) -> bool {
    crate::movement::legal(piece, position.player, to, board, 1, |s| {
        position.targets.contains(&s).then_some(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sq(s: &str) -> Square {
        Square::parse(s).unwrap()
    }
    fn fixture(kind: &str, targets: &[&str], route: &[&str]) -> Level {
        let mut level = Level::embedded();
        level.player.piece = kind.into();
        level.player.square = "a1".into();
        level.targets = targets
            .iter()
            .map(|square| Piece {
                piece: "pawn".into(),
                square: (*square).into(),
            })
            .collect();
        level.example_solution = route.iter().map(|s| (*s).into()).collect();
        level.tutorial_path.clear();
        level.minimum_quiet_moves = 0;
        level.rules.allow_jumping = kind == "knight";
        level.icon.piece = kind.into();
        level
    }
    fn game() -> Puzzle {
        Puzzle::new(fixture(
            "rook",
            &["b1", "d1", "d4"],
            &["a1", "b1", "d1", "d4"],
        ))
        .unwrap()
    }
    #[test]
    fn second_pawn_lesson_requires_promotion_to_capture_the_unreachable_target() {
        let level = crate::campaign::levels()[7].clone();
        assert!(level.pawn_promotion);
        let mut without = Puzzle::new(level.clone()).unwrap();
        without.level.pawn_promotion = false;
        assert!(
            without.shortest_solution().is_none(),
            "Pawn alone cannot reach a1 behind its starting rank"
        );
        let mut g = Puzzle::new(level.clone()).unwrap();
        let mut promoted = false;
        for to in &level.example_solution[1..] {
            let action = g.move_to(sq(to)).unwrap();
            promoted |= action.promoted;
            if !promoted {
                assert!(g.position.targets.contains(&sq("a1")));
            }
        }
        assert!(promoted && g.won());
        assert_eq!(g.position.kind, "queen");
        g.restart();
        assert_eq!(g.position.kind, "pawn");
    }
    #[test]
    fn bishop_slides_diagonally_and_cannot_jump_or_switch_colors() {
        let l = fixture("bishop", &["b2", "d4"], &["a1", "b2", "d4"]);
        let mut g = Puzzle::new(l).unwrap();
        assert!(g.legal(sq("b2")));
        assert!(!g.legal(sq("d4")));
        assert!(!g.legal(sq("a2")));
        g.move_to(sq("b2")).unwrap();
        assert!(g.legal(sq("d4")));
    }
    #[test]
    fn knight_moves_only_in_l_shapes_and_jumps_over_pawns() {
        let mut l = fixture("knight", &["b3", "d4"], &["a1", "b3", "d4"]);
        let mut g = Puzzle::new(l.clone()).unwrap();
        assert!(g.legal(sq("b3")));
        assert!(g.legal(sq("c2")));
        assert!(!g.legal(sq("b2")));
        assert!(!g.legal(sq("a3")));
        // Occupied neighboring squares do not block a knight's hop.
        l.targets.push(Piece {
            piece: "pawn".into(),
            square: "a2".into(),
        });
        g.position.targets.push(sq("a2"));
        assert!(g.legal(sq("b3")));
        assert!(g.move_to(sq("b3")).unwrap().captured);
    }
    #[test]
    fn captures_and_completes_exactly_once() {
        let mut g = game();
        for (i, s) in ["b1", "d1", "d4"].iter().enumerate() {
            let result = g.move_to(sq(s)).unwrap();
            assert!(result.captured);
            assert_eq!(result.completed, i == 2);
        }
        assert!(g.won());
        assert!(g.move_to(sq("a4")).is_err());
        assert_eq!(g.hint(), None);
    }
    #[test]
    fn rejects_blocking_diagonal_stationary_and_offboard_moves_without_mutation() {
        let mut g = game();
        let before = g.position.clone();
        for to in [sq("d1"), sq("b2"), sq("a1"), Square { x: 4, y: 0 }] {
            assert!(g.move_to(to).is_err());
            assert_eq!(g.position, before);
        }
    }
    #[test]
    fn empty_moves_and_reverse_paths_obey_blocking() {
        let mut g = game();
        assert!(!g.move_to(sq("a4")).unwrap().captured);
        assert!(g.move_to(sq("d4")).unwrap().captured);
        assert!(g.move_to(sq("d1")).unwrap().captured);
        assert!(!g.legal(sq("a1")));
        assert!(g.legal(sq("b1")));
    }
    #[test]
    fn restart_restores_all_targets_without_a_lives_counter() {
        let mut g = game();
        let initial = g.position.clone();
        g.move_to(sq("b1")).unwrap();
        g.restart();
        assert_eq!(g.position, initial);
    }
    #[test]
    fn hint_solves_from_every_reachable_state_including_detours() {
        let start = game();
        let mut queue = VecDeque::from([start]);
        let mut seen = HashSet::new();
        while let Some(g) = queue.pop_front() {
            let key = (g.position.player, g.position.targets.clone());
            if !seen.insert(key) {
                continue;
            }
            let mut guided = g.clone();
            for _ in 0..32 {
                if guided.won() {
                    break;
                }
                let next = guided
                    .hint()
                    .expect("Every unfinished reachable state has a hint");
                assert!(guided.legal(next));
                guided.move_to(next).unwrap();
            }
            assert!(guided.won(), "Hints must make progress rather than cycle");
            for y in 0..4 {
                for x in 0..4 {
                    let to = Square { x, y };
                    if g.legal(to) {
                        let mut n = g.clone();
                        n.move_to(to).unwrap();
                        queue.push_back(n);
                    }
                }
            }
        }
        assert!(seen.len() > 50);
    }
    #[test]
    fn rejects_invalid_level_data() {
        let mut l = Level::embedded();
        l.targets[0].square = "a1".into();
        assert!(l.validate().is_err());
        let mut l = Level::embedded();
        l.targets[0].square = "e1".into();
        assert!(l.validate().is_err());
        let mut l = Level::embedded();
        l.rules.targets_attack = true;
        assert!(l.validate().is_err());
        let mut l = Level::embedded();
        l.targets[0].piece = "king".into();
        assert!(l.validate().is_err());
        let mut l = Level::embedded();
        l.example_solution = vec!["a1".into(), "d1".into()];
        assert!(l.validate().is_err());
        assert!(Level::from_json("{}").is_err());
        assert!(Square::parse("ÃƒÆ’Ã‚Â©1").is_err());
        assert!(Square::parse("a0").is_err());
    }
}
