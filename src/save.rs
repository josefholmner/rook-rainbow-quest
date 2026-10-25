use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Save {
    pub version: u32,
    pub started: [bool; 2],
    pub all_levels_unlocked: [bool; 2],
    pub world_cursor: [String; 2],
    pub muted: bool,
    pub reduced_motion: bool,
    pub volume: f32,
    pub active_profile: usize,
    pub profiles: [BTreeSet<String>; 2],
    pub map_level: [String; 2],
    pub language: crate::language::Language,
}
impl Default for Save {
    fn default() -> Self {
        Self {
            version: 1,
            started: [false; 2],
            all_levels_unlocked: [false; 2],
            world_cursor: ["world-01".into(), "world-01".into()],
            muted: false,
            reduced_motion: false,
            volume: 0.55,
            active_profile: 0,
            profiles: Default::default(),
            map_level: ["meadow-01".into(), "meadow-01".into()],
            language: crate::language::Language::Swedish,
        }
    }
}
impl Save {
    pub fn has_adventure(&self) -> bool {
        self.started[self.active_profile] || !self.profiles[self.active_profile].is_empty()
    }
    /// Called only after the UI receives Yes on its reset confirmation.
    pub fn new_game(&mut self, first_level: &str, first_world: &str) {
        let i = self.active_profile;
        self.profiles[i].clear();
        self.all_levels_unlocked[i] = false;
        self.started[i] = true;
        self.map_level[i] = first_level.into();
        self.world_cursor[i] = first_world.into();
    }

    /// Parent shortcut: unlock access without pretending any level was won.
    pub fn unlock_all_levels(&mut self) {
        self.all_levels_unlocked[self.active_profile] = true;
        self.started[self.active_profile] = true;
    }
    pub fn complete(&mut self, id: &str) -> bool {
        self.profiles[self.active_profile.min(1)].insert(id.into())
    }
    pub fn completed(&self, id: &str) -> bool {
        self.profiles[self.active_profile.min(1)].contains(id)
    }
    pub fn path() -> PathBuf {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".local/share")))
            .unwrap_or_else(std::env::temp_dir)
            .join("RookRainbowQuest")
            .join("progress.json")
    }
    pub fn load(path: &Path) -> (Self, Option<String>) {
        match fs::read_to_string(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => (Self::default(), None),
            Ok(raw) => match serde_json::from_str::<Self>(&raw) {
                Ok(mut s) if s.version == 1 && s.active_profile < 2 && s.volume.is_finite() => {
                    s.volume = s.volume.clamp(0., 1.);
                    (s, None)
                }
                _ => (
                    Self::default(),
                    Some("Couldn't read saved progress. Starting fresh.".into()),
                ),
            },
            Err(_) => (
                Self::default(),
                Some("Couldn't read saved progress. Playing without it.".into()),
            ),
        }
    }
    pub fn write(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp = path.with_extension("tmp");
        let bytes = serde_json::to_vec_pretty(self)?;
        {
            use io::Write;
            let mut file = fs::File::create(&temp)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        // On Windows std::fs::rename replaces an existing file in one operation.
        fs::rename(temp, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn renamed_game_keeps_progress_in_its_own_folder() {
        assert!(Save::path().ends_with(Path::new("RookRainbowQuest").join("progress.json")));
    }
    fn path(name: &str) -> PathBuf {
        std::env::temp_dir()
            .join(format!(
                "rook-rainbow-quest-test-{}-{name}",
                std::process::id()
            ))
            .join("save.json")
    }
    #[test]
    fn unlock_all_persists_per_player_without_awarding_stars_and_new_game_resets_it() {
        let mut save = Save::default();
        save.complete("meadow-01");
        let earned = save.profiles.clone();
        save.unlock_all_levels();
        assert!(save.has_adventure());
        assert_eq!(save.profiles, earned);
        let raw = serde_json::to_string(&save).unwrap();
        let mut restored: Save = serde_json::from_str(&raw).unwrap();
        assert_eq!(restored.all_levels_unlocked, [true, false]);
        let levels = crate::campaign::levels();
        let worlds = crate::worlds::load(&levels);
        assert!(crate::campaign::unlocked(&levels, &restored, 39));
        assert!(crate::worlds::unlocked(&worlds, &levels, &restored, 1));
        assert!(!crate::campaign::unlocked(&levels, &restored, levels.len()));
        assert!(!crate::worlds::unlocked(
            &worlds,
            &levels,
            &restored,
            worlds.len()
        ));
        restored.active_profile = 1;
        assert!(!crate::campaign::unlocked(&levels, &restored, 39));
        assert!(!crate::worlds::unlocked(&worlds, &levels, &restored, 1));
        restored.unlock_all_levels();
        restored.active_profile = 0;
        restored.new_game("meadow-01", "world-01");
        assert_eq!(restored.all_levels_unlocked, [false, true]);
        assert!(!crate::campaign::unlocked(&levels, &restored, 39));
        let legacy: Save = serde_json::from_str(r#"{"version":1}"#).unwrap();
        assert_eq!(legacy.all_levels_unlocked, [false, false]);
    }
    #[test]
    fn continue_is_disabled_until_new_game_and_legacy_stars_enable_it() {
        let mut s = Save::default();
        assert!(!s.has_adventure());
        s.new_game("meadow-01", "world-01");
        assert!(s.has_adventure());
        let raw = serde_json::to_string(&s).unwrap();
        let restored: Save = serde_json::from_str(&raw).unwrap();
        assert!(restored.has_adventure());
        s.active_profile = 1;
        assert!(!s.has_adventure());
        s.complete("meadow-01");
        assert!(s.has_adventure());
    }
    #[test]
    fn new_game_resets_only_selected_adventure_and_keeps_preferences() {
        let mut s = Save::default();
        s.complete("old-level");
        s.map_level[0] = "old-level".into();
        s.world_cursor[0] = "world-02".into();
        s.active_profile = 1;
        s.complete("other-level");
        s.active_profile = 0;
        s.language = crate::language::Language::English;
        s.volume = 0.3;
        s.muted = true;
        let other = s.profiles[1].clone();
        s.new_game("meadow-01", "world-01");
        assert!(s.profiles[0].is_empty());
        assert_eq!(s.profiles[1], other);
        assert_eq!(s.map_level[0], "meadow-01");
        assert_eq!(s.world_cursor[0], "world-01");
        assert_eq!(s.volume, 0.3);
        assert!(s.muted);
        assert_eq!(s.language, crate::language::Language::English);
    }
    #[test]
    fn stars_are_idempotent_and_profiles_are_independent() {
        let mut s = Save::default();
        assert!(s.complete("meadow-01"));
        assert!(!s.complete("meadow-01"));
        s.active_profile = 1;
        assert!(!s.completed("meadow-01"));
        s.complete("meadow-01");
        assert_eq!(s.profiles[0].len(), 1);
        assert_eq!(s.profiles[1].len(), 1);
    }
    #[test]
    fn roundtrip_and_replace_existing_save() {
        let p = path("roundtrip");
        let mut s = Save::default();
        s.complete("meadow-01");
        s.write(&p).unwrap();
        s.muted = true;
        s.reduced_motion = true;
        s.active_profile = 1;
        s.language = crate::language::Language::English;
        s.write(&p).unwrap();
        assert_eq!(Save::load(&p), (s, None));
        assert!(!p.with_extension("tmp").exists());
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn missing_corrupt_and_future_saves_recover() {
        let p = path("corrupt");
        assert_eq!(Save::load(&p), (Save::default(), None));
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        for raw in ["not json", r#"{"version":99}"#, r#"{"active_profile":12}"#] {
            fs::write(&p, raw).unwrap();
            let (s, w) = Save::load(&p);
            assert_eq!(s, Save::default());
            assert!(w.is_some());
        }
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn write_failure_is_reported() {
        let p = path("failure");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, "blocking parent").unwrap();
        assert!(Save::default().write(&p.join("nested.json")).is_err());
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn original_one_level_saves_keep_stars_and_gain_default_map_cursors() {
        let p = path("migration");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(
            &p,
            r#"{"version":1,"muted":true,"profiles":[["meadow-01"],[]]}"#,
        )
        .unwrap();
        let (save, warning) = Save::load(&p);
        assert!(warning.is_none());
        assert!(save.muted);
        assert!(save.completed("meadow-01"));
        assert_eq!(save.map_level, ["meadow-01", "meadow-01"]);
        assert_eq!(save.language, crate::language::Language::Swedish);
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn language_choice_roundtrips_without_changing_stars_or_player() {
        let p = path("language");
        let mut save = Save::default();
        assert_eq!(save.language, crate::language::Language::Swedish);
        save.complete("meadow-01");
        save.language = crate::language::Language::English;
        save.write(&p).unwrap();
        let (loaded, warning) = Save::load(&p);
        assert!(warning.is_none());
        assert_eq!(loaded, save);
        save.language = crate::language::Language::Swedish;
        save.write(&p).unwrap();
        assert_eq!(Save::load(&p).0, save);
        fs::remove_file(p).unwrap();
    }
}
