use crate::{
    battle::{Action, Battle, Move, Outcome, Side, Unit},
    rules::{Level, Puzzle, Square},
};
#[derive(Clone, Debug)]
pub struct Session {
    pub level: Level,
    puzzle: Option<Puzzle>,
    pub battle: Option<Battle>,
    pub selected: Option<Square>,
}
impl Session {
    pub fn new(level: Level) -> Result<Self, String> {
        level.validate()?;
        let selected = if level.is_match() {
            None
        } else {
            Some(Square::parse(&level.player.square)?)
        };
        let (puzzle, battle) = if level.is_match() {
            (None, Some(Battle::new(level.clone())))
        } else {
            (Some(Puzzle::new(level.clone())?), None)
        };
        Ok(Self {
            level,
            puzzle,
            battle,
            selected,
        })
    }
    pub fn restart(&mut self) {
        *self = Self::new(self.level.clone()).unwrap();
    }
    pub fn units(&self) -> Vec<Unit> {
        if let Some(b) = &self.battle {
            return b.units.clone();
        }
        let p = self.puzzle.as_ref().unwrap();
        let mut units = vec![Unit {
            kind: p.position.kind.clone(),
            square: p.position.player,
            side: Side::Player,
        }];
        units.extend(p.position.targets.iter().map(|s| Unit {
            kind: "pawn".into(),
            square: *s,
            side: Side::Opponent,
        }));
        units
    }
    pub fn select(&mut self, s: Square) -> bool {
        if self.player_turn()
            && self
                .units()
                .iter()
                .any(|u| u.square == s && u.side == Side::Player)
        {
            self.selected = Some(s);
            true
        } else {
            false
        }
    }
    pub fn player_turn(&self) -> bool {
        self.outcome() == Outcome::Playing
            && self.battle.as_ref().is_none_or(|b| b.turn == Side::Player)
    }
    pub fn outcome(&self) -> Outcome {
        if let Some(b) = &self.battle {
            return b.outcome;
        }
        let p = self.puzzle.as_ref().unwrap();
        if p.won() {
            Outcome::Won
        } else if p.hint().is_none() {
            Outcome::Stuck
        } else {
            Outcome::Playing
        }
    }
    pub fn won(&self) -> bool {
        self.outcome() == Outcome::Won
    }
    pub fn failed(&self) -> bool {
        matches!(self.outcome(), Outcome::Lost | Outcome::Stuck)
    }
    pub fn remaining(&self) -> usize {
        self.units()
            .iter()
            .filter(|u| u.side == Side::Opponent)
            .count()
    }
    pub fn legal(&self, to: Square) -> bool {
        if !self.player_turn() {
            return false;
        }
        let Some(from) = self.selected else {
            return false;
        };
        if let Some(b) = &self.battle {
            return b.legal_move(Move { from, to }, Side::Player);
        }
        self.puzzle.as_ref().unwrap().legal(to)
    }
    pub fn play(&mut self, to: Square) -> Result<Action, &'static str> {
        if !self.player_turn() {
            return Err("Wait for your turn");
        }
        let from = self.selected.ok_or("Choose one of your pieces")?;
        let action = if let Some(b) = &mut self.battle {
            b.play(Move { from, to })?
        } else {
            let kind = self.puzzle.as_ref().unwrap().position.kind.clone();
            let result = self.puzzle.as_mut().unwrap().move_to(to)?;
            Action {
                movement: Move { from, to },
                kind,
                side: Side::Player,
                captured: result.captured,
                promoted: result.promoted,
            }
        };
        self.selected = if self.level.is_match() {
            None
        } else {
            Some(to)
        };
        Ok(action)
    }
    pub fn reply(&mut self) -> Option<Action> {
        let action = self.battle.as_mut()?.opponent_reply();
        self.selected = None;
        action
    }
    pub fn promotion_choices(&self, to: Square) -> Vec<&'static str> {
        self.selected
            .and_then(|from| {
                self.battle
                    .as_ref()
                    .map(|b| b.promotion_choices(Move { from, to }))
            })
            .unwrap_or_default()
    }
    pub fn play_promoted(&mut self, to: Square, promotion: &str) -> Result<Action, &'static str> {
        if !self.player_turn() {
            return Err("Wait for your turn");
        }
        let from = self.selected.ok_or("Choose one of your pieces")?;
        let action = self
            .battle
            .as_mut()
            .ok_or("Not a chess match")?
            .play_promoted(Move { from, to }, promotion)?;
        self.selected = None;
        Ok(action)
    }
    pub fn hint(&self) -> Option<Move> {
        if !self.player_turn() {
            return None;
        }
        if let Some(b) = &self.battle {
            return b.hint();
        }
        let p = self.puzzle.as_ref().unwrap();
        p.hint().map(|to| Move {
            from: p.position.player,
            to,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sq(s: &str) -> Square {
        Square::parse(s).unwrap()
    }
    #[test]
    fn pawn_dead_end_allows_unlimited_restart_without_losing_stars() {
        let levels = crate::campaign::levels();
        let mut save = crate::save::Save::default();
        for l in &levels[..6] {
            save.complete(&l.id);
        }
        let earned = save.clone();
        let mut g = Session::new(levels[6].clone()).unwrap();
        let initial = g.units();
        for _ in 0..10 {
            g.play(sq("b2")).unwrap();
            assert!(!g.failed());
            g.play(sq("b3")).unwrap();
            assert!(g.failed());
            assert!(g.play(sq("b3")).is_err());
            g.restart();
            assert_eq!(g.units(), initial);
            assert_eq!(g.outcome(), Outcome::Playing);
            assert_eq!(save, earned);
            assert!(crate::campaign::unlocked(&levels, &save, 6));
        }
    }
    #[test]
    fn first_pawn_lesson_can_promote_before_returning_for_the_last_target() {
        let level = crate::campaign::levels()[6].clone();
        assert!(level.pawn_promotion);
        let mut g = Session::new(level).unwrap();
        for to in ["b2", "c3", "c4"] {
            let action = g.play(sq(to)).unwrap();
            assert!(!action.promoted);
            assert_eq!(g.outcome(), Outcome::Playing, "detour at {to}");
        }
        assert_eq!(g.remaining(), 1);
        assert_eq!(g.hint().unwrap().to, sq("c5"));
        let action = g.play(sq("c5")).unwrap();
        assert!(action.promoted);
        assert_eq!(g.units()[0].kind, "queen");
        assert_eq!(g.outcome(), Outcome::Playing);
        assert_eq!(g.remaining(), 1);
        g.play(sq("b4")).unwrap();
        assert!(g.won());
        g.restart();
        assert_eq!(g.units()[0].kind, "pawn");
        assert_eq!(g.selected, Some(sq("b1")));
        assert_eq!(g.remaining(), 2);
        assert!(!g.failed());
    }
    #[test]
    fn selecting_allies_hints_and_restart_respect_turns() {
        let l = crate::campaign::levels()[10].clone();
        let mut g = Session::new(l).unwrap();
        let initial = g.units();
        assert_eq!(g.selected, None);
        assert!(!g.select(sq("a3")));
        assert!(g.select(sq("d1")));
        let m = g.hint().unwrap();
        assert!(g.select(m.from));
        assert!(g.legal(m.to));
        g.play(m.to).unwrap();
        assert!(!g.player_turn());
        assert!(g.hint().is_none());
        assert!(!g.select(sq("a1")));
        g.reply();
        assert!(g.player_turn());
        assert_eq!(g.selected, None);
        assert!(g.play(sq("b1")).is_err());
        g.restart();
        assert_eq!(g.units(), initial);
        assert_eq!(g.selected, None);
        assert_eq!(g.battle.as_ref().unwrap().opponent_turns, 0);
    }
    #[test]
    fn restarting_after_a_loss_restores_every_piece_and_player_turn() {
        let mut g = Session::new(crate::campaign::levels()[11].clone()).unwrap();
        let initial = g.units();
        let b = g.battle.as_mut().unwrap();
        b.units.retain(|u| u.side == Side::Opponent);
        b.outcome = Outcome::Lost;
        assert!(g.failed());
        g.restart();
        assert!(!g.failed());
        assert!(g.player_turn());
        assert_eq!(g.units(), initial);
    }
}
