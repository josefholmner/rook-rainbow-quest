use crate::{rules::Level, save::Save};
include!(concat!(env!("OUT_DIR"), "/campaign.rs"));

pub fn levels() -> Vec<Level> {
    let levels: Vec<_> = EMBEDDED_LEVELS
        .iter()
        .map(|s| Level::from_json(s).expect("Valid campaign level"))
        .collect();
    validate(&levels).expect("Valid campaign links");
    levels
}
pub fn validate(levels: &[Level]) -> Result<(), String> {
    if levels.is_empty() {
        return Err("Campaign cannot be empty".into());
    }
    let mut ids = std::collections::HashSet::new();
    let worlds: Vec<crate::worlds::World> =
        serde_json::from_str(include_str!("../worlds/worlds.json")).map_err(|e| e.to_string())?;
    for (i, level) in levels.iter().enumerate() {
        level.validate()?;
        if !ids.insert(&level.id) {
            return Err("Duplicate level ID".into());
        }
        let expected = levels.get(i + 1).map(|l| l.id.as_str());
        let world_end = worlds.iter().any(|w| w.last_level_id == level.id);
        if level.next_level_id.as_deref() != expected
            && !(world_end && level.next_level_id.is_none())
        {
            return Err("Successor must match campaign order".into());
        }
    }
    Ok(())
}
pub fn unlocked(levels: &[Level], save: &Save, index: usize) -> bool {
    index < levels.len()
        && (save.all_levels_unlocked[save.active_profile]
            || index == 0
            || levels[..index].iter().all(|l| save.completed(&l.id)))
}
pub fn restore_cursor(levels: &[Level], save: &Save) -> usize {
    let id = &save.map_level[save.active_profile];
    levels
        .iter()
        .position(|l| &l.id == id)
        .filter(|i| unlocked(levels, save, *i))
        .unwrap_or(0)
}
/// An arbitrarily long trail; the viewport follows the avatar rather than fitting
/// every stop on one fixed screen. There is no 20-level or world-count ceiling.
pub fn node(index: usize) -> (f32, f32) {
    (
        220. + index as f32 * 225.,
        470. + [0., -65., 25., -75., 0.][index % 5],
    )
}
pub fn camera_x(avatar_x: f32, count: usize) -> f32 {
    (avatar_x - 470.).clamp(0., (node(count.saturating_sub(1)).0 + 220. - 1280.).max(0.))
}
#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
pub fn neighbor(index: usize, direction: Direction, count: usize) -> Option<usize> {
    match direction {
        Direction::Left => index.checked_sub(1),
        Direction::Right => (index + 1 < count).then_some(index + 1),
        Direction::Up | Direction::Down => [
            index.checked_add(1).filter(|i| *i < count),
            index.checked_sub(1),
        ]
        .into_iter()
        .flatten()
        .find(|i| {
            let delta = node(*i).1 - node(index).1;
            match direction {
                Direction::Up => delta < 0.,
                _ => delta > 0.,
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{Puzzle, Square};
    #[test]
    fn level_fourteen_has_a_mobile_rook_and_a_safe_opening_reply() {
        use crate::battle::{Battle, Move, Side};
        let l = levels()[13].clone();
        assert_eq!(l.id, "match-04");
        let mut b = Battle::new(l.clone());
        let rook = Square::parse(&l.player.square).unwrap();
        assert!(b.legal_moves(Side::Player).iter().any(|m| m.from == rook));
        assert!(!b.legal_moves(Side::Opponent).iter().any(|m| m.to == rook));
        b.play(Move {
            from: Square::parse("a2").unwrap(),
            to: Square::parse("a3").unwrap(),
        })
        .unwrap();
        assert!(!b.legal_moves(Side::Opponent).iter().any(|m| m.to == rook));
        b.opponent_reply();
        assert!(b
            .units
            .iter()
            .any(|u| u.side == Side::Player && u.kind == "rook" && u.square == rook));
    }
    #[test]
    fn checkmate_world_has_twenty_distinct_boards_and_gentle_material_advantages() {
        let levels = levels();
        let mut boards = std::collections::HashSet::new();
        let mut last_count = 1;
        for (i, l) in levels[20..40].iter().enumerate() {
            assert!(l.is_checkmate());
            assert_eq!((l.board.width, l.board.height), (6, 6), "{}", l.id);
            assert_eq!(l.targets.iter().filter(|p| p.piece == "king").count(), 1);
            assert!(std::iter::once(&l.player)
                .chain(&l.allies)
                .all(|p| p.piece != "king"));
            let count = l.targets.len();
            if i < 2 {
                assert_eq!(count, 1);
                let friends: Vec<_> = std::iter::once(&l.player).chain(&l.allies).collect();
                assert_eq!(friends.iter().filter(|p| p.piece == "rook").count(), 2);
                assert_eq!(friends.iter().filter(|p| p.piece == "queen").count(), 1);
                assert_eq!(friends.len(), 3);
                assert!(!crate::battle::Battle::new(l.clone()).in_check());
                assert!(
                    l.example_turns.len() <= 3,
                    "Opening checkmate lessons stay short"
                );
            } else {
                assert!(count > 1);
            }
            assert!(count >= last_count);
            last_count = count;
            let army = std::iter::once(&l.player)
                .chain(&l.allies)
                .map(|p| crate::movement::value(&p.piece))
                .sum::<i32>();
            let enemy = l
                .targets
                .iter()
                .map(|p| crate::movement::value(&p.piece))
                .sum::<i32>();
            assert!(army - enemy >= 5);
            let board = format!("{:?}{:?}{:?}{:?}", l.board, l.player, l.allies, l.targets);
            assert!(boards.insert(board), "{} repeats another board", l.id);
            let mut g = crate::session::Session::new(l.clone()).unwrap();
            for _ in 0..20 {
                if g.won() {
                    break;
                }
                assert_eq!(g.selected, None);
                let m = g.hint().expect("Checkmate teaching route has a hint");
                assert!(g.select(m.from));
                g.play(m.to).unwrap();
                g.reply();
                assert_eq!(g.selected, None);
            }
            assert!(g.won(), "{} hints win", l.id);
            assert_eq!(
                g.remaining(),
                1 + g
                    .units()
                    .iter()
                    .filter(|u| u.side == crate::battle::Side::Opponent && u.kind != "king")
                    .count()
            );
            let mut bad = l.clone();
            bad.player.piece = "king".into();
            assert!(bad.validate().is_err());
            let mut bad = l.clone();
            bad.targets.retain(|p| p.piece != "king");
            assert!(bad.validate().is_err());
        }
    }
    #[test]
    fn practice_shortest_routes_require_one_then_two_extra_moves_for_every_piece() {
        for (i, l) in levels().into_iter().take(10).enumerate() {
            let extra = i % 2 + 1;
            assert_eq!(l.minimum_quiet_moves as usize, extra);
            let g = Puzzle::new(l.clone()).unwrap();
            let shortest = g.shortest_solution().unwrap();
            assert!(
                shortest.len() >= l.targets.len() + extra,
                "{} can be finished too quickly",
                l.id
            );
            assert_eq!(shortest.len(), l.example_solution.len() - 1);
        }
    }
    #[test]
    fn every_match_has_a_friendly_pawn_and_later_matches_have_more() {
        for (i, l) in levels().into_iter().enumerate().take(20).skip(10) {
            let count = std::iter::once(&l.player)
                .chain(&l.allies)
                .filter(|p| p.piece == "pawn")
                .count();
            assert_eq!(
                count,
                if i < 15 {
                    1
                } else if i < 18 {
                    2
                } else {
                    3
                }
            );
        }
    }
    #[test]
    fn sixty_levels_have_unique_ids_descriptions_and_valid_teaching_routes() {
        let levels = levels();
        assert_eq!(levels.len(), 60);
        for l in levels {
            let mut g = crate::session::Session::new(l.clone()).unwrap();
            if l.is_match() {
                for step in &l.example_turns {
                    assert!(g.select(Square::parse(&step.from).unwrap()));
                    g.play(Square::parse(&step.to).unwrap()).unwrap();
                    g.reply();
                }
                assert!(l.is_checkmate() || g.battle.as_ref().unwrap().opponent_turns > 0);
            } else {
                for s in l.example_solution.iter().skip(1) {
                    g.play(Square::parse(s).unwrap()).unwrap();
                }
            }
            assert!(
                g.won() || (l.is_standard() && l.opening_only && !g.failed()),
                "{} must have a valid teaching route",
                l.id
            );
        }
    }
    #[test]
    fn regular_chess_world_grows_toward_a_balanced_full_board_finale() {
        let levels = levels();
        let mut last_counts = (0, 0);
        let mut advantage = i32::MAX;
        let mut positions = std::collections::HashSet::new();
        for (i, l) in levels[40..].iter().enumerate() {
            assert!(
                positions.insert(format!("{:?}{:?}{:?}", l.player, l.allies, l.targets)),
                "Duplicate chess board: {}",
                l.id
            );
            assert!(l.is_standard());
            assert_eq!((l.board.width, l.board.height), (8, 8));
            let player: Vec<_> = std::iter::once(&l.player).chain(&l.allies).collect();
            let counts = (player.len(), l.targets.len());
            assert!(counts.0 >= last_counts.0 && counts.1 >= last_counts.1);
            last_counts = counts;
            assert_eq!(player.iter().filter(|p| p.piece == "king").count(), 1);
            assert_eq!(l.targets.iter().filter(|p| p.piece == "king").count(), 1);
            let material = player
                .iter()
                .map(|p| crate::movement::value(&p.piece))
                .sum::<i32>()
                - l.targets
                    .iter()
                    .map(|p| crate::movement::value(&p.piece))
                    .sum::<i32>();
            assert!(material >= 0 && material <= advantage);
            advantage = material;
            assert_eq!(l.final_boss, i == 19);
            assert_eq!(l.icon.castle, i == 19);
        }
        assert_eq!(
            crate::standard::Standard::from_level(&levels[59])
                .unwrap()
                .board,
            cozy_chess::Board::default()
        );
        let worlds = crate::worlds::load(&levels);
        let mut save = Save::default();
        for l in &levels[..39] {
            save.complete(&l.id);
        }
        assert!(!crate::worlds::unlocked(&worlds, &levels, &save, 2));
        save.complete(&levels[39].id);
        assert!(crate::worlds::unlocked(&worlds, &levels, &save, 2));
        save.active_profile = 1;
        assert!(!crate::worlds::unlocked(&worlds, &levels, &save, 2));
    }
    #[test]
    fn progression_has_two_lessons_per_piece_then_ten_gentle_matches() {
        let ls = levels();
        for (i, kind) in ["rook", "bishop", "knight", "pawn", "queen"]
            .iter()
            .enumerate()
        {
            for (j, l) in ls[i * 2..i * 2 + 2].iter().enumerate() {
                assert!(!l.is_match());
                assert_eq!(l.player.piece, *kind);
                assert_eq!(l.icon.piece, *kind);
                assert_eq!(l.icon.badge, j as u8 + 1);
            }
        }
        let mut previous = (0, 0);
        for l in &ls[10..20] {
            assert!(l.is_match());
            let army = std::iter::once(&l.player)
                .chain(l.allies.iter())
                .map(|p| crate::movement::value(&p.piece))
                .sum::<i32>();
            let enemy = l
                .targets
                .iter()
                .map(|p| crate::movement::value(&p.piece))
                .sum::<i32>();
            assert!(army > enemy, "Player has a material advantage in {}", l.id);
            let strength = (l.opponent.overlook_every, l.opponent.safety);
            assert!(strength.0 >= previous.0 && strength.1 >= previous.1);
            previous = strength;
        }
    }

    #[test]
    fn completing_each_level_unlocks_only_the_next_and_profiles_stay_separate() {
        let levels = levels();
        let mut save = Save::default();
        for i in 0..levels.len() {
            assert!(unlocked(&levels, &save, i));
            if i + 1 < levels.len() {
                assert!(!unlocked(&levels, &save, i + 1));
            }
            save.complete(&levels[i].id);
        }
        save.active_profile = 1;
        assert!(!unlocked(&levels, &save, 1));
    }
    #[test]
    fn arrows_and_scrolling_cover_twenty_and_larger_campaigns() {
        for count in [20, 100] {
            let mut i = 0;
            while let Some(n) = neighbor(i, Direction::Right, count) {
                i = n;
            }
            assert_eq!(i, count - 1);
            let camera = camera_x(node(i).0, count);
            assert!(camera > 0.);
            assert!(node(i).0 - camera < 1280.);
            while let Some(n) = neighbor(i, Direction::Left, count) {
                i = n;
            }
            assert_eq!(i, 0);
        }
        assert_eq!(neighbor(0, Direction::Left, 20), None);
        assert_eq!(neighbor(0, Direction::Up, 20), Some(1));
        assert_eq!(neighbor(1, Direction::Down, 20), Some(2));
    }
    #[test]
    fn cursor_is_per_profile_and_cannot_restore_a_locked_or_missing_level() {
        let levels = levels();
        let mut save = Save::default();
        save.map_level[0] = levels[19].id.clone();
        assert_eq!(restore_cursor(&levels, &save), 0);
        for l in &levels[..19] {
            save.complete(&l.id);
        }
        assert_eq!(restore_cursor(&levels, &save), 19);
        save.active_profile = 1;
        assert_eq!(restore_cursor(&levels, &save), 0);
    }
    #[test]
    fn broken_links_and_duplicate_ids_are_rejected() {
        let mut ls = levels();
        ls[0].next_level_id = None;
        assert!(validate(&ls).is_err());
        let mut ls = levels();
        ls[1].id = ls[0].id.clone();
        assert!(validate(&ls).is_err());
    }
    #[test]
    fn following_hints_finishes_every_practice_lesson() {
        for l in levels().into_iter().filter(|l| !l.is_match()) {
            let mut g = Puzzle::new(l).unwrap();
            for _ in 0..100 {
                if g.won() {
                    break;
                }
                let hint = g.hint().expect("A hint exists");
                g.move_to(hint).unwrap();
            }
            assert!(g.won(), "{} hints must finish", g.level.id);
        }
    }
}
