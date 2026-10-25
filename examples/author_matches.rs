//! Optional developer helper: validate and document winning routes for authored boards.
use rook_rainbow_quest::{battle::Battle, rules::Level};
use std::{fs, path::Path};
fn name(s: rook_rainbow_quest::rules::Square) -> String {
    format!("{}{}", (b'a' + s.x) as char, s.y + 1)
}
fn main() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("levels");
    let mut files: Vec<_> = fs::read_dir(folder)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    let args: Vec<_> = std::env::args().collect();
    let world2_only = args.iter().any(|a| a == "--world2");
    let world3_only = args.iter().any(|a| a == "--world3");
    let level_id = args
        .windows(2)
        .find(|a| a[0] == "--level")
        .map(|a| a[1].as_str());
    for file in files {
        let raw = fs::read_to_string(&file).unwrap();
        let mut level: Level = serde_json::from_str(&raw).unwrap();
        if !level.is_match()
            || (world2_only && !level.is_checkmate())
            || (world3_only && !level.is_standard())
            || level_id.is_some_and(|id| id != level.id)
        {
            continue;
        }
        if level.is_standard() {
            level.example_turns.clear();
        }
        let battle = Battle::new(level.clone());
        let solution = if level.is_standard() {
            let mut game = battle;
            let mut line = Vec::new();
            for _ in 0..3 {
                if game.outcome != rook_rainbow_quest::battle::Outcome::Playing {
                    break;
                }
                let m = game.hint().expect("Chess teaching move");
                game.play(m).unwrap();
                line.push(m);
                game.opponent_reply();
            }
            assert!(
                !matches!(
                    game.outcome,
                    rook_rainbow_quest::battle::Outcome::Lost
                        | rook_rainbow_quest::battle::Outcome::Stuck
                ),
                "Opening must remain playable"
            );
            line
        } else {
            battle
                .winning_line(16, 160)
                .unwrap_or_else(|| panic!("No winning route for {}", level.title))
        };
        let mut data: serde_json::Value = serde_json::from_str(&raw).unwrap();
        data["exampleTurns"] = serde_json::Value::Array(
            solution
                .iter()
                .map(|m| serde_json::json!({"from":name(m.from),"to":name(m.to)}))
                .collect(),
        );
        let output = serde_json::to_string_pretty(&data).unwrap() + "\n";
        Level::from_json(&output).unwrap_or_else(|e| panic!("{}: {e}", level.title));
        fs::write(&file, output).unwrap();
        println!("{}: {} player turns", level.title, solution.len());
    }
}
