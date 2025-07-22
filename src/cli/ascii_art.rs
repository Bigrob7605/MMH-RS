use std::collections::HashMap;
use rand::Rng;

/// MMH-RS ASCII Art Collection
/// Each ASCII set is carefully crafted for clarity, style, and a sense of legendary importance.
pub struct AsciiArt {
    arts: HashMap<String, String>,
}

impl AsciiArt {
    pub fn new() -> Self {
        let mut arts = HashMap::new();
        
        // 🧬 1. Digital DNA - Universal Format
        arts.insert("digital_dna".to_string(), r#"
  ┌─────────────────────┐
  │ ┌──┐ ┌──┐ ┌──┐ ┌──┐ │
  │ │▒▒│ │▒▒│ │▒▒│ │▒▒│ │
  │ └──┘ └──┘ └──┘ └──┘ │
  │   M M H - R S       │
  └─────────────────────┘
     Digital DNA Format"#.to_string());

        // 💎 2. Crystal Seed
        arts.insert("crystal_seed".to_string(), r#"
         /\
        /  \
       / /\ \
      / /  \ \
     /_/____\_\
     \        /
      \ MMH  /
       \ RS /
        \  /
         \/
"One Seed, Infinite Data""#.to_string());

        // 🌐 3. Infinity Loop (Folding Forever)
        arts.insert("infinity_loop".to_string(), r#"
     ┌───────┐
 ┌───┘       └───┐
 │   M M H - R S │
 └───┐       ┌───┘
     └───────┘
"Fold. Restore. Repeat.""#.to_string());

        // ⚙️ 4. Compression Gear
        arts.insert("compression_gear".to_string(), r#"
     ┌─────────┐
 ┌───┤ ╔═════╗ ├───┐
 │   │ ║ MMH ║ │   │
 │   │ ║  RS ║ │   │
 └───┤ ╚═════╝ ├───┘
     └─────────┘
"Precision Compression""#.to_string());

        // 🧙 5. The Gandalf Meme (Humor/Easter Egg)
        arts.insert("gandalf".to_string(), r#"
      🧙
     (•_•)
    <)   )╯  mmh fold world/
     /    \
"YOU SHALL NOT LOSE DATA!""#.to_string());

        // 🔒 6. Fortress of Integrity
        arts.insert("integrity_fortress".to_string(), r#"
     |￣￣￣￣￣￣￣|
     |   MMH-RS  |
     |___________|
     |  _____    |
     | |     |   |
     | |DATA |   |
     | |SAFE |   |
     | |_____|   |
     |___________|
"Data Integrity Fortress""#.to_string());

        // 🌳 7. Merkle Tree
        arts.insert("merkle_tree".to_string(), r#"
       ┌───┐
       │ R │
       └─┬─┘
      ┌──┴──┐
    ┌─┴─┐  ┌┴──┐
    │ L │  │ R │
    └─┬─┘  └─┬─┘
      │      │
  ┌───┴─┐  ┌─┴───┐
  │MMH  │  │ RS  │
  └─────┘  └─────┘
"Merkle Integrity Tree""#.to_string());

        // 🧩 8. Puzzle Piece (Universal Fit)
        arts.insert("universal_fit".to_string(), r#"
   ┌───────────┐
 ┌─┤  M M H    ├─┐
 │ └─────┬─────┘ │
 │       │       │
 │  R S  │       │
 │       │       │
 └───────┴───────┘
 "One Format Fits All""#.to_string());

        Self { arts }
    }

    /// Display a specific ASCII art by name
    pub fn display(&self, name: &str) {
        if let Some(art) = self.arts.get(name) {
            println!("{}", art);
        } else {
            println!("ASCII art '{}' not found", name);
        }
    }

    /// Display a random ASCII art
    pub fn display_random(&self) {
        let mut rng = rand::thread_rng();
        let art_names: Vec<&String> = self.arts.keys().collect();
        let random_name = art_names[rng.gen_range(0..art_names.len())];
        self.display(random_name);
    }

    /// Display the menu of available ASCII arts
    pub fn display_menu(&self) {
        println!("{}", "=".repeat(50));
        println!("          MMH-RS Universal Storage");
        println!("{}", "=".repeat(50));
        println!();

        let menu_items = vec![
            ("digital_dna", "🧬 Digital DNA"),
            ("crystal_seed", "💎 Crystal Seed"),
            ("infinity_loop", "🌐 Infinity Loop"),
            ("compression_gear", "⚙️ Compression Gear"),
            ("gandalf", "🧙 Gandalf Easter Egg"),
            ("integrity_fortress", "🔒 Integrity Fortress"),
            ("merkle_tree", "🌳 Merkle Tree"),
            ("universal_fit", "🧩 Universal Fit"),
        ];

        for (i, (key, description)) in menu_items.iter().enumerate() {
            println!("[{}] {}", i + 1, description);
        }

        println!();
        println!("Choose your destiny [1-8]:");
    }

    /// Display ASCII art based on command context
    pub fn display_for_command(&self, command: &str) {
        match command {
            "pack" => self.display("digital_dna"),
            "unpack" => self.display("infinity_loop"),
            "verify" => self.display("integrity_fortress"),
            "gen" => self.display("crystal_seed"),
            "wizard" => self.display("gandalf"),
            _ => self.display_random(),
        }
    }

    /// Display startup ASCII art
    pub fn display_startup(&self) {
        println!("{}", "=".repeat(50));
        println!("          MMH-RS Universal Storage");
        println!("{}", "=".repeat(50));
        println!();
        self.display("digital_dna");
        println!();
        println!("🧬 Universal Digital DNA Format");
        println!("One format. Every file. Every platform. Forever.");
        println!();
    }

    /// Display completion ASCII art
    pub fn display_completion(&self) {
        println!();
        self.display("universal_fit");
        println!();
        println!("✅ Operation completed successfully!");
        println!("🧬 Your data is now immortal.");
        println!();
    }

    /// Display error ASCII art
    pub fn display_error(&self) {
        println!();
        self.display("integrity_fortress");
        println!();
        println!("❌ Operation failed!");
        println!("🔒 Data integrity compromised.");
        println!("Please check your inputs and try again.");
        println!();
    }

    /// Display all ASCII arts (for testing)
    pub fn display_all(&self) {
        for (name, art) in &self.arts {
            println!("=== {} ===", name.to_uppercase());
            println!("{}", art);
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_art_creation() {
        let art = AsciiArt::new();
        assert_eq!(art.arts.len(), 8);
    }

    #[test]
    fn test_ascii_art_display() {
        let art = AsciiArt::new();
        art.display("digital_dna");
        // Should not panic
    }

    #[test]
    fn test_ascii_art_menu() {
        let art = AsciiArt::new();
        art.display_menu();
        // Should not panic
    }

    #[test]
    fn test_ascii_art_command_context() {
        let art = AsciiArt::new();
        art.display_for_command("pack");
        art.display_for_command("unpack");
        art.display_for_command("verify");
        art.display_for_command("wizard");
        // Should not panic
    }
} 