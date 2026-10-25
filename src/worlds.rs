//! Worlds refer to stable level IDs, keeping the level trail expandable.
use crate::{
    campaign::{self, Direction},
    rules::Level,
    save::Save,
};
use serde::Deserialize;
use std::ops::Range;
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub id: String,
    pub title: String,
    pub first_level_id: String,
    pub last_level_id: String,
}
impl World {
    pub fn completed(&self, levels: &[Level], save: &Save) -> bool {
        self.range(levels)
            .is_some_and(|range| range.into_iter().all(|i| save.completed(&levels[i].id)))
    }
    pub fn range(&self, levels: &[Level]) -> Option<Range<usize>> {
        let first = levels.iter().position(|l| l.id == self.first_level_id)?;
        let last = levels.iter().position(|l| l.id == self.last_level_id)?;
        (first <= last).then_some(first..last + 1)
    }
}
pub fn load(levels: &[Level]) -> Vec<World> {
    let worlds: Vec<World> =
        serde_json::from_str(include_str!("../worlds/worlds.json")).expect("World data");
    validate(&worlds, levels).expect("Valid world membership");
    worlds
}
pub fn validate(worlds: &[World], levels: &[Level]) -> Result<(), String> {
    let mut next = 0;
    let mut ids = std::collections::HashSet::new();
    for world in worlds {
        let range = world
            .range(levels)
            .ok_or("Unknown or reversed level bounds")?;
        if range.start != next || !ids.insert(&world.id) || world.title.trim().is_empty() {
            return Err("Worlds must uniquely cover consecutive levels".into());
        }
        next = range.end;
    }
    if worlds.is_empty() || next != levels.len() {
        return Err("Every level belongs to one world".into());
    }
    Ok(())
}
pub fn unlocked(worlds: &[World], levels: &[Level], save: &Save, index: usize) -> bool {
    index < worlds.len()
        && (save.all_levels_unlocked[save.active_profile]
            || worlds[..index].iter().all(|w| {
                w.range(levels)
                    .unwrap()
                    .all(|i| save.completed(&levels[i].id))
            }))
}
pub fn restore(worlds: &[World], levels: &[Level], save: &Save) -> usize {
    worlds
        .iter()
        .position(|w| w.id == save.world_cursor[save.active_profile])
        .filter(|i| unlocked(worlds, levels, save, *i))
        .unwrap_or(0)
}
pub fn neighbor(
    index: usize,
    direction: Direction,
    worlds: &[World],
    levels: &[Level],
    save: &Save,
) -> Option<usize> {
    campaign::neighbor(index, direction, worlds.len())
        .filter(|i| unlocked(worlds, levels, save, *i))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn world_completion_requires_all_earned_stars_and_is_per_player() {
        let levels = campaign::levels();
        let worlds = load(&levels);
        let mut save = Save::default();
        save.unlock_all_levels();
        assert!(!worlds[0].completed(&levels, &save));
        save.complete(&levels[19].id);
        assert!(!worlds[0].completed(&levels, &save));
        for level in &levels[..19] {
            save.complete(&level.id);
        }
        assert!(worlds[0].completed(&levels, &save));
        assert!(!worlds[1].completed(&levels, &save));
        save.active_profile = 1;
        assert!(!worlds[0].completed(&levels, &save));
        save.active_profile = 0;
        for level in &levels[20..] {
            save.complete(&level.id);
        }
        assert!(worlds[1].completed(&levels, &save));
        save.new_game(&levels[0].id, &worlds[0].id);
        assert!(!worlds[0].completed(&levels, &save));
        assert!(!worlds[1].completed(&levels, &save));
    }
    #[test]
    fn first_world_contains_all_twenty_levels_and_invalid_bounds_are_rejected() {
        let ls = campaign::levels();
        let mut ws = load(&ls);
        assert_eq!(ws.len(), 3);
        assert_eq!(ws[2].range(&ls), Some(40..60));
        assert_eq!(ws[1].range(&ls), Some(20..40));
        assert_eq!(ws[0].range(&ls), Some(0..20));
        ws[0].last_level_id = "missing".into();
        assert!(validate(&ws, &ls).is_err());
    }
    #[test]
    fn second_world_unlocks_after_world_one_and_remains_per_profile() {
        let levels = campaign::levels();
        let ws = load(&levels);
        let mut save = Save::default();
        assert_eq!(neighbor(0, Direction::Right, &ws, &levels, &save), None);
        for l in &levels[..19] {
            save.complete(&l.id);
        }
        assert!(!unlocked(&ws, &levels, &save, 1));
        save.complete(&levels[19].id);
        assert_eq!(neighbor(0, Direction::Right, &ws, &levels, &save), Some(1));
        assert_eq!(neighbor(1, Direction::Left, &ws, &levels, &save), Some(0));
        assert_eq!(neighbor(1, Direction::Right, &ws, &levels, &save), None);
        save.world_cursor[0] = "world-02".into();
        assert_eq!(restore(&ws, &levels, &save), 1);
        save.active_profile = 1;
        assert!(!unlocked(&ws, &levels, &save, 1));
        assert_eq!(restore(&ws, &levels, &save), 0);
    }
}
