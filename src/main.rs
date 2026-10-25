#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod performance;

use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound},
    prelude::*,
};
use rook_rainbow_quest::{
    battle::{Action, Move, Side},
    campaign::{self, Direction},
    language::{self, Language},
    lifecycle::Shutdown,
    persistence::SaveWriter,
    rules::{Level, Square},
    save::Save,
    session::Session,
    viewport::{curve_segments, font_raster, Viewport},
    worlds::{self, World},
};
use std::path::PathBuf;

thread_local! {
    static FONT_RASTERS: std::cell::RefCell<std::collections::HashSet<u16>> = std::cell::RefCell::new(std::collections::HashSet::new());
    static TEXT_MATERIAL: std::cell::OnceCell<Material> = const { std::cell::OnceCell::new() };
    static RENDER_SCALE: std::cell::Cell<f32> = const { std::cell::Cell::new(1.) };
    static LANGUAGE: std::cell::Cell<Language> = const { std::cell::Cell::new(Language::Swedish) };
    static RAW_ART:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
    static PIECE_SPRITES:std::cell::OnceCell<std::collections::HashMap<(String,Side),(Texture2D,Rect)>>=const{std::cell::OnceCell::new()};
    static MASCOT_SPRITE:std::cell::OnceCell<(Texture2D,Rect)>=const{std::cell::OnceCell::new()};
    static SPRITE_QUEUE:std::cell::RefCell<Vec<(Texture2D,Rect,Vec2,Vec2)>>=const{std::cell::RefCell::new(Vec::new())};
    static TEXT_QUEUE:std::cell::RefCell<Vec<(String,Vec2,f32,Color)>>=const{std::cell::RefCell::new(Vec::new())};
    static UI_FONT: std::cell::RefCell<Option<Font>> = const { std::cell::RefCell::new(None) };
}

const W: f32 = 1280.;
const H: f32 = 800.;
const INK: Color = Color::new(0.16, 0.16, 0.25, 1.);
const CREAM: Color = Color::new(1., 0.97, 0.88, 1.);
const CORAL: Color = Color::new(0.98, 0.40, 0.40, 1.);
const MINT: Color = Color::new(0.65, 0.84, 0.69, 1.);
const GOLD: Color = Color::new(1., 0.77, 0.25, 1.);
const PURPLE: Color = Color::new(0.68, 0.52, 0.88, 1.);
const BLUE: Color = Color::new(0.41, 0.73, 0.94, 1.);

fn config() -> Conf {
    Conf {
        window_title: concat!("Rook Rainbow Quest v", env!("CARGO_PKG_VERSION")).into(),
        icon: Some(macroquad::miniquad::conf::Icon {
            small: *include_bytes!("../assets/icon-16.rgba"),
            medium: *include_bytes!("../assets/icon-32.rgba"),
            big: *include_bytes!("../assets/icon-64.rgba"),
        }),
        window_width: 1280,
        window_height: 800,
        high_dpi: true,
        window_resizable: true,
        sample_count: if std::env::args().any(|s| s == "--performance-msaa1") {
            1
        } else {
            4
        },
        ..Default::default()
    }
}

fn native_camera(viewport: (i32, i32, i32, i32)) -> Camera2D {
    let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., W, H));
    // Macroquad flips camera Y for the default framebuffer, unlike render targets.
    // Cancel that flip so UI y=0 remains at the visible top, matching hit-testing.
    camera.zoom.y = camera.zoom.y.abs();
    camera.viewport = Some(viewport);
    camera
}

#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Main,
    Worlds,
    Map,
    Level,
}
struct Motion {
    from: Square,
    to: Square,
    start: f64,
    captured: bool,
    promoted: bool,
    kind: String,
    side: Side,
}
struct MapTravel {
    from: Vec2,
    to: Vec2,
    start: f64,
}
struct Spark {
    p: Vec2,
    v: Vec2,
    color: Color,
    life: f32,
}
struct App {
    screen: Screen,
    worlds: Vec<World>,
    world_index: usize,
    world_avatar: Vec2,
    world_travel: Option<MapTravel>,
    world_camera: f32,
    confirm_new: bool,
    game: Session,
    levels: Vec<Level>,
    map_index: usize,
    map_avatar: Vec2,
    map_camera: f32,
    map_travel: Option<MapTravel>,
    map_key_at: f64,
    save: Save,
    save_writer: Option<SaveWriter>,
    settings: bool,
    shutdown: Shutdown,
    fullscreen: bool,
    selected: bool,
    motion: Option<Motion>,
    hint: Option<(Move, f64)>,
    enemy_due: Option<f64>,
    failed_at: Option<f64>,
    message: String,
    message_until: f64,
    won_at: Option<f64>,
    completed_world: Option<usize>,
    promotion_to: Option<Square>,
    particles: Vec<Spark>,
    sounds: Vec<Option<Sound>>,
    world_backgrounds: [Texture2D; 3],
    universe: Texture2D,
    initial_games: Vec<Session>,
    hovered: Option<usize>,
    focus: Option<usize>,
    interactive: Vec<usize>,
    activated: Option<usize>,
    clicked: bool,
    pointer: Vec2,
    invalid_until: f64,
    smoke: bool,
    mute_audio_available: bool,
}

impl App {
    async fn new(smoke: bool) -> Self {
        let save_path = if smoke {
            std::env::temp_dir()
                .join(format!("rook-rainbow-quest-smoke-{}", std::process::id()))
                .join("progress.json")
        } else {
            Save::path()
        };
        let (save, warning) = Save::load(&save_path);
        let levels = campaign::levels();
        let initial_games: Vec<_> = levels
            .iter()
            .cloned()
            .map(|l| Session::new(l).unwrap())
            .collect();
        let worlds = worlds::load(&levels);
        let world_index = worlds::restore(&worlds, &levels, &save);
        let range = worlds[world_index].range(&levels).unwrap();
        let map_index = campaign::restore_cursor(&levels, &save);
        let map_index = if range.contains(&map_index) {
            map_index
        } else {
            range.start
        };
        let (ax, ay) = campaign::node(map_index - range.start);
        let mut sounds = vec![];
        for (notes, duration) in [
            (&[440., 660.][..], 0.06),
            (&[523.25, 783.99][..], 0.10),
            (&[659.25, 880.][..], 0.13),
            (&[523.25, 659.25, 783.99, 1046.5][..], 0.18),
        ] {
            let bytes = wave(notes, duration);
            sounds.push(load_sound_from_bytes(&bytes).await.ok());
        }
        let available = sounds.iter().any(Option::is_some);
        // Decode and upload once, before play; changing worlds only selects a texture.
        let world_backgrounds = [
            include_bytes!("../assets/meadow-v2.png").as_slice(),
            include_bytes!("../assets/castle-v082.png").as_slice(),
            include_bytes!("../assets/royal-v082.png").as_slice(),
        ]
        .map(|bytes| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        });
        let universe = Texture2D::from_file_with_format(
            include_bytes!("../assets/universe-v06.png"),
            Some(ImageFormat::Png),
        );
        universe.set_filter(FilterMode::Linear);
        let save_writer = (!smoke || performance::enabled()).then(|| SaveWriter::new(save_path));
        Self {
            screen: Screen::Main,
            world_avatar: world_node(world_index),
            world_travel: None,
            world_camera: (world_node(world_index).x - 640.).max(0.),
            worlds,
            world_index,
            confirm_new: false,
            game: initial_games[map_index].clone(),
            initial_games,
            map_index,
            map_avatar: vec2(ax, ay),
            map_camera: campaign::camera_x(ax, range.len()),
            map_travel: None,
            map_key_at: 0.,
            levels,
            save,
            save_writer,
            settings: false,
            shutdown: Shutdown::default(),
            fullscreen: false,
            selected: Level::embedded().initially_selected,
            motion: None,
            hint: None,
            enemy_due: None,
            failed_at: None,
            message: warning.unwrap_or_default(),
            message_until: 8.,
            won_at: None,
            completed_world: None,
            promotion_to: None,
            particles: vec![],
            sounds,
            world_backgrounds,
            universe,
            hovered: None,
            focus: None,
            interactive: vec![],
            activated: None,
            clicked: false,
            pointer: vec2(-1., -1.),
            invalid_until: 0.,
            smoke,
            mute_audio_available: available,
        }
    }
    fn sound(&self, index: usize) {
        if !self.save.muted {
            if let Some(s) = &self.sounds[index] {
                play_sound(
                    s,
                    PlaySoundParams {
                        looped: false,
                        volume: self.save.volume * 0.5,
                    },
                );
            }
        }
    }
    fn persist(&mut self) {
        if self.smoke && !performance::enabled() {
            return;
        }
        let timer = performance::begin();
        let result = self.save_writer.as_ref().unwrap().queue(&self.save);
        performance::end("save", timer);
        if result.is_err() {
            self.note("Couldn't save progress. You can still play!", 6.);
        }
    }
    fn note(&mut self, text: &str, seconds: f64) {
        self.message = text.into();
        self.message_until = get_time() + seconds;
    }
    fn restore_map(&mut self) {
        self.world_index = worlds::restore(&self.worlds, &self.levels, &self.save);
        self.world_avatar = world_node(self.world_index);
        self.world_camera = (self.world_avatar.x - 640.).max(0.);
        self.world_travel = None;
        self.map_index = campaign::restore_cursor(&self.levels, &self.save);
        let range = self.world_range();
        if !range.contains(&self.map_index) {
            self.map_index = range.start;
        }
        let (x, y) = self.level_node(self.map_index);
        self.map_avatar = vec2(x, y);
        self.map_camera = campaign::camera_x(x, self.world_range().len());
        self.map_travel = None;
        self.restart();
    }
    fn walk_to(&mut self, index: usize) {
        if self.settings || self.map_travel.is_some() || !self.world_range().contains(&index) {
            return;
        }
        if !campaign::unlocked(&self.levels, &self.save, index) {
            self.note("Finish this level to open the next!", 2.5);
            return;
        }
        if self.map_index.abs_diff(index) != 1 {
            return;
        }
        let (x, y) = self.level_node(index);
        let to = vec2(x, y);
        self.map_index = index;
        self.map_travel = Some(MapTravel {
            from: self.map_avatar,
            to,
            start: get_time(),
        });
        self.save.map_level[self.save.active_profile] = self.levels[index].id.clone();
        self.focus = None;
        self.sound(0);
        self.persist();
    }
    fn navigate(&mut self, direction: Direction) {
        if self.screen != Screen::Map || self.settings {
            return;
        }
        if let Some(index) = campaign::neighbor(
            self.map_index - self.world_range().start,
            direction,
            self.world_range().len(),
        )
        .map(|i| i + self.world_range().start)
        {
            self.walk_to(index);
        }
    }
    fn start(&mut self) {
        if self.map_travel.is_some()
            || !campaign::unlocked(&self.levels, &self.save, self.map_index)
        {
            return;
        }
        if self.game.level.id != self.levels[self.map_index].id {
            self.restart();
        }
        self.screen = Screen::Level;
        self.focus = None;
        if self.game.won() {
            self.restart();
        }
        self.selected = self.game.level.initially_selected;
        if self.game.level.is_match() {
            self.game.selected = None;
            self.selected = false;
        }
        self.sound(1);
    }
    fn restart(&mut self) {
        self.game = self.initial_games[self.map_index].clone();
        self.motion = None;
        self.hint = None;
        self.won_at = None;
        self.completed_world = None;
        self.promotion_to = None;
        self.failed_at = None;
        self.enemy_due = None;
        self.particles.clear();
        self.selected = self.game.level.initially_selected;
        if self.game.level.is_match() {
            self.game.selected = None;
            self.selected = false;
        }
        self.focus = None;
    }
    fn animate(&mut self, action: Action) {
        self.hint = None;
        self.motion = Some(Motion {
            from: action.movement.from,
            to: action.movement.to,
            start: get_time(),
            captured: action.captured,
            promoted: action.promoted,
            kind: action.kind,
            side: action.side,
        });
        self.sound(0);
    }
    fn move_piece(&mut self, to: Square) {
        if self.motion.is_some() || !self.game.player_turn() || self.failed_at.is_some() {
            return;
        }
        if self.promotion_to.is_some() {
            return;
        }
        if !self.smoke && !self.game.promotion_choices(to).is_empty() {
            self.promotion_to = Some(to);
            self.focus = None;
            return;
        }
        match self.game.play(to) {
            Ok(action) => self.animate(action),
            Err(_) => {
                self.invalid_until = get_time() + 0.35;
                self.note("Try a glowing square", 2.);
            }
        }
    }
    fn settle(&mut self) {
        if self.game.won() && self.won_at.is_none() {
            self.won_at = Some(get_time());
            self.focus = None;
            self.sound(3);
            let id = self.game.level.id.clone();
            let was_complete = self.worlds[self.world_index].completed(&self.levels, &self.save);
            self.save.complete(&id);
            if (!was_complete || self.worlds[self.world_index].last_level_id == id)
                && self.worlds[self.world_index].completed(&self.levels, &self.save)
            {
                self.completed_world = Some(self.world_index);
            }
            self.persist();
            self.burst(vec2(640., 285.), 75);
        } else if self.game.failed() && self.failed_at.is_none() {
            self.failed_at = Some(get_time());
            self.focus = None;
            self.enemy_due = None;
        } else if self.game.level.is_match()
            && !self.game.player_turn()
            && self.won_at.is_none()
            && self.failed_at.is_none()
        {
            self.enemy_due = Some(get_time() + if self.smoke { 0.06 } else { 0.65 });
        }
    }
    fn update(&mut self) {
        if self
            .save_writer
            .as_ref()
            .and_then(SaveWriter::error)
            .is_some()
        {
            self.note("Couldn't save progress. You can still play!", 6.);
        }
        let dt = get_frame_time().min(0.05);
        let now = get_time();
        if let Some(travel) = &self.map_travel {
            let u = if self.save.reduced_motion {
                1.
            } else {
                ((now - travel.start) / 0.32).clamp(0., 1.) as f32
            };
            self.map_avatar = travel.from.lerp(travel.to, u * u * (3. - 2. * u));
            if u >= 1. {
                self.map_travel = None;
            }
        }
        let target_camera = campaign::camera_x(self.map_avatar.x, self.world_range().len());
        self.map_camera = if self.save.reduced_motion {
            target_camera
        } else {
            self.map_camera + (target_camera - self.map_camera) * (dt * 12.).min(1.)
        };
        if self
            .motion
            .as_ref()
            .is_some_and(|m| now - m.start >= if self.save.reduced_motion { 0.08 } else { 0.24 })
        {
            let m = self.motion.take().unwrap();
            if m.promoted {
                let notice = if self.game.level.is_standard() {
                    match self
                        .game
                        .units()
                        .iter()
                        .find(|u| u.square == m.to && u.side == m.side)
                        .map(|u| u.kind.as_str())
                    {
                        Some("knight") => "A new knight!",
                        Some("bishop") => "A new bishop!",
                        Some("rook") => "A new rook!",
                        _ => "A new queen!",
                    }
                } else {
                    "A new queen!"
                };
                self.note(notice, 2.5);
                self.sound(3);
            }
            if m.captured {
                self.sound(2);
                self.burst(center(m.to, &self.game.level), 14);
            }
            self.settle();
        }
        if self.motion.is_none()
            && self.enemy_due.is_some_and(|t| get_time() >= t)
            && !self.settings
            && self.screen == Screen::Level
        {
            self.enemy_due = None;
            let reply = self.game.reply();
            self.selected = self.game.selected.is_some();
            if let Some(action) = reply {
                self.animate(action);
            } else {
                self.settle();
            }
        }
        if let Some(travel) = &self.world_travel {
            let u = if self.save.reduced_motion {
                1.
            } else {
                ((now - travel.start) / 0.32).clamp(0., 1.) as f32
            };
            self.world_avatar = travel.from.lerp(travel.to, u * u * (3. - 2. * u));
            if u >= 1. {
                self.world_travel = None;
            }
        }
        let camera = (self.world_avatar.x - 640.)
            .clamp(0., (world_node(self.worlds.len() - 1).x - 640.).max(0.));
        self.world_camera = if self.save.reduced_motion {
            camera
        } else {
            self.world_camera + (camera - self.world_camera) * (dt * 12.).min(1.)
        };
        for p in &mut self.particles {
            p.life -= dt;
            p.p += p.v * dt;
            p.v.y += 180. * dt;
        }
        self.particles.retain(|p| p.life > 0.);
        if self.hint.is_some_and(|(_, until)| now > until) {
            self.hint = None;
        }
    }
    fn burst(&mut self, p: Vec2, n: usize) {
        if self.save.reduced_motion {
            return;
        }
        for _ in 0..n {
            self.particles.push(Spark {
                p,
                v: vec2(rand::gen_range(-220., 220.), rand::gen_range(-280., 50.)),
                color: [GOLD, CORAL, PURPLE, BLUE, WHITE][rand::gen_range(0, 5)],
                life: rand::gen_range(0.7, 1.7),
            });
        }
    }
    fn input(&mut self, offset: Vec2, scale: f32) {
        let viewport = Viewport {
            scale,
            x: offset.x,
            y: offset.y,
        };
        let (mx, my) = mouse_position();
        self.pointer = viewport
            .to_virtual(mx, my)
            .map(|(x, y)| vec2(x, y))
            .unwrap_or(vec2(-1., -1.));
        self.clicked = is_mouse_button_pressed(MouseButton::Left);
        self.activated = None;
        self.hovered = None;
        if is_key_pressed(KeyCode::Tab) {
            let n = self.interactive.len();
            if n > 0 {
                let old = self
                    .focus
                    .and_then(|id| self.interactive.iter().position(|v| *v == id));
                let i = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                    old.map_or(n - 1, |i| (i + n - 1) % n)
                } else {
                    old.map_or(0, |i| (i + 1) % n)
                };
                self.focus = Some(self.interactive[i]);
            }
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            self.activated = self.focus.or(if self.confirm_new {
                Some(205)
            } else if self.settings {
                Some(99)
            } else if self.promotion_to.is_some() {
                Some(120)
            } else if self.screen == Screen::Main {
                Some(if self.save.has_adventure() { 201 } else { 202 })
            } else if self.screen == Screen::Worlds {
                Some(200)
            } else if self.screen == Screen::Map {
                Some(1)
            } else if self.won_at.is_some() {
                Some(if self.completed_world.is_some() {
                    33
                } else {
                    30
                })
            } else {
                None
            });
        }
        if is_key_pressed(KeyCode::F11) {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        if is_key_pressed(KeyCode::M) {
            self.save.muted = !self.save.muted;
            self.persist();
        }
        if is_key_pressed(KeyCode::Escape) {
            if self.confirm_new {
                self.confirm_new = false;
                self.focus = None;
            } else if self.settings {
                self.settings = false;
            } else if self.promotion_to.is_some() {
                self.promotion_to = None;
                self.focus = None;
            } else if self.screen == Screen::Level {
                if self.completed_world.is_some() {
                    self.return_to_galaxy();
                } else {
                    self.screen = Screen::Map;
                    self.focus = None;
                }
            } else if self.screen == Screen::Map {
                self.screen = Screen::Worlds;
                self.focus = None;
            } else if self.screen == Screen::Worlds {
                self.screen = Screen::Main;
                self.focus = None;
                self.persist();
            }
        }
        if matches!(self.screen, Screen::Map | Screen::Worlds)
            && !self.settings
            && !self.confirm_new
            && get_time() >= self.map_key_at
        {
            let keys = [
                (KeyCode::Right, Direction::Right),
                (KeyCode::Left, Direction::Left),
                (KeyCode::Up, Direction::Up),
                (KeyCode::Down, Direction::Down),
            ];
            for (key, direction) in keys {
                if is_key_pressed(key) || is_key_down(key) {
                    if self.screen == Screen::Worlds {
                        self.navigate_world(direction);
                    } else {
                        self.navigate(direction);
                    }
                    self.map_key_at = get_time() + 0.34;
                    break;
                }
            }
        }
        self.interactive.clear();
    }
    fn button(&mut self, id: usize, r: Rect, label: &str, color: Color, enabled: bool) -> bool {
        if enabled {
            self.interactive.push(id);
        }
        let over = enabled && r.contains(self.pointer);
        if over {
            self.hovered = Some(id);
        }
        let lift = if over && !self.save.reduced_motion {
            -3.
        } else {
            0.
        };
        let r = Rect::new(r.x, r.y + lift, r.w, r.h);
        if self.focus == Some(id) {
            rounded(
                Rect::new(r.x - 5., r.y - 5., r.w + 10., r.h + 10.),
                22.,
                INK,
            );
        }
        rounded(
            Rect::new(r.x, r.y + 6., r.w, r.h),
            18.,
            Color::new(0.2, 0.22, 0.27, 0.18),
        );
        rounded(
            r,
            18.,
            if enabled {
                color
            } else {
                Color::new(0.78, 0.80, 0.77, 1.)
            },
        );
        rounded(
            Rect::new(r.x + 4., r.y + 3., r.w - 8., 3.),
            1.5,
            Color::new(1., 1., 1., 0.4),
        );
        text_center(
            label,
            r.x + r.w / 2.,
            r.y + r.h / 2. + 9.,
            {
                let size = if r.w < 125. { 22. } else { 27. };
                let width = text_width(label, size);
                size.min(size * (r.w - 22.) / width.max(1.))
            },
            if enabled {
                INK
            } else {
                Color::new(0.40, 0.43, 0.42, 1.)
            },
        );
        let hit = enabled && ((over && self.clicked) || self.activated == Some(id));
        if hit {
            self.sound(0);
            self.focus = None;
        }
        hit
    }
    fn draw(&mut self) {
        begin_scene();
        LANGUAGE.with(|l| l.set(self.save.language));
        draw_texture_ex(
            if self.screen == Screen::Worlds {
                &self.universe
            } else if matches!(self.screen, Screen::Map | Screen::Level) {
                &self.world_backgrounds[self.world_index.min(self.world_backgrounds.len() - 1)]
            } else {
                &self.world_backgrounds[0]
            },
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(W, H)),
                ..Default::default()
            },
        );
        // A light veil keeps the illustrated background subordinate to readable controls.
        if self.screen == Screen::Level {
            let veil = if self.world_index == 0 {
                Color::new(0.91, 0.97, 0.89, 0.68)
            } else {
                Color::new(0.98, 0.97, 1., 0.62)
            };
            draw_rectangle(0., 0., W, H, veil);
        }
        let t = if self.save.reduced_motion {
            0.
        } else {
            get_time() as f32
        };
        if self.screen == Screen::Map {
            for i in 0..3 {
                cloud(
                    ((t * 7. + i as f32 * 460.) % 1550.) - 130.,
                    110. + i as f32 * 25.,
                    0.65,
                );
            }
        }
        // Do not route pointer or keyboard activation to controls behind a modal.
        let blocked = self.settings || self.confirm_new;
        let saved_click = self.clicked;
        let saved_active = self.activated;
        if blocked {
            self.clicked = false;
            self.activated = None;
        }
        match self.screen {
            Screen::Main => self.main_menu(),
            Screen::Worlds => self.world_map(t),
            Screen::Map => self.map(t),
            Screen::Level => self.level(t),
        }
        if !blocked {
            if self.button(
                90,
                Rect::new(1010., 24., 110., 56.),
                if self.save.muted {
                    "Sound off"
                } else {
                    "Sound on"
                },
                CREAM,
                true,
            ) {
                self.save.muted = !self.save.muted;
                self.persist();
            }
            if self.button(
                91,
                Rect::new(1134., 24., 114., 56.),
                "Settings",
                CREAM,
                true,
            ) {
                self.settings = true;
                self.focus = None;
            }
        }
        self.clicked = saved_click;
        self.activated = saved_active;
        if self.settings {
            self.interactive.clear();
            self.options();
        } else if self.confirm_new {
            self.interactive.clear();
            self.new_game_dialog();
        }
        if get_time() < self.message_until && !self.message.is_empty() {
            let y = if self.screen == Screen::Map {
                249.
            } else {
                752.
            };
            rounded(Rect::new(290., y, 700., 36.), 14., INK);
            text_center(&self.message, 640., y + 25., 22., WHITE);
        }
        // The game never grabs the cursor. Repeated ClipCursor calls on hover
        // synchronize with Windows unnecessarily; leave the default cursor alone.
        flush_rendered();
    }
    fn world_range(&self) -> std::ops::Range<usize> {
        self.worlds[self.world_index].range(&self.levels).unwrap()
    }
    fn chess_tip(&self) -> &'static str {
        match self.game.level.id.as_str() {
            "chess-01" => "Protect your own king.",
            "chess-02" => "Start: one or two steps.",
            "chess-03" => "Castle: king to c1 or g1.",
            "chess-04" => "Try e5 to f6 this turn.",
            "chess-05" => "Reach a8. Pick a new piece.",
            _ => "Checkmate wins this match.",
        }
    }
    fn return_to_galaxy(&mut self) {
        self.completed_world = None;
        self.screen = Screen::Worlds;
        self.focus = None;
        self.persist();
    }
    fn level_node(&self, index: usize) -> (f32, f32) {
        campaign::node(index - self.world_range().start)
    }
    fn main_menu(&mut self) {
        rounded(Rect::new(310., 155., 660., 535.), 32., CREAM);
        text_center("ROOK RAINBOW QUEST", 640., 220., 36., INK);
        rook(vec2(640., 320.), 0.85, 0.);
        let profile = if self.save.active_profile == 0 {
            "Sunshine"
        } else {
            "Moonbeam"
        };
        text_center(profile, 640., 396., 25., INK);
        if self.button(
            201,
            Rect::new(435., 420., 410., 65.),
            "Continue",
            GOLD,
            self.save.has_adventure() && !self.settings && !self.confirm_new,
        ) {
            self.restore_map();
            self.screen = Screen::Worlds;
            self.focus = None;
        }
        if self.button(
            202,
            Rect::new(435., 505., 410., 65.),
            "New game",
            BLUE,
            !self.settings && !self.confirm_new,
        ) {
            self.confirm_new = true;
            self.focus = None;
        }
        if self.button(
            203,
            Rect::new(435., 590., 410., 65.),
            "Quit",
            PURPLE,
            !self.settings && !self.confirm_new,
        ) {
            self.shutdown.request();
        }
    }
    fn new_game_dialog(&mut self) {
        flush_rendered();
        draw_rectangle(0., 0., W, H, Color::new(0.12, 0.19, 0.24, 0.6));
        rounded(Rect::new(305., 220., 670., 350.), 30., CREAM);
        text_center("Start a new adventure?", 640., 280., 36., INK);
        text_center(
            "This player's previous progress will be lost.",
            640.,
            339.,
            25.,
            INK,
        );
        text_center("Other players and settings are kept.", 640., 380., 24., INK);
        if self.button(
            204,
            Rect::new(365., 450., 240., 65.),
            "Yes, new game",
            CORAL,
            true,
        ) {
            self.save.new_game(&self.levels[0].id, &self.worlds[0].id);
            self.confirm_new = false;
            self.restore_map();
            self.screen = Screen::Worlds;
            self.persist();
            self.focus = None;
        }
        if self.button(
            205,
            Rect::new(675., 450., 240., 65.),
            "No, go back",
            BLUE,
            true,
        ) {
            self.confirm_new = false;
            self.focus = None;
        }
    }
    fn navigate_world(&mut self, direction: Direction) {
        if self.settings || self.world_travel.is_some() {
            return;
        }
        if let Some(index) = worlds::neighbor(
            self.world_index,
            direction,
            &self.worlds,
            &self.levels,
            &self.save,
        ) {
            self.walk_world(index);
        }
    }
    fn walk_world(&mut self, index: usize) {
        if self.world_travel.is_some()
            || self.world_index.abs_diff(index) != 1
            || !worlds::unlocked(&self.worlds, &self.levels, &self.save, index)
        {
            return;
        }
        self.world_travel = Some(MapTravel {
            from: self.world_avatar,
            to: world_node(index),
            start: get_time(),
        });
        self.world_index = index;
        self.save.world_cursor[self.save.active_profile] = self.worlds[index].id.clone();
        self.focus = None;
        self.sound(0);
        self.persist();
    }
    fn enter_world(&mut self) {
        if self.world_travel.is_some()
            || !worlds::unlocked(&self.worlds, &self.levels, &self.save, self.world_index)
        {
            return;
        }
        self.save.world_cursor[self.save.active_profile] = self.worlds[self.world_index].id.clone();
        self.restore_map();
        self.screen = Screen::Map;
        self.focus = None;
        self.persist();
    }
    fn world_map(&mut self, t: f32) {
        for i in 0..self.worlds.len().saturating_sub(1) {
            let a = world_node(i) - vec2(self.world_camera, 0.);
            let b = world_node(i + 1) - vec2(self.world_camera, 0.);
            draw_line(a.x, a.y, b.x, b.y, 22., GOLD);
        }
        let mut clicked = None;
        for i in 0..self.worlds.len() {
            let p = world_node(i) - vec2(self.world_camera, 0.);
            if p.x < -130. || p.x > W + 130. {
                continue;
            }
            let open = worlds::unlocked(&self.worlds, &self.levels, &self.save, i);
            draw_ellipse(
                p.x,
                p.y + 28.,
                110.,
                47.,
                0.,
                Color::new(0.47, 0.32, 0.19, 0.5),
            );
            draw_ellipse(p.x, p.y, 110., 52., 0., if open { MINT } else { GRAY });
            if i == self.world_index {
                draw_ellipse_lines(p.x, p.y, 112., 54., 0., 5., GOLD);
            }
            if i == 2 {
                castle_icon(p + vec2(0., -8.), 1.3);
            } else if i == 1 {
                chess_icon("rook", p + vec2(-45., 0.), 0.5, Side::Player);
                chess_icon("king", p + vec2(0., -7.), 0.7, Side::Opponent);
                chess_icon("queen", p + vec2(47., 0.), 0.5, Side::Player);
            } else {
                for j in 0..5 {
                    chess_icon(
                        ["rook", "bishop", "knight", "pawn", "queen"][j],
                        p + vec2((j as f32 - 2.) * 34., 0.),
                        0.46,
                        Side::Player,
                    );
                }
            }
            if !open {
                lock(p.x + 94., p.y - 20.);
            }
            circle(p.x, p.y + 70., 19., CREAM);
            text_center(&format!("{}", i + 1), p.x, p.y + 78., 26., INK);
            if self.clicked && !self.settings && self.pointer.distance(p) < 115. {
                clicked = Some(i);
            }
        }
        if let Some(i) = clicked {
            if i == self.world_index {
                self.enter_world();
            } else {
                self.walk_world(i);
            }
        }
        let hop = if self.world_travel.is_some() && !self.save.reduced_motion {
            (t * 18.).sin().abs() * 8.
        } else {
            0.
        };
        rook(
            self.world_avatar - vec2(self.world_camera, 112. + hop),
            1.02,
            0.,
        );
        rounded(Rect::new(65., 45., 650., 188.), 28., CREAM);
        text("ADVENTURE GALAXY", 95., 103., 38., INK);
        text("Choose your next adventure", 95., 152., 29., CORAL);
        text(
            "Arrow keys: follow the path   |   Enter: play",
            95.,
            198.,
            23.,
            INK,
        );
        rounded(Rect::new(180., 622., 920., 111.), 26., CREAM);
        text_center(&self.worlds[self.world_index].title, 640., 666., 34., INK);
        let range = self.world_range();
        let count = self.levels[range.clone()]
            .iter()
            .filter(|l| self.save.completed(&l.id))
            .count();
        text_center(
            &format!("{} of {}", count, range.len()),
            640.,
            708.,
            25.,
            INK,
        );
        if self.button(
            211,
            Rect::new(65., 642., 100., 65.),
            "<",
            BLUE,
            !self.settings && self.world_index > 0 && self.world_travel.is_none(),
        ) {
            self.navigate_world(Direction::Left);
        }
        if self.button(
            212,
            Rect::new(1115., 642., 100., 65.),
            ">",
            BLUE,
            !self.settings
                && worlds::unlocked(&self.worlds, &self.levels, &self.save, self.world_index + 1)
                && self.world_travel.is_none(),
        ) {
            self.navigate_world(Direction::Right);
        }
        if self.button(
            200,
            Rect::new(480., 737., 320., 58.),
            "Enter world",
            CORAL,
            !self.settings && self.world_travel.is_none(),
        ) {
            self.enter_world();
        }
        if self.button(
            5,
            Rect::new(65., 737., 250., 58.),
            "Main menu",
            CREAM,
            !self.settings,
        ) {
            self.screen = Screen::Main;
            self.focus = None;
            self.persist();
        }
        if self.worlds.len() == 1 {
            rounded(Rect::new(365., 538., 550., 46.), 16., CREAM);
            text_center("More worlds will grow here!", 640., 570., 26., INK);
        }
    }
    fn map(&mut self, t: f32) {
        // All level stops live in world coordinates. Only nearby stops are drawn.
        for i in self.world_range().start..self.world_range().end.saturating_sub(1) {
            let (ax, ay) = self.level_node(i);
            let (bx, by) = self.level_node(i + 1);
            let a = vec2(ax - self.map_camera, ay);
            let b = vec2(bx - self.map_camera, by);
            if b.x < -80. || a.x > W + 80. {
                continue;
            }
            draw_line(
                a.x,
                a.y + 5.,
                b.x,
                b.y + 5.,
                27.,
                Color::new(0.55, 0.40, 0.23, 0.35),
            );
            draw_line(a.x, a.y, b.x, b.y, 20., Color::new(1., 0.88, 0.59, 1.));
        }
        let mut clicked_node = None;
        for i in self.world_range() {
            let (wx, y) = self.level_node(i);
            let x = wx - self.map_camera;
            if !(-80. ..W + 80.).contains(&x) {
                continue;
            }
            let open = campaign::unlocked(&self.levels, &self.save, i);
            let completed = self.save.completed(&self.levels[i].id);
            let selected = i == self.map_index;
            draw_ellipse(x, y + 14., 49., 15., 0., Color::new(0.26, 0.33, 0.20, 0.25));
            circle(
                x,
                y,
                if selected {
                    46. + if self.save.reduced_motion {
                        0.
                    } else {
                        t.sin() * 2.
                    }
                } else {
                    41.
                },
                if selected {
                    GOLD
                } else if open {
                    MINT
                } else {
                    Color::new(0.73, 0.75, 0.72, 1.)
                },
            );
            circle(x, y, 34., CREAM);
            let icon = &self.levels[i].icon;
            if icon.castle {
                castle_icon(vec2(x, y - 4.), 0.72);
            } else if let Some(opponent) = &icon.opponent {
                chess_icon(&icon.piece, vec2(x - 13., y - 2.), 0.44, Side::Player);
                chess_icon(opponent, vec2(x + 14., y - 2.), 0.44, Side::Opponent);
            } else {
                chess_icon(&icon.piece, vec2(x, y - 2.), 0.59, Side::Player);
                for j in 0..icon.badge {
                    star(vec2(x + 26., y - 12. + j as f32 * 11.), 5., GOLD);
                }
            }
            rounded(Rect::new(x - 18., y + 30., 36., 24.), 9., CREAM);
            text_center(
                &format!("{}", i - self.world_range().start + 1),
                x,
                y + 48.,
                18.,
                INK,
            );
            if !open {
                lock(x + 35., y + 18.);
            }
            if completed {
                star(vec2(x, y + 69.), 14., GOLD);
            }
            if self.levels[i].final_boss {
                rounded(Rect::new(x - 65., y + 76., 130., 29.), 10., CREAM);
                text_center("FINAL BOSS", x, y + 97., 18., CORAL);
            }
            if self.clicked && !self.settings && self.pointer.distance(vec2(x, y)) < 46. {
                clicked_node = Some(i);
            }
            if i % 5 == 0 {
                draw_line(x, y - 52., x, y - 110., 5., INK);
                let wave = if self.save.reduced_motion {
                    0.
                } else {
                    (t * 2.).sin() * 3.
                };
                draw_triangle(
                    vec2(x + 2., y - 110.),
                    vec2(x + 54., y - 98. + wave),
                    vec2(x + 2., y - 76.),
                    [CORAL, BLUE, PURPLE, GOLD][(i / 5) % 4],
                );
            }
        }
        if let Some(i) = clicked_node {
            if i == self.map_index {
                self.start();
            } else if self.map_index.abs_diff(i) == 1 {
                self.walk_to(i);
            } else {
                self.note("Follow the path with the arrow keys", 2.);
            }
        }
        let hop = if self.map_travel.is_some() && !self.save.reduced_motion {
            (t * 18.).sin().abs() * 8.
        } else {
            0.
        };
        rook(
            vec2(
                self.map_avatar.x - self.map_camera,
                self.map_avatar.y - 81. - hop,
            ),
            1.02,
            0.,
        );
        // Fixed HUD stays readable while the path moves underneath it.
        rounded(Rect::new(65., 45., 650., 188.), 28., CREAM);
        text("ROOK RAINBOW QUEST", 93., 100., 36., INK);
        text(&self.levels[self.map_index].world, 95., 150., 35., CORAL);
        text(
            "Arrow keys: follow the path   |   Enter: play",
            95.,
            198.,
            23.,
            INK,
        );
        let level = &self.levels[self.map_index];
        let number = if level.is_standard() {
            self.map_index - self.world_range().start + 1
        } else {
            self.map_index + 1
        };
        let title = format!("{}. {}", number, level.title);
        let count = self.levels[self.world_range()]
            .iter()
            .filter(|l| self.save.completed(&l.id))
            .count();
        let play_label = if self.save.completed(&level.id) {
            "Play again"
        } else {
            "Let's play!"
        };
        rounded(Rect::new(180., 622., 920., 111.), 26., CREAM);
        text_center(&title, 640., 663., 31., INK);
        text_center(
            &format!(
                "Level {} of {}   /   {} stars   /   {}",
                self.map_index - self.world_range().start + 1,
                self.world_range().len(),
                count,
                if self.save.active_profile == 0 {
                    "Sunshine"
                } else {
                    "Moonbeam"
                }
            ),
            640.,
            706.,
            22.,
            INK,
        );
        if self.button(
            2,
            Rect::new(65., 642., 100., 65.),
            "<",
            BLUE,
            !self.settings
                && self.map_index > self.world_range().start
                && self.map_travel.is_none(),
        ) {
            self.navigate(Direction::Left);
        }
        let next_open = self.map_index + 1 < self.world_range().end
            && campaign::unlocked(&self.levels, &self.save, self.map_index + 1);
        if self.button(
            3,
            Rect::new(1115., 642., 100., 65.),
            ">",
            BLUE,
            !self.settings && next_open && self.map_travel.is_none(),
        ) {
            self.navigate(Direction::Right);
        }
        if self.button(
            4,
            Rect::new(745., 24., 180., 56.),
            "Galaxy",
            CREAM,
            !self.settings,
        ) {
            self.screen = Screen::Worlds;
            self.focus = None;
        }
        if self.button(
            1,
            Rect::new(480., 737., 320., 58.),
            play_label,
            CORAL,
            !self.settings && self.map_travel.is_none(),
        ) {
            self.start();
        }
    }
    fn level(&mut self, t: f32) {
        rounded(Rect::new(275., 23., 710., 119.), 25., CREAM);
        let title_size = 38_f32.min(38. * 660. / text_width(&self.game.level.title, 38.).max(1.));
        text_center(&self.game.level.title, 640., 70., title_size, INK);
        let instruction = if self.game.level.final_boss {
            "FINAL BOSS — full game!".to_owned()
        } else if self.game.level.is_royal() {
            "Checkmate the king!".into()
        } else if self.game.level.pawn_promotion && !self.game.level.is_match() {
            "Reach the far edge to become a queen!".into()
        } else if self.game.level.is_match() {
            "Capture their pieces!".into()
        } else if self.game.level.targets.len() == 1 {
            "Capture the pawn!".into()
        } else {
            format!("Capture all {} pawns!", self.game.level.targets.len())
        };
        text_center(&instruction, 640., 115., 28., INK);
        let board = self.game.level.board.clone();
        let cell = (480. / board.width.max(board.height) as f32).min(120.);
        let (bw, bh) = (cell * board.width as f32, cell * board.height as f32);
        let (bx, by) = (640. - bw / 2., 180.);
        rounded(
            Rect::new(bx - 12., by - 12., bw + 24., bh + 24.),
            18.,
            Color::new(0.72, 0.49, 0.28, 1.),
        );
        let units = self.game.units();
        for y in 0..board.height {
            for x in 0..board.width {
                let square = Square { x, y };
                let c = center(square, &self.game.level);
                draw_rectangle(
                    c.x - cell / 2.,
                    c.y - cell / 2.,
                    cell,
                    cell,
                    if (x + y) % 2 == 0 { CREAM } else { MINT },
                );
                if self.selected
                    && self.game.level.show_legal_moves
                    && self.motion.is_none()
                    && self.game.legal(square)
                {
                    if units
                        .iter()
                        .any(|u| u.square == square && u.side == Side::Opponent)
                    {
                        ring(c.x, c.y, cell * 0.39, 4., Color::new(0.23, 0.59, 0.40, 1.));
                    } else {
                        circle(c.x, c.y, 7., Color::new(0.23, 0.59, 0.40, 0.6));
                    }
                }
                if self
                    .hint
                    .is_some_and(|(m, _)| m.to == square || m.from == square)
                {
                    draw_rectangle_lines(
                        c.x - cell / 2. + 5.,
                        c.y - cell / 2. + 5.,
                        cell - 10.,
                        cell - 10.,
                        5.,
                        GOLD,
                    );
                }
            }
        }
        for unit in &units {
            if self
                .motion
                .as_ref()
                .is_some_and(|m| m.to == unit.square && m.side == unit.side)
            {
                continue;
            }
            let c = center(unit.square, &self.game.level);
            if self.selected && self.game.selected == Some(unit.square) && unit.side == Side::Player
            {
                ring(c.x, c.y, cell * 0.41, 4., GOLD);
            }
            if unit.kind == "king"
                && self
                    .game
                    .battle
                    .as_ref()
                    .is_some_and(|b| b.king_in_check(unit.side))
            {
                ring(c.x, c.y, cell * 0.44, 5., CORAL);
            }
            chess_icon(&unit.kind, c, cell / 128., unit.side);
        }
        if let Some(m) = &self.motion {
            let duration = if self.save.reduced_motion { 0.08 } else { 0.24 };
            let u = ((get_time() - m.start) / duration).clamp(0., 1.) as f32;
            let mut c = center(m.from, &self.game.level)
                .lerp(center(m.to, &self.game.level), u * u * (3. - 2. * u));
            if !self.save.reduced_motion {
                c.y -= (u * std::f32::consts::PI).sin() * 8.;
            }
            chess_icon(&m.kind, c, cell / 128., m.side);
        }
        for p in &self.particles {
            star(p.p, 5., p.color);
        }
        rounded(Rect::new(65., 235., 270., 260.), 26., CREAM);
        let kind = self
            .game
            .selected
            .and_then(|q| {
                units
                    .iter()
                    .find(|u| u.square == q && u.side == Side::Player)
            })
            .map(|u| u.kind.as_str())
            .unwrap_or(self.game.level.player.piece.as_str());
        let (first, second) = match kind {
            "bishop" => ("Bishops move", "diagonally."),
            "knight" => ("Knights hop", "in an L."),
            "pawn" => ("Walk forward.", "Capture diagonally."),
            "queen" => ("Queens move", "straight or diagonally."),
            "king" => ("Kings move", "one square, safely."),
            _ => ("Rooks move", "straight."),
        };
        if self.game.level.is_match() && self.game.selected.is_none() {
            text_center("YOUR TEAM", 200., 278., 23., INK);
            text_center(
                if self.game.player_turn() {
                    "Choose a piece"
                } else {
                    "Wait for your turn"
                },
                200.,
                322.,
                23.,
                INK,
            );
            text_center(
                if self.game.player_turn() {
                    "Then choose a square"
                } else {
                    "Watch the purple team"
                },
                200.,
                354.,
                20.,
                INK,
            );
            for (j, u) in units
                .iter()
                .filter(|u| u.side == Side::Player)
                .take(4)
                .enumerate()
            {
                chess_icon(
                    &u.kind,
                    vec2(125. + j as f32 * 50., 410.),
                    0.5,
                    Side::Player,
                );
            }
        } else {
            text_center(
                &format!("{}'S RULE", kind.to_uppercase()),
                200.,
                278.,
                23.,
                INK,
            );
            text_center(first, 200., 322., 25., INK);
            text_center(second, 200., 354., 22., INK);
            chess_icon(kind, vec2(200., 404.), 0.58, Side::Player);
            text_center(
                if self.game.level.is_standard() {
                    self.chess_tip()
                } else if kind == "pawn" && self.game.level.pawn_promotion {
                    "Far edge: a new queen!"
                } else if self.game.level.is_royal() {
                    "The king stays on the board."
                } else if self.game.level.is_match() {
                    "Choose one of your pieces"
                } else {
                    "Click a glowing square"
                },
                200.,
                465.,
                18.,
                INK,
            );
        }
        if self.game.level.is_royal() && self.game.selected.is_none() {
            text_center(
                if self.game.level.is_standard() {
                    self.chess_tip()
                } else {
                    "Check and no escape wins."
                },
                200.,
                465.,
                18.,
                INK,
            );
        }
        rounded(Rect::new(945., 235., 270., 260.), 26., CREAM);
        text_center(
            if self.game.level.is_standard() {
                "CHESS MATCH"
            } else if self.game.level.is_checkmate() {
                "CHECKMATE"
            } else if self.game.level.is_match() {
                "FRIENDLY MATCH"
            } else {
                "COLLECT STARS"
            },
            1080.,
            278.,
            22.,
            INK,
        );
        if self.game.level.is_royal() {
            chess_icon("king", vec2(1080., 340.), 0.7, Side::Opponent);
            text_center(
                if self.game.battle.as_ref().is_some_and(|b| b.in_check()) {
                    "Check!"
                } else {
                    "Cover every escape"
                },
                1080.,
                398.,
                23.,
                INK,
            );
        } else {
            let captured = self.game.level.targets.len() - self.game.remaining();
            let total = self.game.level.targets.len();
            for i in 0..total {
                let spacing = 210. / total.max(3) as f32;
                star(
                    vec2(1080. + (i as f32 - (total - 1) as f32 / 2.) * spacing, 339.),
                    24.,
                    if i < captured {
                        GOLD
                    } else {
                        Color::new(0.83, 0.83, 0.76, 1.)
                    },
                );
            }
            text_center(&format!("{} of {}", captured, total), 1080., 398., 27., INK);
        }
        let turn = if !self.game.level.is_match() {
            "Take your time!"
        } else if self.game.player_turn() {
            "Your turn"
        } else {
            "Opponent's turn"
        };
        text_center(turn, 1080., 446., 24., INK);
        if self.game.level.is_match() {
            let count = units.iter().filter(|u| u.side == Side::Player).count();
            text_center(&format!("Your team: {count}"), 1080., 480., 20., INK);
        }
        let active = !self.settings
            && self.won_at.is_none()
            && self.failed_at.is_none()
            && self.promotion_to.is_none();
        if self.won_at.is_none() && self.failed_at.is_none() {
            if self.button(
                10,
                Rect::new(370., 692., 160., 56.),
                "Hint",
                GOLD,
                active && self.motion.is_none() && self.game.player_turn(),
            ) {
                if let Some(m) = self.game.hint() {
                    if !self.game.level.is_match() {
                        self.game.select(m.from);
                        self.selected = true;
                    }
                    self.hint = Some((m, get_time() + 5.));
                    self.sound(1);
                }
            }
            if self.button(
                12,
                Rect::new(560., 692., 160., 56.),
                "Try again",
                CORAL,
                active,
            ) {
                self.restart();
            }
            if self.button(13, Rect::new(750., 692., 160., 56.), "Map", BLUE, active) {
                self.screen = Screen::Map;
                self.focus = None;
            }
        }
        if self.clicked
            && active
            && self.motion.is_none()
            && self.game.player_turn()
            && Rect::new(bx, by, bw, bh).contains(self.pointer)
        {
            let square = Square {
                x: ((self.pointer.x - bx) / cell) as u8,
                y: board.height - 1 - ((self.pointer.y - by) / cell) as u8,
            };
            if self.game.select(square) {
                self.selected = true;
                self.hint = None;
                self.sound(1);
            } else if self.selected {
                self.move_piece(square);
            }
        }
        if self.won_at.is_some() {
            self.victory(t);
        } else if self.failed_at.is_some() {
            self.failure();
        } else if let Some(to) = self.promotion_to {
            self.promotion_dialog(to);
        }
    }
    fn promotion_dialog(&mut self, to: Square) {
        flush_rendered();
        draw_rectangle(0., 0., W, H, Color::new(0.12, 0.19, 0.24, 0.10));
        rounded(
            Rect::new(280., 220., 720., 350.),
            30.,
            Color { a: 0.82, ..CREAM },
        );
        text_center("Choose your new piece", 640., 285., 35., INK);
        self.interactive.clear();
        for (i, (piece, label)) in [
            ("queen", "Queen"),
            ("rook", "Rook"),
            ("bishop", "Bishop"),
            ("knight", "Knight"),
        ]
        .iter()
        .enumerate()
        {
            let x = 310. + i as f32 * 170.;
            chess_icon(piece, vec2(x + 75., 365.), 0.6, Side::Player);
            if self.button(
                120 + i,
                Rect::new(x, 420., 150., 65.),
                label,
                if i == 0 { GOLD } else { BLUE },
                !self.settings,
            ) {
                match self.game.play_promoted(to, piece) {
                    Ok(action) => {
                        self.promotion_to = None;
                        self.animate(action);
                    }
                    Err(_) => {
                        self.promotion_to = None;
                    }
                }
            }
        }
        text_center("Escape: choose another move", 640., 535., 22., INK);
    }
    fn failure(&mut self) {
        flush_rendered();
        result_backdrop();
        let draw = self.game.battle.as_ref().is_some_and(|b| {
            b.outcome == rook_rainbow_quest::battle::Outcome::Stuck
                && (b.standard.is_some() || b.draw_reason.is_some())
        });
        let stalemate = self.game.level.is_checkmate()
            && self.game.battle.as_ref().is_some_and(|b| {
                b.outcome == rook_rainbow_quest::battle::Outcome::Stuck && b.turn == Side::Opponent
            });
        text(
            if draw {
                "Draw — try again!"
            } else if self.game.level.is_standard() {
                "Your king is checkmated"
            } else if stalemate {
                "Stalemate — try again!"
            } else {
                "Good try!"
            },
            74.,
            716.,
            31.,
            INK,
        );
        let reason = if draw {
            let reason = self
                .game
                .battle
                .as_ref()
                .and_then(|b| {
                    b.draw_reason
                        .or_else(|| b.standard.as_ref().and_then(|s| s.draw_reason))
                })
                .unwrap_or("Start this level again.");
            if reason == "Stalemate" {
                STALEMATE_EXPLANATION
            } else {
                reason
            }
        } else if stalemate {
            STALEMATE_EXPLANATION
        } else {
            "Start this level again."
        };
        result_detail(&format!(
            "{}  {}",
            localized(reason),
            localized("Your stars are safe.")
        ));
        self.interactive.clear();
        if self.button(
            12,
            Rect::new(735., 705., 225., 60.),
            "Try again",
            CORAL,
            !self.settings,
        ) {
            self.restart();
        }
        if self.button(
            13,
            Rect::new(990., 705., 225., 60.),
            "Back to map",
            BLUE,
            !self.settings,
        ) {
            self.screen = Screen::Map;
            self.focus = None;
        }
    }
    fn victory(&mut self, t: f32) {
        if let Some(world) = self.completed_world {
            self.world_victory(world, t);
            return;
        }
        flush_rendered();
        result_backdrop();
        let bounce = if self.save.reduced_motion {
            0.
        } else {
            (t * 3.).sin() * 5.
        };
        star(vec2(600., 716. + bounce), 24., GOLD);
        text(
            if self.game.level.is_royal() {
                "Checkmate!"
            } else {
                "You did it!"
            },
            74.,
            716.,
            31.,
            INK,
        );
        result_detail(if self.map_index + 1 < self.world_range().end {
            "A new stop is ready on your path!"
        } else {
            "You finished this world!"
        });
        self.interactive.clear();
        let enabled = !self.settings;
        if self.button(
            30,
            Rect::new(645., 705., 180., 60.),
            "Play again",
            CORAL,
            enabled,
        ) {
            self.restart();
        }
        if self.button(
            31,
            Rect::new(840., 705., 180., 60.),
            "Back to map",
            BLUE,
            enabled,
        ) {
            self.screen = Screen::Map;
            self.focus = None;
        }
        if self.button(
            32,
            Rect::new(1035., 705., 180., 60.),
            "Next level",
            GOLD,
            enabled && self.map_index + 1 < self.world_range().end,
        ) {
            self.screen = Screen::Map;
            self.walk_to(self.map_index + 1);
        }
        for p in &self.particles {
            star(p.p, 6., p.color);
        }
    }
    fn world_victory(&mut self, world: usize, t: f32) {
        flush_rendered();
        result_backdrop();
        let bounce = if self.save.reduced_motion {
            0.
        } else {
            (t * 3.).sin() * 5.
        };
        star(vec2(600., 716. + bounce), 24., GOLD);
        text("World complete!", 74., 716., 31., INK);
        result_detail(&format!(
            "{} {}",
            localized("You completed"),
            localized(&self.worlds[world].title)
        ));
        self.interactive.clear();
        if self.button(
            33,
            Rect::new(655., 705., 560., 60.),
            "Back to Adventure Galaxy",
            BLUE,
            !self.settings,
        ) {
            self.return_to_galaxy();
        }
        for p in &self.particles {
            star(p.p, 6., p.color);
        }
    }
    fn options(&mut self) {
        flush_rendered();
        draw_rectangle(0., 0., W, H, Color::new(0.12, 0.19, 0.24, 0.6));
        rounded(Rect::new(300., 95., 680., 665.), 30., CREAM);
        text_center("Make yourself at home", 640., 184., 35., INK);
        text("YOUR PLAYER", 340., 236., 19., INK);
        for i in 0..2 {
            let label = if i == 0 { "Sunshine" } else { "Moonbeam" };
            let r = Rect::new(340. + i as f32 * 315., 254., 280., 60.);
            if self.save.active_profile == i {
                rounded(Rect::new(r.x - 4., r.y - 4., r.w + 8., r.h + 8.), 21., GOLD);
            }
            if self.button(100 + i, r, label, if i == 0 { GOLD } else { PURPLE }, true)
                && self.save.active_profile != i
            {
                self.save.active_profile = i;
                self.restore_map();
                self.screen = Screen::Main;
                self.persist();
            }
        }
        if self.button(
            102,
            Rect::new(340., 340., 280., 58.),
            if self.save.muted {
                "Sound: off"
            } else {
                "Sound: on"
            },
            BLUE,
            true,
        ) {
            self.save.muted = !self.save.muted;
            self.persist();
        }
        if self.button(
            103,
            Rect::new(655., 340., 280., 58.),
            if self.save.reduced_motion {
                "Move animation: Off"
            } else {
                "Move animation: On"
            },
            MINT,
            true,
        ) {
            self.save.reduced_motion = !self.save.reduced_motion;
            self.particles.clear();
            self.persist();
        }
        text(
            &format!("Volume: {}%", (self.save.volume * 100.).round() as u8),
            345.,
            441.,
            24.,
            INK,
        );
        if self.button(
            104,
            Rect::new(545., 411., 110., 56.),
            "Softer",
            PURPLE,
            true,
        ) {
            self.save.volume = (self.save.volume - 0.1).max(0.);
            self.persist();
            self.sound(1);
        }
        if self.button(105, Rect::new(670., 411., 110., 56.), "Louder", GOLD, true) {
            self.save.volume = (self.save.volume + 0.1).min(1.);
            self.persist();
            self.sound(1);
        }
        if self.button(
            106,
            Rect::new(795., 411., 140., 56.),
            "Fullscreen",
            BLUE,
            true,
        ) {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        text("Language", 345., 513., 22., INK);
        for (id, label, language, x, width) in [
            (108, "Svenska", Language::Swedish, 545., 165.),
            (109, "English", Language::English, 730., 205.),
        ] {
            if self.button(
                id,
                Rect::new(x, 482., width, 56.),
                label,
                if self.save.language == language {
                    GOLD
                } else {
                    MINT
                },
                true,
            ) {
                self.save.language = language;
                LANGUAGE.with(|l| l.set(language));
                self.persist();
            }
        }
        let unlocked = self.save.all_levels_unlocked[self.save.active_profile];
        if self.button(
            110,
            Rect::new(340., 548., 595., 48.),
            if unlocked {
                "All levels unlocked"
            } else {
                "Unlock all levels"
            },
            MINT,
            !unlocked,
        ) {
            self.save.unlock_all_levels();
            self.persist();
            self.note("All levels are now unlocked.", 3.);
        }
        text_center(
            "M: sound   |   F11: fullscreen   |   Tab + Enter: buttons",
            640.,
            624.,
            17.,
            INK,
        );
        if !self.mute_audio_available {
            text_center(
                "Audio unavailable - silent play is ready.",
                640.,
                646.,
                17.,
                INK,
            );
        }
        if self.button(
            99,
            Rect::new(340., 665., 400., 62.),
            "Back to adventure",
            CORAL,
            true,
        ) {
            self.settings = false;
            self.focus = None;
        }
        if self.button(
            107,
            Rect::new(770., 665., 165., 62.),
            "Exit game",
            PURPLE,
            true,
        ) {
            self.shutdown.request();
        }
    }
}

fn circle(x: f32, y: f32, r: f32, color: Color) {
    draw_poly(
        x,
        y,
        curve_segments(r, RENDER_SCALE.with(std::cell::Cell::get)),
        r,
        0.,
        color,
    );
}
fn ring(x: f32, y: f32, r: f32, width: f32, color: Color) {
    draw_poly_lines(
        x,
        y,
        curve_segments(r, RENDER_SCALE.with(std::cell::Cell::get)),
        r,
        0.,
        width,
        color,
    );
}
fn chess_icon(kind: &str, p: Vec2, s: f32, side: Side) {
    if RAW_ART.with(std::cell::Cell::get) {
        chess_icon_raw(kind, p, s, side);
        return;
    }
    let drawn = PIECE_SPRITES.with(|cache| {
        if let Some((texture, source)) = cache.get().and_then(|c| c.get(&(kind.to_owned(), side))) {
            SPRITE_QUEUE.with(|q| {
                q.borrow_mut().push((
                    texture.clone(),
                    *source,
                    p - vec2(64., 64.) * s,
                    vec2(128., 128.) * s,
                ))
            });
            true
        } else {
            false
        }
    });
    if !drawn {
        chess_icon_raw(kind, p, s, side);
    }
}

fn chess_icon_raw(kind: &str, p: Vec2, s: f32, side: Side) {
    let fill = if side == Side::Player { CREAM } else { PURPLE };
    let edge = if side == Side::Player {
        Color::new(0.55, 0.40, 0.26, 1.)
    } else {
        Color::new(0.39, 0.25, 0.58, 1.)
    };
    let rect = |x: f32, y: f32, w: f32, h: f32| Rect::new(p.x + x * s, p.y + y * s, w * s, h * s);
    draw_ellipse(
        p.x,
        p.y + 39. * s,
        32. * s,
        8. * s,
        0.,
        Color::new(0.2, 0.22, 0.24, 0.16),
    );
    rounded(rect(-30., 26., 60., 14.), 5. * s, edge);
    rounded(rect(-27., 26., 54., 10.), 4. * s, fill);
    if kind == "rook" {
        rounded(rect(-23., -30., 46., 61.), 6. * s, edge);
        rounded(rect(-20., -28., 40., 56.), 5. * s, fill);
        rounded(rect(-31., -39., 62., 20.), 4. * s, edge);
        rounded(rect(-28., -37., 56., 15.), 3. * s, fill);
        for x in [-29., -8., 13.] {
            rounded(rect(x, -49., 16., 22.), 3. * s, edge);
            rounded(rect(x + 2., -47., 12., 20.), 2. * s, fill);
        }
    } else {
        draw_triangle(
            p + vec2(-23., 29.) * s,
            p + vec2(23., 29.) * s,
            p + vec2(0., -26.) * s,
            edge,
        );
        draw_triangle(
            p + vec2(-18., 27.) * s,
            p + vec2(18., 27.) * s,
            p + vec2(0., -23.) * s,
            fill,
        );
        match kind {
            "bishop" => {
                draw_ellipse(p.x, p.y - 21. * s, 21. * s, 29. * s, 0., edge);
                draw_ellipse(p.x, p.y - 22. * s, 18. * s, 26. * s, 0., fill);
                draw_line(
                    p.x + 4. * s,
                    p.y - 44. * s,
                    p.x - 5. * s,
                    p.y - 27. * s,
                    3. * s,
                    edge,
                );
                circle(p.x, p.y - 53. * s, 5. * s, fill);
            }
            "knight" => {
                rounded(rect(-23., -35., 42., 41.), 9. * s, edge);
                rounded(rect(-19., -32., 35., 35.), 7. * s, fill);
                rounded(rect(3., -30., 32., 22.), 6. * s, edge);
                rounded(rect(4., -28., 28., 17.), 5. * s, fill);
                draw_triangle(
                    p + vec2(-20., -30.) * s,
                    p + vec2(-15., -55.) * s,
                    p + vec2(0., -32.) * s,
                    edge,
                );
                draw_triangle(
                    p + vec2(-15., -32.) * s,
                    p + vec2(-13., -47.) * s,
                    p + vec2(-5., -32.) * s,
                    fill,
                );
                circle(p.x + 4. * s, p.y - 23. * s, 3. * s, INK);
            }
            "king" => {
                rounded(rect(-22., -31., 44., 35.), 6. * s, edge);
                rounded(rect(-18., -28., 36., 29.), 5. * s, fill);
                rounded(rect(-6., -61., 12., 36.), 2. * s, edge);
                rounded(rect(-17., -51., 34., 11.), 2. * s, edge);
                rounded(rect(-3., -58., 6., 30.), 1. * s, GOLD);
                rounded(rect(-14., -48., 28., 5.), 1. * s, GOLD);
            }
            "queen" => {
                rounded(rect(-23., -23., 46., 24.), 5. * s, edge);
                rounded(rect(-20., -21., 40., 19.), 4. * s, fill);
                for (x, y) in [(-22., -48.), (0., -57.), (22., -48.)] {
                    draw_triangle(
                        p + vec2(x, y) * s,
                        p + vec2(x - 10., -18.) * s,
                        p + vec2(x + 10., -18.) * s,
                        edge,
                    );
                    draw_triangle(
                        p + vec2(x, y + 5.) * s,
                        p + vec2(x - 7., -20.) * s,
                        p + vec2(x + 7., -20.) * s,
                        fill,
                    );
                    circle(p.x + x * s, p.y + y * s, 5. * s, GOLD);
                }
            }
            _ => {
                circle(p.x, p.y - 23. * s, 21. * s, edge);
                circle(p.x, p.y - 24. * s, 18. * s, fill);
            }
        }
    }
    if kind != "knight" {
        let ey = if kind == "rook" {
            -5.
        } else if kind == "pawn" {
            -21.
        } else {
            -11.
        };
        for x in [-6., 6.] {
            circle(p.x + x * s, p.y + ey * s, 2.3 * s, INK);
        }
        draw_line(
            p.x - 4. * s,
            p.y + (ey + 8.) * s,
            p.x + 4. * s,
            p.y + (ey + 8.) * s,
            1.7 * s,
            INK,
        );
    }
    rounded(
        rect(-22., 17., 44., 5.),
        2. * s,
        if side == Side::Player {
            MINT
        } else {
            Color::new(0.51, 0.35, 0.72, 1.)
        },
    );
}

fn center(s: Square, l: &Level) -> Vec2 {
    let cell = (480. / l.board.width.max(l.board.height) as f32).min(120.);
    vec2(
        640. - cell * l.board.width as f32 / 2. + (s.x as f32 + 0.5) * cell,
        180. + (l.board.height as f32 - s.y as f32 - 0.5) * cell,
    )
}
// Keep result controls below even the largest board, so children can study the final position.
const STALEMATE_EXPLANATION: &str =
    "The king is not in check, but the opponent has no legal moves.";
const RESULT_PANEL: Rect = Rect::new(36., 680., 1208., 106.);
fn result_backdrop() {
    draw_rectangle(0., 0., W, H, Color::new(0.12, 0.19, 0.24, 0.08));
    rounded(RESULT_PANEL, 26., Color { a: 0.86, ..CREAM });
}
fn result_detail(message: &str) {
    let message = localized(message);
    let mut line = String::new();
    let mut y = 746.;
    for word in message.split_whitespace() {
        let next = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if text_width(&next, 22.) > 550. && !line.is_empty() {
            text_raw(&line, 74., y, 22., INK);
            y += 25.;
            line.clear();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    text_raw(&line, 74., y, 22., INK);
}

fn rounded(r: Rect, radius: f32, c: Color) {
    let z = radius.min(r.w / 2.).min(r.h / 2.).max(0.);
    let steps = (curve_segments(z, RENDER_SCALE.with(std::cell::Cell::get)) as usize).div_ceil(4);
    let mut vertices = Vec::with_capacity(4 * (steps + 1) + 1);
    let mut indices = Vec::with_capacity(12 * (steps + 1));
    vertices.push(Vertex::new(r.x + r.w / 2., r.y + r.h / 2., 0., 0., 0., c));
    for (x, y, start) in [
        (r.x + z, r.y + z, std::f32::consts::PI),
        (r.x + r.w - z, r.y + z, std::f32::consts::PI * 1.5),
        (r.x + r.w - z, r.y + r.h - z, 0.),
        (r.x + z, r.y + r.h - z, std::f32::consts::FRAC_PI_2),
    ] {
        for i in 0..=steps {
            let a = start + i as f32 * std::f32::consts::FRAC_PI_2 / steps as f32;
            vertices.push(Vertex::new(x + z * a.cos(), y + z * a.sin(), 0., 0., 0., c));
        }
    }
    for i in 1..vertices.len() {
        indices.extend_from_slice(&[
            0,
            i as u16,
            if i + 1 < vertices.len() {
                i as u16 + 1
            } else {
                1
            },
        ]);
    }
    draw_mesh(&Mesh {
        vertices,
        indices,
        texture: None,
    });
}

fn castle_icon(p: Vec2, s: f32) {
    rounded(
        Rect::new(p.x - 36. * s, p.y - 10. * s, 72. * s, 42. * s),
        5. * s,
        CORAL,
    );
    for x in [-30., 30.] {
        rounded(
            Rect::new(p.x + (x - 11.) * s, p.y - 29. * s, 22. * s, 61. * s),
            3. * s,
            CREAM,
        );
        for dx in [-8., 0., 8.] {
            draw_rectangle(
                p.x + (x + dx - 3.) * s,
                p.y - 36. * s,
                6. * s,
                12. * s,
                CREAM,
            );
        }
        draw_rectangle(p.x + (x - 3.) * s, p.y - 12. * s, 6. * s, 13. * s, BLUE);
    }
    rounded(
        Rect::new(p.x - 9. * s, p.y + 7. * s, 18. * s, 25. * s),
        8. * s,
        INK,
    );
    draw_line(p.x, p.y - 12. * s, p.x, p.y - 48. * s, 3. * s, INK);
    draw_triangle(
        p + vec2(2., -48.) * s,
        p + vec2(24., -40.) * s,
        p + vec2(2., -32.) * s,
        GOLD,
    );
}
fn world_node(index: usize) -> Vec2 {
    vec2(
        640. + index as f32 * 410.,
        450. + [0., -65., 25., -75., 0.][index % 5],
    )
}

fn localized(s: &str) -> String {
    LANGUAGE.with(|l| language::translate(l.get(), s))
}
fn font_plan(size: f32) -> (u16, f32) {
    font_raster(size, RENDER_SCALE.with(|s| s.get()), screen_dpi_scale())
}
fn text_width(s: &str, size: f32) -> f32 {
    let timer = performance::begin();
    let s = localized(s);
    let (font_size, font_scale) = font_plan(size);
    let width = with_size_font(font_size, |f| {
        measure_text(&s, Some(f), font_size, font_scale).width
    });
    performance::end("text_measure", timer);
    width
}
fn text_raw(s: &str, x: f32, y: f32, size: f32, c: Color) {
    TEXT_QUEUE.with(|q| q.borrow_mut().push((s.to_owned(), vec2(x, y), size, c)));
}
fn flush_rendered() {
    // Finish all glyph additions before submitting quads, since atlas growth
    // changes the coordinates of older glyphs. Use highp UVs for the large atlas.
    TEXT_QUEUE.with(|q| {
        for (s, _, size, _) in q.borrow().iter() {
            let size = font_plan(*size).0;
            with_size_font(size, |font| {
                font.populate_font_cache(
                    &s.chars().collect::<Vec<_>>(),
                    (size as f32 * screen_dpi_scale()).ceil() as u16,
                )
            });
        }
    });
    SPRITE_QUEUE.with(|q| {
        for (texture, source, p, size) in q.borrow_mut().drain(..) {
            draw_texture_ex(
                &texture,
                p.x,
                p.y,
                WHITE,
                DrawTextureParams {
                    source: Some(source),
                    dest_size: Some(size),
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
    });
    TEXT_MATERIAL.with(|m| gl_use_material(m.get().unwrap()));
    TEXT_QUEUE.with(|q| {
        let mut q = q.borrow_mut();
        q.sort_by_key(|(_, _, size, _)| font_plan(*size).0);
        for (s, p, size, c) in q.drain(..) {
            text_now(&s, p.x, p.y, size, c);
        }
    });
    // A following modal may grow the atlas; finish the previous layer first.
    unsafe {
        get_internal_gl().flush();
    }
    gl_use_default_material();
}
fn begin_scene() {
    SPRITE_QUEUE.with(|q| q.borrow_mut().clear());
    TEXT_QUEUE.with(|q| q.borrow_mut().clear());
}
fn text_now(s: &str, x: f32, y: f32, size: f32, c: Color) {
    let timer = performance::begin();
    let (font_size, font_scale) = font_plan(size);
    with_size_font(font_size, |f| {
        draw_text_ex(
            s,
            x,
            y,
            TextParams {
                font: Some(f),
                font_size,
                font_scale,
                color: c,
                ..Default::default()
            },
        );
    });
    performance::end("text_draw", timer);
}
fn text(s: &str, x: f32, y: f32, size: f32, c: Color) {
    text_raw(&localized(s), x, y, size, c);
}
fn text_center(s: &str, x: f32, y: f32, size: f32, c: Color) {
    let localized = localized(s);
    let (font_size, font_scale) = font_plan(size);
    let width = with_size_font(font_size, |f| {
        measure_text(&localized, Some(f), font_size, font_scale).width
    });
    text_raw(&localized, x - width / 2., y, size, c);
}
fn star(p: Vec2, r: f32, c: Color) {
    for i in 0..10 {
        let a = i as f32 * std::f32::consts::PI / 5. - std::f32::consts::FRAC_PI_2;
        let b = (i + 1) as f32 * std::f32::consts::PI / 5. - std::f32::consts::FRAC_PI_2;
        let ra = if i % 2 == 0 { r } else { r * 0.47 };
        let rb = if i % 2 == 0 { r * 0.47 } else { r };
        draw_triangle(
            p,
            p + vec2(a.cos(), a.sin()) * ra,
            p + vec2(b.cos(), b.sin()) * rb,
            c,
        );
    }
}
fn cloud(x: f32, y: f32, s: f32) {
    let c = Color::new(1., 1., 1., 0.60);
    for (dx, dy, r) in [
        (0., 0., 36.),
        (35., -15., 46.),
        (78., 0., 35.),
        (38., 13., 38.),
    ] {
        circle(x + dx * s, y + dy * s, r * s, c);
    }
}
fn lock(x: f32, y: f32) {
    ring(x, y - 6., 9., 4., INK);
    rounded(Rect::new(x - 14., y - 4., 28., 23.), 5., INK);
    circle(x, y + 5., 3., CREAM);
}
fn rook(p: Vec2, s: f32, wiggle: f32) {
    if RAW_ART.with(std::cell::Cell::get) {
        rook_raw(p, s, wiggle);
        return;
    }
    let drawn = MASCOT_SPRITE.with(|cache| {
        if let Some((texture, source)) = cache.get() {
            SPRITE_QUEUE.with(|q| {
                q.borrow_mut().push((
                    texture.clone(),
                    *source,
                    p + vec2(wiggle, 0.) - vec2(64., 80.) * s,
                    vec2(128., 160.) * s,
                ))
            });
            true
        } else {
            false
        }
    });
    if !drawn {
        rook_raw(p, s, wiggle);
    }
}

fn rook_raw(p: Vec2, s: f32, wiggle: f32) {
    let p = p + vec2(wiggle, 0.);
    let tr = |x: f32, y: f32| Rect::new(p.x + x * s, p.y + y * s, 0., 0.);
    draw_ellipse(
        p.x,
        p.y + 38. * s,
        38. * s,
        10. * s,
        0.,
        Color::new(0.2, 0.22, 0.24, 0.18),
    );
    let body = Color::new(0.96, 0.87, 0.67, 1.);
    let edge = Color::new(0.57, 0.41, 0.28, 1.);
    let rect = |x: f32, y: f32, w: f32, h: f32| {
        let q = tr(x, y);
        Rect::new(q.x, q.y, w * s, h * s)
    };
    rounded(rect(-29., -29., 58., 66.), 9. * s, edge);
    rounded(rect(-25., -28., 50., 62.), 8. * s, body);
    rounded(rect(-35., -40., 70., 23.), 5. * s, edge);
    rounded(rect(-32., -40., 64., 18.), 4. * s, CREAM);
    for x in [-32., -9., 14.] {
        rounded(rect(x, -51., 18., 22.), 3. * s, edge);
        rounded(rect(x + 2., -49., 14., 20.), 2. * s, CREAM);
    }
    rounded(rect(-36., 29., 72., 14.), 5. * s, edge);
    rounded(rect(-33., 29., 66., 10.), 4. * s, CREAM);
    rounded(rect(-27., 15., 54., 8.), 3. * s, CORAL);
    draw_triangle(
        vec2(p.x + 12. * s, p.y + 20. * s),
        vec2(p.x + 41. * s, p.y + 26. * s),
        vec2(p.x + 29. * s, p.y + 8. * s),
        CORAL,
    );
    for dx in [-11., 11.] {
        circle(p.x + dx * s, p.y - 7. * s, 3.7 * s, INK);
        circle(p.x + (dx - 1.) * s, p.y - 8. * s, 1.2 * s, WHITE);
    }
    circle(
        p.x - 18. * s,
        p.y + 2. * s,
        4. * s,
        Color::new(1., 0.60, 0.54, 1.),
    );
    circle(
        p.x + 18. * s,
        p.y + 2. * s,
        4. * s,
        Color::new(1., 0.60, 0.54, 1.),
    );
    draw_line(p.x - 5. * s, p.y + 3. * s, p.x, p.y + 7. * s, 2.5 * s, INK);
    draw_line(p.x, p.y + 7. * s, p.x + 5. * s, p.y + 3. * s, 2.5 * s, INK);
}
// Bake the original procedural artwork once at 3x logical size. Runtime
// animation moves crisp texture quads instead of rebuilding thousands of curves.
async fn prepare_sprite_cache() {
    set_default_camera();
    clear_background(CREAM);
    text_center(
        "Getting your adventure ready...",
        screen_width() / 2.,
        screen_height() / 2.,
        30.,
        INK,
    );
    flush_rendered();
    next_frame().await;
    // Twelve pieces plus the mascot share one texture: their animation is one batch.
    let target = render_target_msaa(1536, 1920);
    target.texture.set_filter(FilterMode::Linear);
    let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., 512., 640.));
    camera.render_target = Some(target.clone());
    set_camera(&camera);
    clear_background(Color::new(0., 0., 0., 0.));
    let mut cache = std::collections::HashMap::new();
    let mut i = 0;
    for side in [Side::Player, Side::Opponent] {
        for kind in rook_rainbow_quest::movement::KINDS {
            let x = (i % 4) as f32 * 128.;
            let y = (i / 4) as f32 * 160.;
            chess_icon_raw(kind, vec2(x + 64., y + 64.), 1., side);
            cache.insert(
                (kind.to_owned(), side),
                (
                    target.texture.clone(),
                    Rect::new(x * 3., 1920. - y * 3. - 384., 384., 384.),
                ),
            );
            i += 1;
        }
    }
    let x = (i % 4) as f32 * 128.;
    let y = (i / 4) as f32 * 160.;
    rook_raw(vec2(x + 64., y + 80.), 1., 0.);
    set_default_camera();
    clear_background(CREAM);
    text_center(
        "Getting your adventure ready...",
        screen_width() / 2.,
        screen_height() / 2.,
        30.,
        INK,
    );
    flush_rendered();
    next_frame().await;
    PIECE_SPRITES.with(|c| assert!(c.set(cache).is_ok()));
    MASCOT_SPRITE.with(|c| {
        assert!(c
            .set((
                target.texture.clone(),
                Rect::new(x * 3., 1920. - y * 3. - 480., 384., 480.)
            ))
            .is_ok())
    });
}

fn reset_ui_font() {
    let mut font = load_ttf_font_from_bytes(include_bytes!("../assets/Nunito-Bold.ttf"))
        .expect("Bundled font");
    font.set_filter(FilterMode::Linear);
    UI_FONT.with(|f| *f.borrow_mut() = Some(font));
    FONT_RASTERS.with(|s| s.borrow_mut().clear());
}
fn with_size_font<T>(size: u16, draw: impl FnOnce(&Font) -> T) -> T {
    UI_FONT.with(|fonts| {
        let fonts = fonts.borrow();
        let font = fonts.as_ref().unwrap();
        let raster = (size as f32 * screen_dpi_scale()).ceil() as u16;
        if FONT_RASTERS.with(|s| s.borrow_mut().insert(raster)) {
            let characters: Vec<_> = (32u8..=126)
                .map(char::from)
                .chain("åäöÅÄÖ–—…é".chars())
                .collect();
            font.populate_font_cache(&characters, raster);
        }
        draw(font)
    })
}
fn warm_font_cache() {
    for logical in [
        18., 19., 20., 22., 23., 24., 25., 27., 28., 31., 34., 35., 36., 38., 48., 58.,
    ] {
        with_size_font(font_plan(logical).0, |_| ());
    }
}

// Small original synthesized WAV effects, embedded by code; no external audio files.
fn wave(notes: &[f32], duration: f32) -> Vec<u8> {
    let rate = 22050u32;
    let samples = (rate as f32 * duration) as usize;
    let count = samples * notes.len();
    let mut out = Vec::with_capacity(44 + count * 2);
    out.extend(b"RIFF");
    out.extend(&(36 + count as u32 * 2).to_le_bytes());
    out.extend(b"WAVEfmt ");
    out.extend(&16u32.to_le_bytes());
    out.extend(&1u16.to_le_bytes());
    out.extend(&1u16.to_le_bytes());
    out.extend(&rate.to_le_bytes());
    out.extend(&(rate * 2).to_le_bytes());
    out.extend(&2u16.to_le_bytes());
    out.extend(&16u16.to_le_bytes());
    out.extend(b"data");
    out.extend(&(count as u32 * 2).to_le_bytes());
    for freq in notes {
        for i in 0..samples {
            let t = i as f32 / rate as f32;
            let u = i as f32 / samples as f32;
            let env = (u * 20.).min(1.) * (1. - u).powf(2.);
            let value = ((t * freq * std::f32::consts::TAU).sin() * 0.7
                + (t * freq * std::f32::consts::TAU * 2.).sin() * 0.15)
                * env;
            out.extend(&((value * 16000.) as i16).to_le_bytes());
        }
    }
    out
}

#[macroquad::main(config)]
async fn main() {
    reset_ui_font();
    use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
    let text_material = load_material(
        ShaderSource::Glsl {
            vertex: "#version 100\nattribute vec3 position; attribute vec2 texcoord; attribute vec4 color0; varying highp vec2 uv; varying lowp vec4 color; uniform mat4 Model; uniform mat4 Projection; void main(){gl_Position=Projection*Model*vec4(position,1.0); uv=texcoord; color=color0/255.0;}",
            fragment: "#version 100\nprecision highp float; varying highp vec2 uv; varying lowp vec4 color; uniform sampler2D Texture; void main(){gl_FragColor=color*texture2D(Texture,uv);}",
        },
        MaterialParams { pipeline_params: PipelineParams { color_blend: Some(BlendState::new(Equation::Add, BlendFactor::Value(BlendValue::SourceAlpha), BlendFactor::OneMinusValue(BlendValue::SourceAlpha))), ..Default::default() }, ..Default::default() },
    ).expect("High-precision text shader");
    TEXT_MATERIAL.with(|m| assert!(m.set(text_material).is_ok()));
    let args: Vec<String> = std::env::args().collect();
    let perf_test = args.iter().any(|s| s == "--performance-test");
    if perf_test {
        performance::enable();
        // Read GL diagnostics only while a context is current on this thread.
        unsafe {
            let renderer = macroquad::miniquad::gl::glGetString(0x1F01);
            if !renderer.is_null() {
                println!(
                    "GL renderer: {}",
                    std::ffi::CStr::from_ptr(renderer.cast()).to_string_lossy()
                );
            }
        }
    }
    RAW_ART.with(|v| v.set(perf_test && args.iter().any(|s| s == "--performance-raw-art")));
    let perf_frames = args
        .iter()
        .position(|s| s == "--performance-frames")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(960);
    let perf_report = args
        .iter()
        .position(|s| s == "--performance-report")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);
    let campaign_test = args.iter().any(|s| s == "--campaign-test");
    let settings_test = args.iter().any(|s| s == "--settings-test");
    let exit_test = args.iter().any(|s| s == "--exit-test");
    let retry_test = args.iter().any(|s| s == "--retry-test");
    let mate_test = args.iter().any(|s| s == "--mate-test");
    let menu_test = args.iter().any(|s| s == "--menu-test");
    let chess_test = args.iter().any(|s| s == "--chess-test");
    let dead_test = args.iter().any(|s| s == "--dead-test");
    let regression_test = settings_test
        || exit_test
        || retry_test
        || menu_test
        || mate_test
        || chess_test
        || dead_test;
    let smoke =
        perf_test || regression_test || campaign_test || args.iter().any(|s| s == "--smoke-test");
    let exit_report = args
        .iter()
        .position(|s| s == "--exit-report")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);
    let screenshot_dir = args
        .iter()
        .position(|s| s == "--screenshots")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);
    if let Some(p) = &screenshot_dir {
        std::fs::create_dir_all(p).expect("Create screenshot directory");
    }
    let mut app = App::new(smoke).await;
    prepare_sprite_cache().await;
    if perf_test {
        for _ in 0..20 {
            let timer = performance::begin();
            set_cursor_grab(false);
            performance::end("cursor_release_micro", timer);
        }
    }
    assert!(!regression_test || app.save.language == Language::Swedish);
    let mut target = render_target_msaa(W as u32, H as u32);
    target.texture.set_filter(FilterMode::Linear);
    // The scene is opaque. Ignore intermediate framebuffer alpha during the final
    // composite so translucent clouds/shadows do not darken when drawn twice.
    let composite=load_material(
        ShaderSource::Glsl {
            vertex: "#version 100\nattribute vec3 position; attribute vec2 texcoord; varying highp vec2 uv; uniform mat4 Model; uniform mat4 Projection; void main(){gl_Position=Projection*Model*vec4(position,1.0); uv=texcoord;}",
            fragment: "#version 100\nprecision highp float; varying highp vec2 uv; uniform sampler2D Texture; void main(){gl_FragColor=vec4(texture2D(Texture,uv).rgb,1.0);}",
        },
        MaterialParams::default(),
    ).expect("Opaque scene composite shader");
    let mut frame = 0u32;
    let mut campaign_phase = 0u8;
    let mut campaign_phase_frame = 0u32;
    let mut campaign_captures = std::collections::HashSet::new();
    let mut smoke_step = 0usize;
    let mut warmed_scale = 0.;
    loop {
        performance::frame(frame);
        let frame_timer = performance::begin();
        let viewport = Viewport::new(screen_width(), screen_height());
        let scale = viewport.scale;
        let offset = vec2(viewport.x, viewport.y);
        let offscreen =
            (campaign_test && campaign_phase == 3) || (settings_test && (60..=70).contains(&frame));
        let physical_scale = if offscreen {
            3.
        } else {
            scale * screen_dpi_scale()
        };
        if offscreen && (target.texture.width() != 3840. || target.texture.height() != 2400.) {
            target = render_target_msaa(3840, 2400);
            target.texture.set_filter(FilterMode::Linear);
        }
        RENDER_SCALE.with(|s| s.set(physical_scale));
        if !offscreen && (physical_scale - warmed_scale).abs() > 0.01 {
            // Rebuild on display-scale changes rather than accumulating every
            // raster size from a long window-resize gesture in one growing atlas.
            reset_ui_font();
            warm_font_cache();
            warmed_scale = physical_scale;
        }
        let input_timer = performance::begin();
        app.input(offset, scale);
        performance::end("input", input_timer);
        if perf_test {
            app.pointer = if frame < 480 {
                vec2(1165., 675.)
            } else {
                vec2(115., 675.)
            };
            app.clicked = false;
            app.activated = None;
            if frame == 0 {
                for l in &app.levels {
                    app.save.complete(&l.id);
                }
                app.screen = Screen::Map;
                app.save.muted = true;
            }
            if frame == 4 && !args.iter().any(|s| s == "--performance-windowed") {
                app.fullscreen = true;
                set_fullscreen(true);
            }
            if frame >= 12 && (frame - 12).is_multiple_of(24) {
                let step = (frame - 12) / 24;
                let timer = performance::begin();
                if step < 19 {
                    app.navigate(Direction::Right);
                } else if step < 38 {
                    app.navigate(Direction::Left);
                }
                performance::end("navigate", timer);
            }
        }
        if settings_test || exit_test {
            if frame == 4 {
                app.activated = Some(91);
            }
            if frame == 8 && (!exit_test || !args.iter().any(|s| s == "--exit-windowed")) {
                app.activated = Some(106);
            }
            if settings_test {
                match frame {
                    12 => app.activated = Some(110),
                    16 => {
                        assert!(app.save.all_levels_unlocked[0]);
                        assert!(!app.save.all_levels_unlocked[1]);
                        assert!(app.save.profiles[0].is_empty());
                        assert!(campaign::unlocked(&app.levels, &app.save, 39));
                        assert!(worlds::unlocked(&app.worlds, &app.levels, &app.save, 1));
                        assert!(!app.interactive.contains(&110));
                    }
                    18 | 22 | 26 | 32 => app.activated = Some(103),
                    24 => app.activated = Some(109),
                    34 => app.activated = Some(106),
                    40 => request_new_screen_size(960., 600.),
                    52 => app.activated = Some(108),
                    72 => app.shutdown.request(),
                    _ => {}
                }
            }
            if exit_test && frame == 16 {
                app.pointer = vec2(850., 696.);
                app.clicked = true;
            }
        }
        if menu_test {
            match frame {
                2 => {
                    assert!(app.screen == Screen::Main);
                    assert!(!app.interactive.contains(&201));
                    assert!(!app.save.has_adventure());
                }
                4 | 12 => app.activated = Some(202),
                8 => app.activated = Some(205),
                10 => {
                    assert!(!app.confirm_new);
                    assert!(!app.save.has_adventure());
                }
                16 => app.activated = Some(204),
                18 => {
                    assert!(app.screen == Screen::Worlds);
                    assert!(app.save.has_adventure());
                }
                20 => app.activated = Some(200),
                22 => {
                    assert!(app.screen == Screen::Map);
                    assert_eq!(app.world_range(), 0..20);
                }
                24 => app.activated = Some(4),
                28 => app.activated = Some(5),
                30 => {
                    assert!(app.screen == Screen::Main);
                }
                32 => app.activated = Some(201),
                34 => {
                    assert!(app.screen == Screen::Worlds);
                    app.save.complete(&app.levels[0].id);
                    app.save.active_profile = 1;
                    app.save.complete("other-player-star");
                    app.save.active_profile = 0;
                }
                36 => app.activated = Some(5),
                40 => app.activated = Some(202),
                44 => app.activated = Some(205),
                46 => {
                    assert!(app.save.completed(&app.levels[0].id));
                }
                48 => app.activated = Some(202),
                52 => app.activated = Some(204),
                54 => {
                    assert!(app.save.profiles[0].is_empty());
                    assert!(app.save.profiles[1].contains("other-player-star"));
                    assert!(app.screen == Screen::Worlds);
                }
                56 => app.activated = Some(5),
                60 => app.activated = Some(203),
                _ => {}
            }
        }
        if dead_test {
            let square = |s: &str| Square::parse(s).unwrap();
            match frame {
                4 => {
                    app.save.unlock_all_levels();
                    app.map_index = 10;
                    app.start();
                    app.save.reduced_motion = true;
                    app.game.battle.as_mut().unwrap().units = vec![
                        rook_rainbow_quest::battle::Unit {
                            kind: "bishop".into(),
                            square: square("a1"),
                            side: Side::Player,
                        },
                        rook_rainbow_quest::battle::Unit {
                            kind: "rook".into(),
                            square: square("b2"),
                            side: Side::Opponent,
                        },
                        rook_rainbow_quest::battle::Unit {
                            kind: "bishop".into(),
                            square: square("b1"),
                            side: Side::Opponent,
                        },
                    ];
                }
                8 => {
                    assert!(app.game.select(square("a1")));
                    app.selected = true;
                    app.move_piece(square("b2"));
                }
                20 => {
                    assert!(app.failed_at.is_some());
                    assert!(!app.game.won());
                    assert_eq!(
                        app.game.battle.as_ref().unwrap().draw_reason,
                        Some("A bishop is out of reach.")
                    );
                    assert!(app.enemy_due.is_none());
                    assert!(app.save.profiles[0].is_empty());
                }
                24 => app.save.language = Language::English,
                32 => app.activated = Some(12),
                36 => {
                    assert!(app.failed_at.is_none());
                    assert!(app.game.player_turn());
                    assert!(app.game.selected.is_none());
                    assert!(app.game.battle.as_ref().unwrap().draw_reason.is_none());
                    assert!(app.save.profiles[0].is_empty());
                }
                40 => app.shutdown.request(),
                _ => {}
            }
        }
        if retry_test {
            match frame {
                4 => {
                    for l in &app.levels[..6] {
                        app.save.complete(&l.id);
                    }
                    app.map_index = 6;
                    app.game = Session::new(app.levels[6].clone()).unwrap();
                    app.start();
                    app.save.reduced_motion = true;
                }
                8 => {
                    app.pointer = center(Square::parse("b2").unwrap(), &app.game.level);
                    app.clicked = true;
                }
                16 => {
                    app.pointer = center(Square::parse("b3").unwrap(), &app.game.level);
                    app.clicked = true;
                }
                24 => {
                    assert!(app.failed_at.is_some());
                    assert_eq!(app.game.remaining(), 2);
                    assert_eq!(app.save.profiles[0].len(), 6);
                    assert!(campaign::unlocked(&app.levels, &app.save, 6));
                }
                28 => {
                    app.pointer = vec2(845., 735.);
                    app.clicked = true;
                }
                32 => {
                    assert!(app.failed_at.is_none());
                    assert!(app.game.player_turn());
                    assert_eq!(app.game.selected, Some(Square::parse("b1").unwrap()));
                    assert_eq!(app.game.remaining(), 2);
                    assert_eq!(app.save.profiles[0].len(), 6);
                    assert!(!app.interactive.contains(&11), "No undo control");
                }
                36 => app.move_piece(Square::parse("b2").unwrap()),
                44 => app.move_piece(Square::parse("c3").unwrap()),
                52 => app.move_piece(Square::parse("c4").unwrap()),
                60 => {
                    assert!(app.failed_at.is_none());
                    assert!(app.game.player_turn());
                    assert_eq!(app.game.remaining(), 1);
                    assert_eq!(app.save.profiles[0].len(), 6);
                    assert_eq!(app.game.hint().unwrap().to, Square::parse("c5").unwrap());
                }
                68 => app.move_piece(Square::parse("c5").unwrap()),
                76 => {
                    assert!(app.failed_at.is_none());
                    assert_eq!(app.game.units()[0].kind, "queen");
                    app.move_piece(Square::parse("b4").unwrap());
                }
                84 => {
                    assert!(app.game.won());
                    assert_eq!(app.game.remaining(), 0);
                    assert_eq!(app.save.profiles[0].len(), 7);
                    assert!(app.save.completed("lesson-pawn-01"));
                }
                88 => app.shutdown.request(),
                _ => {}
            }
        }
        if chess_test {
            let sq = |s: &str| Square::parse(s).unwrap();
            let fixture = |app: &mut App, fen: &str| {
                let mut position = rook_rainbow_quest::standard::Standard::from_fen(fen).unwrap();
                let b = app.game.battle.as_mut().unwrap();
                b.units = position.units();
                b.turn = Side::Player;
                b.outcome = position.outcome();
                b.standard = Some(position);
            };
            match frame {
                2 => {
                    app.save.unlock_all_levels();
                    app.world_index = 2;
                    app.map_index = 42;
                    app.start();
                    app.save.reduced_motion = true;
                }
                3 => {
                    assert!(app.game.select(sq("e1")));
                    app.selected = true;
                }
                4 => {
                    assert!(app.game.select(sq("e1")));
                    app.selected = true;
                    app.move_piece(sq("g1"));
                }
                12 => {
                    assert!(app.game.units().iter().any(|u| u.kind == "king"
                        && u.side == Side::Player
                        && u.square == sq("g1")));
                    assert!(app.game.units().iter().any(|u| u.kind == "rook"
                        && u.side == Side::Player
                        && u.square == sq("f1")));
                    assert_eq!(app.game.selected, None);
                }
                16 => {
                    app.map_index = 43;
                    app.start();
                    assert!(app.game.select(sq("e5")));
                    app.selected = true;
                    app.move_piece(sq("f6"));
                }
                24 => {
                    assert!(!app.game.units().iter().any(|u| u.square == sq("f5")));
                    assert!(app
                        .game
                        .units()
                        .iter()
                        .any(|u| u.square == sq("f6") && u.side == Side::Player));
                }
                28 => {
                    app.map_index = 44;
                    app.start();
                    assert!(app.game.select(sq("a7")));
                    app.selected = true;
                    app.smoke = false;
                    app.move_piece(sq("a8"));
                    app.smoke = true;
                    assert_eq!(app.promotion_to, Some(sq("a8")));
                }
                30 => app.activated = Some(123),
                40 => {
                    assert_eq!(app.promotion_to, None);
                    assert_eq!(app.message, "A new knight!");
                    assert!(app.game.units().iter().any(|u| u.square == sq("a8")
                        && u.kind == "knight"
                        && u.side == Side::Player));
                }
                44 => {
                    app.map_index = 59;
                    app.start();
                    assert!(app.game.level.final_boss && app.game.level.icon.castle);
                    assert_eq!(app.game.units().len(), 32);
                }
                50 => {
                    fixture(&mut app, "7k/5Q2/6K1/8/8/8/8/8 w - - 0 1");
                    assert!(app.game.select(sq("f7")));
                    app.move_piece(sq("g7"));
                }
                62 => {
                    assert!(app.game.won());
                    assert!(app.save.completed("chess-20"));
                    assert_eq!(
                        app.completed_world, None,
                        "Access alone cannot complete a world"
                    );
                }
                66 => {
                    app.map_index = 58;
                    app.start();
                    fixture(&mut app, "7k/5Q2/6K1/8/8/8/8/8 w - - 0 1");
                    assert!(app.game.select(sq("f7")));
                    app.move_piece(sq("e6"));
                }
                78 => {
                    assert!(app.failed_at.is_some());
                    assert!(!app.save.completed("chess-19"));
                    assert!(app.save.completed("chess-20"));
                }
                80 => app.activated = Some(12),
                88 => {
                    assert!(!app.game.failed());
                    assert_eq!(app.game.selected, None);
                    assert!(app.save.completed("chess-20"));
                }
                90 => app.shutdown.request(),
                _ => {}
            }
        }
        if mate_test {
            let square = |s: &str| Square::parse(s).unwrap();
            match frame {
                2 => {
                    for l in &app.levels[..20] {
                        app.save.complete(&l.id);
                    }
                    app.screen = Screen::Worlds;
                    app.world_index = 1;
                    app.world_avatar = world_node(1);
                    app.world_camera = 410.;
                }
                4 => {
                    for l in &app.levels[..20] {
                        app.save.complete(&l.id);
                    }
                    app.world_index = 1;
                    app.map_index = 20;
                    app.start();
                    app.save.reduced_motion = true;
                    app.game.battle.as_mut().unwrap().units = vec![
                        rook_rainbow_quest::battle::Unit {
                            kind: "queen".into(),
                            square: square("d3"),
                            side: Side::Player,
                        },
                        rook_rainbow_quest::battle::Unit {
                            kind: "king".into(),
                            square: square("f6"),
                            side: Side::Opponent,
                        },
                    ];
                }
                8 => {
                    assert!(app.game.select(square("d3")));
                    app.selected = true;
                    app.move_piece(square("e4"));
                }
                20 => {
                    assert!(app.failed_at.is_some());
                    assert!(!app.game.won());
                    assert!(!app.game.battle.as_ref().unwrap().in_check());
                    assert_eq!(app.save.profiles[0].len(), 20);
                }
                22 => app.save.language = Language::English,
                24 => app.activated = Some(12),
                28 => {
                    assert!(app.failed_at.is_none());
                    assert_eq!(app.game.selected, None);
                    assert_eq!(
                        app.game.units().iter().filter(|u| u.kind == "rook").count(),
                        2
                    );
                    app.game.battle.as_mut().unwrap().units = vec![
                        rook_rainbow_quest::battle::Unit {
                            kind: "rook".into(),
                            square: square("a1"),
                            side: Side::Player,
                        },
                        rook_rainbow_quest::battle::Unit {
                            kind: "rook".into(),
                            square: square("b5"),
                            side: Side::Player,
                        },
                        rook_rainbow_quest::battle::Unit {
                            kind: "king".into(),
                            square: square("f6"),
                            side: Side::Opponent,
                        },
                    ];
                }
                32 => {
                    assert!(app.game.select(square("a1")));
                    app.selected = true;
                    app.move_piece(square("a6"));
                }
                40 => {
                    assert!(app.game.won());
                    assert!(app.won_at.is_some());
                    assert_eq!(app.game.remaining(), 1);
                    assert!(app.save.completed("mate-01"));
                }
                44 => app.shutdown.request(),
                _ => {}
            }
        }
        let update_timer = performance::begin();
        app.update();
        performance::end("update", update_timer);
        let draw_timer = performance::begin();
        set_default_camera();
        clear_background(INK);
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., W, H));
        if offscreen {
            camera.render_target = Some(target.clone());
        } else {
            let (tw, th) = viewport.raster_size(screen_dpi_scale());
            camera = native_camera((
                (offset.x * screen_dpi_scale()).round() as i32,
                (offset.y * screen_dpi_scale()).round() as i32,
                tw as i32,
                th as i32,
            ));
        }
        set_camera(&camera);
        if offscreen {
            clear_background(CREAM);
        }
        if !perf_test || !args.iter().any(|s| s == "--performance-clear-only") {
            app.draw();
        }
        performance::end("draw", draw_timer);
        set_default_camera();
        if offscreen {
            gl_use_material(&composite);
            draw_texture_ex(
                &target.texture,
                offset.x,
                offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(W * scale, H * scale)),
                    flip_y: true,
                    ..Default::default()
                },
            );
            gl_use_default_material();
        }
        if settings_test {
            if let Some(name) = match frame {
                10 => Some("settings-sv-unlock"),
                20 => Some("settings-sv-fullscreen"),
                30 => Some("settings-en-fullscreen"),
                48 => Some("settings-en-windowed"),
                65 => Some("settings-sv-4k"),
                _ => None,
            } {
                assert!(app.settings);
                // Exercise the real toggle in both languages: Off means reduced motion.
                assert_eq!(app.save.reduced_motion, frame == 20 || frame == 30);
                assert_eq!(
                    app.save.language,
                    if frame == 30 || frame == 48 {
                        Language::English
                    } else {
                        Language::Swedish
                    }
                );
                if let Some(dir) = &screenshot_dir {
                    get_screen_data().export_png(dir.join(format!("{name}.png")).to_str().unwrap());
                    if offscreen {
                        target
                            .texture
                            .get_texture_data()
                            .export_png(dir.join("settings-native-4k.png").to_str().unwrap());
                    }
                }
            }
        }
        if menu_test {
            if frame == 2 {
                assert!(!app.interactive.contains(&201));
            }
            if frame == 30 {
                assert!(app.interactive.contains(&201));
            }
            let name = match frame {
                2 => Some("main-first-start"),
                6 => Some("new-game-confirm"),
                18 => Some("world-map"),
                30 => Some("main-continue"),
                _ => None,
            };
            if let (Some(name), Some(dir)) = (name, &screenshot_dir) {
                get_screen_data().export_png(dir.join(format!("{name}.png")).to_str().unwrap());
            }
        }
        if dead_test && [20, 28].contains(&frame) {
            if let Some(dir) = &screenshot_dir {
                get_screen_data().export_png(
                    dir.join(format!("dead-position-{frame}.png"))
                        .to_str()
                        .unwrap(),
                );
            }
        }
        if chess_test && [3, 29, 46, 62, 78].contains(&frame) {
            if let Some(dir) = &screenshot_dir {
                get_screen_data()
                    .export_png(dir.join(format!("chess-{frame}.png")).to_str().unwrap());
            }
        }
        if mate_test && frame == 3 {
            if let Some(dir) = &screenshot_dir {
                get_screen_data().export_png(dir.join("worlds-unlocked.png").to_str().unwrap());
            }
        }
        if mate_test && (frame == 20 || frame == 23 || frame == 40) {
            if let Some(dir) = &screenshot_dir {
                get_screen_data().export_png(
                    dir.join(if frame == 20 {
                        "stalemate-retry.png"
                    } else if frame == 23 {
                        "stalemate-retry-en.png"
                    } else {
                        "checkmate-win.png"
                    })
                    .to_str()
                    .unwrap(),
                );
            }
        }
        if retry_test && frame == 24 {
            if let Some(dir) = &screenshot_dir {
                get_screen_data().export_png(dir.join("retry-failed.png").to_str().unwrap());
            }
        }
        if retry_test && [60, 74, 84].contains(&frame) {
            if let Some(dir) = &screenshot_dir {
                get_screen_data().export_png(
                    dir.join(format!("pawn-detour-{frame}.png"))
                        .to_str()
                        .unwrap(),
                );
            }
        }
        if campaign_test {
            let capture =
                if let Some(world) = app.completed_world.filter(|_| app.screen == Screen::Level) {
                    assert!(app.interactive.contains(&33));
                    assert!(!app.interactive.contains(&32));
                    Some((500 + world, format!("world-complete-{:02}.png", world + 1)))
                } else if app.screen == Screen::Map
                    && app.map_travel.is_none()
                    && [6, 10, 15, 20, 40].contains(&app.map_index)
                {
                    Some((
                        200 + app.map_index,
                        format!("map-{:02}.png", app.map_index + 1),
                    ))
                } else if app.screen == Screen::Level
                    && app.map_index == 7
                    && app.motion.is_none()
                    && !app.game.won()
                    && app
                        .game
                        .units()
                        .iter()
                        .any(|u| u.side == Side::Player && u.kind == "queen")
                {
                    assert_eq!(app.game.remaining(), 1);
                    Some((407, "pawn-lesson-promoted.png".into()))
                } else if app.screen == Screen::Level
                    && app.game.level.is_match()
                    && !app.game.player_turn()
                    && app.motion.is_none()
                    && app.won_at.is_none()
                {
                    Some((
                        300 + app.map_index,
                        format!("opponent-turn-{:02}.png", app.map_index + 1),
                    ))
                } else {
                    None
                };
            if let Some((key, name)) = capture {
                if campaign_captures.insert(key) {
                    if let Some(dir) = &screenshot_dir {
                        get_screen_data().export_png(dir.join(name).to_str().unwrap());
                    }
                }
            }
        }
        if smoke && !regression_test && !perf_test {
            if frame == 12 {
                if let Some(dir) = &screenshot_dir {
                    get_screen_data().export_png(dir.join("01-map.png").to_str().unwrap());
                }
            }
            if frame >= 16 {
                app.save.reduced_motion = true;
                match campaign_phase {
                    0 => {
                        assert!(app.screen == Screen::Main);
                        assert!(!app.save.has_adventure());
                        app.activated = Some(202);
                        app.main_menu();
                        assert!(app.confirm_new);
                        app.activated = Some(204);
                        app.new_game_dialog();
                        assert!(app.screen == Screen::Worlds);
                        app.activated = Some(200);
                        app.world_map(0.);
                        assert!(app.screen == Screen::Map);
                        app.activated = Some(1);
                        app.map(0.);
                        assert!(app.screen == Screen::Level);
                        campaign_phase = 1;
                    }
                    1 => {
                        if app.screen == Screen::Level && campaign_captures.insert(app.map_index) {
                            if let Some(dir) = &screenshot_dir {
                                get_screen_data().export_png(
                                    dir.join(format!("level-{:02}.png", app.map_index + 1))
                                        .to_str()
                                        .unwrap(),
                                );
                            }
                        }
                        assert!(
                            app.failed_at.is_none(),
                            "Authored route failed at level {}",
                            app.map_index + 1
                        );
                        if frame.is_multiple_of(4) {
                            if app.screen == Screen::Worlds && app.world_travel.is_none() {
                                app.enter_world();
                            } else if app.screen == Screen::Map && app.map_travel.is_none() {
                                app.start();
                            } else if app.screen == Screen::Level
                                && app.game.level.is_standard()
                                && app.motion.is_none()
                                && smoke_step == app.game.level.example_turns.len()
                                && (app.game.player_turn() || app.game.won())
                            {
                                assert!(app.levels[..40].iter().all(|l| app.save.completed(&l.id)));
                                if !app.game.won() {
                                    assert!(
                                        !app.save.completed(&app.game.level.id),
                                        "Opening must not award a win"
                                    );
                                }
                                smoke_step = 0;
                                if app.map_index + 1 == app.levels.len() {
                                    app.screen = Screen::Map;
                                    campaign_phase = 2;
                                } else {
                                    // Isolated tour access only; never invent completion stars.
                                    app.save.unlock_all_levels();
                                    app.screen = Screen::Map;
                                    app.walk_to(app.map_index + 1);
                                }
                            } else if app.won_at.is_some() && app.screen == Screen::Level {
                                if !campaign_test {
                                    break;
                                }
                                if app.map_index + 1 == app.world_range().end {
                                    smoke_step = 0;
                                    let completed = app.world_index;
                                    assert_eq!(app.completed_world, Some(completed));
                                    app.won_at = None;
                                    app.completed_world = None;
                                    app.settle();
                                    assert_eq!(app.completed_world, Some(completed));
                                    app.activated = Some(33);
                                    app.victory(0.);
                                    assert!(app.screen == Screen::Worlds);
                                    assert_eq!(app.completed_world, None);
                                    assert!(worlds::unlocked(
                                        &app.worlds,
                                        &app.levels,
                                        &app.save,
                                        completed + 1
                                    ));
                                    app.navigate_world(Direction::Right);
                                } else {
                                    smoke_step = 0;
                                    app.activated = Some(32);
                                    app.victory(0.);
                                }
                            } else if app.screen == Screen::Level
                                && app.motion.is_none()
                                && app.game.player_turn()
                            {
                                let m = if app.game.level.is_match() {
                                    let step = app
                                        .game
                                        .level
                                        .example_turns
                                        .get(smoke_step)
                                        .expect("Match witness remains");
                                    Move {
                                        from: Square::parse(&step.from).unwrap(),
                                        to: Square::parse(&step.to).unwrap(),
                                    }
                                } else {
                                    Move {
                                        from: app.game.selected.unwrap(),
                                        to: Square::parse(
                                            app.game
                                                .level
                                                .example_solution
                                                .get(smoke_step + 1)
                                                .expect("Lesson witness remains"),
                                        )
                                        .unwrap(),
                                    }
                                };
                                assert!(app.game.select(m.from));
                                app.move_piece(m.to);
                                smoke_step += 1;
                            }
                        }
                    }
                    2 => {
                        if app.map_index + 1 == app.levels.len() && campaign_captures.insert(100) {
                            if let Some(dir) = &screenshot_dir {
                                get_screen_data().export_png(
                                    dir.join("campaign-final-map.png").to_str().unwrap(),
                                );
                            }
                        }
                        if frame.is_multiple_of(3) && app.map_travel.is_none() {
                            if app.map_index > app.world_range().start {
                                app.navigate(Direction::Left);
                            } else {
                                app.save.active_profile = 1;
                                app.restore_map();
                                app.navigate(Direction::Right);
                                assert_eq!(app.map_index, 0);
                                app.save.active_profile = 0;
                                app.restore_map();
                                app.start();
                                app.fullscreen = true;
                                set_fullscreen(true);
                                campaign_phase = 3;
                                campaign_phase_frame = frame;
                            }
                        }
                    }
                    3 => {
                        if frame > campaign_phase_frame + 15 {
                            assert!(
                                campaign_captures.contains(&407),
                                "Pawn lesson promoted before its last capture"
                            );
                            assert!(campaign_captures.contains(&500));
                            assert!(campaign_captures.contains(&501));
                            assert_eq!(
                                (
                                    target.texture.width() as u32,
                                    target.texture.height() as u32
                                ),
                                (3840, 2400)
                            );
                            if let Some(dir) = &screenshot_dir {
                                get_screen_data()
                                    .export_png(dir.join("fullscreen.png").to_str().unwrap());
                                target
                                    .texture
                                    .get_texture_data()
                                    .export_png(dir.join("native-4k-render.png").to_str().unwrap());
                            }
                            set_fullscreen(false);
                            println!("Campaign passed: 60 levels across three worlds; forty winning routes and twenty legal regular-chess openings, world traversal, completion popups, piece selection, unlocking, castle finale and 4K rendering.");
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
                assert!(frame < 5000, "Campaign must not stall");
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        if regression_test {
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        frame += 1;
        next_frame().await;
        performance::end("frame", frame_timer);
        if perf_test && frame >= perf_frames {
            break;
        }
        app.shutdown.frame_presented();
        if app.shutdown.should_exit() {
            app.persist();
            if let Some(writer) = &app.save_writer {
                let _ = writer.flush();
            }
            for sound in app.sounds.iter().flatten() {
                macroquad::audio::stop_sound(sound);
            }
            break;
        }
    }
    if let Some(writer) = &app.save_writer {
        let _ = writer.flush();
    }
    if perf_test {
        performance::report(
            perf_report
                .as_deref()
                .expect("--performance-report PATH required"),
        );
    }
    if regression_test {
        assert!(
            app.shutdown.should_exit(),
            "Exit must complete through the main loop"
        );
        if let Some(report) = exit_report {
            std::fs::write(report, "graceful-exit-ok").expect("Write exit regression report");
        }
    }
}

#[cfg(test)]
mod render_tests {
    use super::*;
    use macroquad::camera::Camera;
    #[test]
    fn result_banner_leaves_every_board_square_uncovered() {
        for size in [4., 5., 6., 8.] {
            let cell = (480_f32 / size).min(120.);
            let board = Rect::new(640. - size * cell / 2., 180., size * cell, size * cell);
            assert!(!RESULT_PANEL.overlaps(&board));
            assert!(RESULT_PANEL.bottom() <= H);
        }
    }
    #[test]
    fn native_settings_camera_keeps_top_left_and_bottom_right_upright() {
        let camera = native_camera((96, 0, 1728, 1080));
        let top_left = camera.matrix().transform_point3(vec3(0., 0., 0.));
        let bottom_right = camera.matrix().transform_point3(vec3(W, H, 0.));
        assert!((top_left.x + 1.).abs() < 0.0001 && (top_left.y - 1.).abs() < 0.0001);
        assert!((bottom_right.x - 1.).abs() < 0.0001 && (bottom_right.y + 1.).abs() < 0.0001);
    }
}
