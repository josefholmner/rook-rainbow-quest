//! Presentation-only localization. Level/save IDs and rule data never change language.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "en")]
    English,
}

pub const TEXTS: &[(&str, &str)] = &[
    (
        "The king is not in check, but the opponent has no legal moves.",
        "Kungen står inte i schack, men motståndaren har inga tillåtna drag.",
    ),
    (
        "Your queen helps the rooks cover the king's escapes.",
        "Din dam hjälper tornen att stänga kungens flyktvägar.",
    ),
    ("KING'S RULE", "KUNGENS REGEL"),
    ("A new knight!", "En ny springare!"),
    ("A new bishop!", "En ny löpare!"),
    ("A new rook!", "Ett nytt torn!"),
    ("Protect your own king.", "Skydda din egen kung."),
    (
        "Castle: king to c1 or g1.",
        "Rockad: kung till c1 eller g1.",
    ),
    ("Try e5 to f6 this turn.", "Prova e5 till f6 den här turen."),
    ("Reach a8. Pick a new piece.", "Nå a8. Välj en ny pjäs."),
    ("Checkmate wins this match.", "Schackmatt vinner partiet."),
    ("Hello, Your Majesty!", "Hej, ers majestät!"),
    ("Two Steps Forward", "Två steg framåt"),
    ("Castle to Safety", "Rockera till trygghet"),
    ("A Passing Pawn", "En passerande bonde"),
    ("Choose Your Promotion", "Välj din förvandling"),
    ("Knights Join In", "Springarna är med"),
    ("Bishop Buddies", "Löparkompisar"),
    ("Keep Your King Safe", "Skydda din kung"),
    ("Trade and Protect", "Byt och skydda"),
    ("A Growing Army", "En växande armé"),
    ("Find the Check", "Hitta schacken"),
    ("Open a File", "Öppna linjen"),
    ("The Royal Guard", "Kungens livvakt"),
    ("Pieces Work Together", "Pjäserna samarbetar"),
    ("Almost Even", "Nästan jämnt"),
    ("A Fair Fight", "En jämn kamp"),
    ("Opening Practice", "Öva på öppningen"),
    ("Both Wings Ready", "Båda sidor redo"),
    ("One More Adventure", "Ett äventyr till"),
    ("Grand Castle", "Det stora slottet"),
    ("Royal Chess Kingdom", "Kungliga schackriket"),
    ("FINAL BOSS", "SLUTBOSS"),
    ("FINAL BOSS — full game!", "SLUTBOSS — helt parti!"),
    ("CHESS MATCH", "SCHACKPARTI"),
    ("Kings move", "Kungar går"),
    ("one square, safely.", "en säker ruta."),
    ("Start: one or two steps.", "Start: ett eller två steg."),
    ("Choose your new piece", "Välj din nya pjäs"),
    ("Queen", "Dam"),
    ("Rook", "Torn"),
    ("Bishop", "Löpare"),
    ("Knight", "Springare"),
    ("Escape: choose another move", "Escape: välj ett annat drag"),
    ("Draw — try again!", "Remi — försök igen!"),
    ("Your king is checkmated", "Din kung är schackmatt"),
    ("Fifty-move draw", "Remi: femtiodragsregeln"),
    ("Stalemate", "Patt"),
    ("Insufficient material", "Otillräckligt material"),
    ("Threefold repetition", "Trefaldig upprepning"),
    ("A bishop is out of reach.", "En löpare går inte att nå."),
    ("Neither team can move.", "Inget lag kan flytta."),
    ("Sunflower Meadows", "Solrosängarna"),
    ("Checkmate Castle", "Schackmattslottet"),
    ("Adventure Galaxy", "Äventyrsgalaxen"),
    ("Galaxy", "Galaxen"),
    ("ADVENTURE GALAXY", "ÄVENTYRSGALAXEN"),
    ("World complete!", "Världen är klar!"),
    ("You completed", "Du klarade"),
    ("Back to Adventure Galaxy", "Till äventyrsgalaxen"),
    ("Unlock all levels", "Lås upp alla banor"),
    ("All levels unlocked", "Alla banor upplåsta"),
    ("All levels are now unlocked.", "Alla banor är nu upplåsta."),
    ("Hello, King!", "Hej kungen!"),
    ("Two Rooks Take Turns", "Två torn turas om"),
    ("The Pawn Guard", "Bondens vakt"),
    ("The Little Gate", "Den lilla grinden"),
    ("Queen and Rook", "Dam och torn"),
    ("A Knight Visits", "Springaren hälsar på"),
    ("The Bishop Bridge", "Löparbron"),
    ("Two Pawn Guards", "Två bondevakter"),
    ("Open the File", "Öppna linjen"),
    ("Rooks Make Room", "Tornen skapar plats"),
    ("Watch the Knight", "Se upp för springaren"),
    ("Meet the Bishop", "Möt löparen"),
    ("The Guarded Corner", "Det bevakade hörnet"),
    ("Three Pawn Steps", "Tre bondesteg"),
    ("The Royal Rook", "Kungens torn"),
    ("The Queen Team", "Damens lag"),
    ("Clear the Diagonal", "Rensa diagonalen"),
    ("Castle Helpers", "Slottets hjälpare"),
    ("The Royal Puzzle", "Kungens kluring"),
    ("Checkmate Celebration", "Mattkalaset"),
    ("The Checkmate Kingdom", "Schackmattriket"),
    ("Checkmate the king!", "Gör kungen schackmatt!"),
    ("CHECKMATE", "SCHACKMATT"),
    ("Check!", "Schack!"),
    ("Cover every escape", "Täck alla flyktvägar"),
    ("Checkmate!", "Schackmatt!"),
    ("Stalemate — try again!", "Patt — försök igen!"),
    (
        "The king needs a safe move.",
        "Kungen behöver ett säkert drag.",
    ),
    ("Check and no escape wins.", "Schack utan flyktväg vinner."),
    ("The king stays on the board.", "Kungen stannar på brädet."),
    (
        "Getting your adventure ready...",
        "Vi gör ditt äventyr redo...",
    ),
    (
        "Reach the far edge to become a queen!",
        "Nå andra kanten och bli en dam!",
    ),
    ("Far edge: a new queen!", "Andra kanten: en ny dam!"),
    ("Continue", "Fortsätt"),
    ("New game", "Nytt spel"),
    ("Quit", "Avsluta"),
    ("Start a new adventure?", "Starta ett nytt äventyr?"),
    (
        "This player's previous progress will be lost.",
        "Den här spelarens tidigare framsteg försvinner.",
    ),
    (
        "Other players and settings are kept.",
        "Andra spelare och inställningar finns kvar.",
    ),
    ("Yes, new game", "Ja, nytt spel"),
    ("No, go back", "Nej, gå tillbaka"),
    ("Main menu", "Huvudmeny"),
    ("World map", "Världskarta"),
    ("WORLD MAP", "VÄRLDSKARTA"),
    ("Choose your next adventure", "Välj ditt nästa äventyr"),
    ("Enter world", "Gå till världen"),
    (
        "More worlds will grow here!",
        "Fler världar växer fram här!",
    ),
    ("The First Adventure", "Det första äventyret"),
    ("A new queen!", "En ny dam!"),
    ("You finished this world!", "Du klarade den här världen!"),
    ("YOUR TEAM", "DITT LAG"),
    ("Choose a piece", "Välj en pjäs"),
    ("Then choose a square", "Välj sedan en ruta"),
    ("Watch the purple team", "Titta på det lila laget"),
    ("PAWN'S RULE", "BONDENS REGEL"),
    ("QUEEN'S RULE", "DAMENS REGEL"),
    ("Walk forward.", "Gå framåt."),
    ("Capture diagonally.", "Ta pjäser snett."),
    ("Queens move", "Damer går"),
    ("straight or diagonally.", "rakt eller snett."),
    ("Choose one of your pieces", "Välj en av dina pjäser"),
    ("Wait for your turn", "Vänta på din tur"),
    ("Capture their pieces!", "Ta motståndarens pjäser!"),
    ("FRIENDLY MATCH", "VÄNSKAPSMATCH"),
    ("Your turn", "Din tur"),
    ("Opponent's turn", "Motståndarens tur"),
    ("Good try!", "Bra försök!"),
    ("Start this level again.", "Börja om den här banan."),
    ("Your stars are safe.", "Dina stjärnor finns kvar."),
    ("Piece Garden", "Pjästrädgården"),
    ("Friendly Matches", "Vänskapsmatcher"),
    ("Team Adventures", "Lagäventyr"),
    ("Hello, Pawn!", "Hej, bonde!"),
    ("Pawn's Garden", "Bondens trädgård"),
    ("Hello, Queen!", "Hej, dam!"),
    ("Queen's Rainbow", "Damens regnbåge"),
    ("Our First Match", "Vår första match"),
    ("A Little Rook Rival", "En liten tornutmaning"),
    ("Four Friendly Helpers", "Fyra glada hjälpare"),
    ("A Diagonal Rival", "En sned utmaning"),
    ("The Queen Helps", "Damen hjälper till"),
    ("Hops Across the River", "Hopp över ån"),
    ("Teamwork in the Woods", "Samarbete i skogen"),
    ("Two Rooks Together", "Två torn tillsammans"),
    ("A Clever Little Team", "Ett klurigt litet lag"),
    ("The Friendly Final", "Den vänliga finalen"),
    ("ROOK RAINBOW QUEST", "ROOK RAINBOW QUEST"),
    ("Sound off", "Ljud av"),
    ("Sound on", "Ljud på"),
    ("Settings", "Inställningar"),
    (
        "Arrow keys: follow the path   |   Enter: play",
        "Piltangenter: följ stigen   |   Enter: spela",
    ),
    ("Let's play!", "Nu spelar vi!"),
    ("Play again", "Spela igen"),
    ("Sunshine", "Solstråle"),
    ("Moonbeam", "Månstråle"),
    ("ROOK'S RULE", "TORNETS REGEL"),
    ("BISHOP'S RULE", "LÖPARENS REGEL"),
    ("KNIGHT'S RULE", "SPRINGARENS REGEL"),
    ("Rooks move", "Torn går"),
    ("straight.", "rakt."),
    ("Bishops move", "Löpare går"),
    ("diagonally.", "snett."),
    ("Knights hop", "Springare hoppar"),
    ("in an L.", "som ett L."),
    ("Click a glowing square", "Klicka på en lysande ruta"),
    ("COLLECT STARS", "SAMLA STJÄRNOR"),
    ("Take your time!", "Ta det lugnt!"),
    ("Capture the pawn!", "Ta bonden!"),
    ("Hint", "Tips"),
    ("Try again", "Börja om"),
    ("Map", "Karta"),
    ("You did it!", "Du klarade det!"),
    (
        "A new stop is ready on your path!",
        "En ny bana väntar på dig!",
    ),
    (
        "You finished the whole adventure!",
        "Du klarade hela äventyret!",
    ),
    ("Back to map", "Till kartan"),
    ("Next level", "Nästa bana"),
    ("Make yourself at home", "Välkommen till ditt äventyr!"),
    ("YOUR PLAYER", "DIN SPELARE"),
    ("Sound: off", "Ljud: av"),
    ("Sound: on", "Ljud: på"),
    ("Gentle motion: on", "Lugna rörelser: på"),
    ("Gentle motion: off", "Lugna rörelser: av"),
    ("Softer", "Lägre"),
    ("Louder", "Högre"),
    ("Fullscreen", "Helskärm"),
    ("Language", "Språk"),
    (
        "M: sound   |   F11: fullscreen   |   Tab + Enter: buttons",
        "M: ljud   |   F11: helskärm   |   Tab + Enter: knappar",
    ),
    (
        "Audio unavailable - silent play is ready.",
        "Ljudet fungerar inte. Du kan spela ändå.",
    ),
    ("Back to adventure", "Till äventyret"),
    ("Exit game", "Avsluta spelet"),
    (
        "Couldn't save progress. You can still play!",
        "Kunde inte spara. Du kan spela ändå!",
    ),
    (
        "Couldn't read saved progress. Starting fresh.",
        "Kunde inte läsa sparningen. Vi börjar om.",
    ),
    (
        "Couldn't read saved progress. Playing without it.",
        "Kunde inte läsa sparningen. Du kan spela ändå.",
    ),
    (
        "Finish this level to open the next!",
        "Klara den här banan för att öppna nästa!",
    ),
    ("Try a glowing square", "Prova en lysande ruta"),
    (
        "Follow the path with the arrow keys",
        "Följ stigen med piltangenterna",
    ),
    ("Sunny Meadow", "Solängen"),
    ("Rainbow River", "Regnbågsån"),
    ("Blossom Woods", "Blomsterskogen"),
    ("Starry Summit", "Stjärntoppen"),
    ("Rook's Rainbow Trail", "Tornets regnbågsstig"),
    ("Up to the Clouds", "Upp till molnen"),
    ("The Other Way", "Åt andra hållet"),
    ("Flower Corners", "Blomsterhörnen"),
    ("Meadow Picnic", "Picknick på ängen"),
    ("Across the River", "Över ån"),
    ("Stepping Stones", "Stenar över vattnet"),
    ("Rook's Ribbon", "Tornets rosett"),
    ("Hello, Bishop!", "Hej, löpare!"),
    ("Butterfly Diagonals", "Fjärilens sneda stigar"),
    ("Blossom Bridges", "Blomsterbroar"),
    ("Petal Paths", "Blombladens stigar"),
    ("The Other Color", "Den andra färgen"),
    ("Bishop's Bouquet", "Löparens bukett"),
    ("Hello, Knight!", "Hej, springare!"),
    ("Two Little Hops", "Två små hopp"),
    ("Knight's Dance", "Springarens dans"),
    ("Mountain Hops", "Hopp i bergen"),
    ("Starry Stepping", "Steg bland stjärnorna"),
    ("Summit Celebration", "Fest på toppen"),
];

pub fn translate(language: Language, text: &str) -> String {
    if language == Language::English {
        return text.into();
    }
    if let Some((_, swedish)) = TEXTS.iter().find(|(english, _)| *english == text) {
        return (*swedish).into();
    }
    if let Some(n) = text
        .strip_prefix("Capture all ")
        .and_then(|s| s.strip_suffix(" pawns!"))
    {
        if n.parse::<usize>().is_ok() {
            return if n == "1" {
                "Ta bonden!".into()
            } else {
                format!("Ta alla {n} bönder!")
            };
        }
    }
    if let Some(n) = text.strip_prefix("Your team: ") {
        return format!("Ditt lag: {n}");
    }
    if let Some(volume) = text.strip_prefix("Volume: ") {
        return format!("Volym: {volume}");
    }
    if let Some((n, title)) = text.split_once(". ") {
        if n.parse::<usize>().is_ok() {
            return format!("{n}. {}", translate(language, title));
        }
    }
    // Shared captured-count and campaign progress formats contain only numeric slots.
    let tokens: Vec<_> = text.split_whitespace().collect();
    if tokens.len() == 3
        && tokens[1] == "of"
        && tokens[0].parse::<usize>().is_ok()
        && tokens[2].parse::<usize>().is_ok()
    {
        return format!("{} av {}", tokens[0], tokens[2]);
    }
    if tokens.len() == 9 && tokens[0] == "Level" && tokens[2] == "of" && tokens[6] == "stars" {
        let stars = if tokens[5] == "1" {
            "stjärna"
        } else {
            "stjärnor"
        };
        return format!(
            "Bana {} av {}   /   {} {stars}   /   {}",
            tokens[1],
            tokens[3],
            tokens[5],
            translate(language, tokens[8])
        );
    }
    // Native language names, numeric node labels and arrow symbols are identical.
    text.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_menu_worlds_and_promotion_are_localized() {
        for w in crate::worlds::load(&crate::campaign::levels()) {
            assert_ne!(translate(Language::Swedish, &w.title), w.title);
        }
        for key in [
            "Continue",
            "New game",
            "Quit",
            "World map",
            "Sunflower Meadows",
            "Checkmate Castle",
            "Adventure Galaxy",
            "World complete!",
            "KING'S RULE",
            "Choose your new piece",
            "FINAL BOSS — full game!",
            "CHESS MATCH",
            "Draw — try again!",
            "You completed",
            "Back to Adventure Galaxy",
            "A new queen!",
            "Yes, new game",
            "No, go back",
        ] {
            assert_ne!(translate(Language::Swedish, key), key);
            assert_eq!(translate(Language::English, key), key);
        }
    }
    #[test]
    fn every_campaign_title_and_world_has_a_swedish_translation() {
        for l in crate::campaign::levels() {
            assert_ne!(translate(Language::Swedish, &l.title), l.title);
            assert_ne!(translate(Language::Swedish, &l.world), l.world);
        }
    }
    #[test]
    fn dynamic_ui_text_is_localized_without_changing_numbers() {
        for (en, sv) in [
            ("Capture all 3 pawns!", "Ta alla 3 bönder!"),
            ("1 of 5", "1 av 5"),
            ("Your team: 3", "Ditt lag: 3"),
            ("Volume: 55%", "Volym: 55%"),
            ("2. Up to the Clouds", "2. Upp till molnen"),
            (
                "Level 2 of 20   /   1 stars   /   Sunshine",
                "Bana 2 av 20   /   1 stjärna   /   Solstråle",
            ),
        ] {
            assert_eq!(translate(Language::Swedish, en), sv);
            assert_eq!(translate(Language::English, en), en);
        }
    }
    #[test]
    fn catalog_is_unique_nonempty_and_supports_swedish_characters() {
        let mut seen = std::collections::HashSet::new();
        for (en, sv) in TEXTS {
            assert!(seen.insert(*en));
            assert!(!sv.is_empty());
            assert!(
                !sv.chars().any(|c| matches!(c, 'Ã' | 'Â' | 'ƒ' | '�')),
                "Corrupt Swedish: {sv}"
            );
            assert_eq!(translate(Language::Swedish, sv), *sv);
        }
        assert_eq!(translate(Language::Swedish, "Try again"), "Börja om");
        assert_eq!(translate(Language::Swedish, "Settings"), "Inställningar");
        assert_eq!(translate(Language::Swedish, "Continue"), "Fortsätt");
        assert_eq!(translate(Language::Swedish, "WORLD MAP"), "VÄRLDSKARTA");
    }
    #[test]
    fn switching_language_is_presentation_only() {
        let before = crate::campaign::levels();
        for l in &before {
            let _ = translate(Language::Swedish, &l.title);
            let _ = translate(Language::English, &l.title);
        }
        let after = crate::campaign::levels();
        assert_eq!(
            before.iter().map(|l| &l.id).collect::<Vec<_>>(),
            after.iter().map(|l| &l.id).collect::<Vec<_>>()
        );
    }
}
