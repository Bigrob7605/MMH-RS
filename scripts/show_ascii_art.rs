#!/usr/bin/env rust
//! MMH-RS ASCII Art Showcase
//! 
//! This script displays all the legendary ASCII art designs for MMH-RS.

use std::collections::HashMap;

/// MMH-RS ASCII Art Collection
/// Each ASCII set is carefully crafted for clarity, style, and a sense of legendary importance.
struct AsciiArt {
    arts: HashMap<String, String>,
}

impl AsciiArt {
    fn new() -> Self {
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
    fn display(&self, name: &str) {
        if let Some(art) = self.arts.get(name) {
            println!("{}", art);
        } else {
            println!("ASCII art '{}' not found", name);
        }
    }

    /// Display a specific ASCII art by index
    fn display_by_index(&self, index: usize) {
        let art_names = vec![
            "digital_dna",
            "crystal_seed", 
            "infinity_loop",
            "compression_gear",
            "gandalf",
            "integrity_fortress",
            "merkle_tree",
            "universal_fit",
        ];
        
        if index < art_names.len() {
            self.display(art_names[index]);
        } else {
            println!("ASCII art index {} not found", index);
        }
    }

    /// Display the menu of available ASCII arts
    fn display_menu(&self) {
        println!("{}", "=".repeat(50));
        println!("          MMH-RS Universal Storage");
        println!("{}", "=".repeat(50));
        println!();

        let menu_items = vec![
            "🧬 Digital DNA",
            "💎 Crystal Seed",
            "🌐 Infinity Loop",
            "⚙️ Compression Gear",
            "🧙 Gandalf Easter Egg",
            "🔒 Integrity Fortress",
            "🌳 Merkle Tree",
            "🧩 Universal Fit",
        ];

        for (i, description) in menu_items.iter().enumerate() {
            println!("[{}] {}", i + 1, description);
        }

        println!();
        println!("Choose your destiny [1-8]:");
    }

    /// Display ASCII art based on command context
    fn display_for_command(&self, command: &str) {
        match command {
            "pack" => self.display("digital_dna"),
            "unpack" => self.display("infinity_loop"),
            "verify" => self.display("integrity_fortress"),
            "gen" => self.display("crystal_seed"),
            "wizard" => self.display("gandalf"),
            _ => self.display("digital_dna"), // Default to Digital DNA
        }
    }

    /// Display startup ASCII art
    fn display_startup(&self) {
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
    fn display_completion(&self) {
        println!();
        self.display("universal_fit");
        println!();
        println!("✅ Operation completed successfully!");
        println!("🧬 Your data is now immortal.");
        println!();
    }

    /// Display error ASCII art
    fn display_error(&self) {
        println!();
        self.display("integrity_fortress");
        println!();
        println!("❌ Operation failed!");
        println!("🔒 Data integrity compromised.");
        println!("Please check your inputs and try again.");
        println!();
    }

    /// Display all ASCII arts (for testing)
    fn display_all(&self) {
        for (name, art) in &self.arts {
            println!("=== {} ===", name.to_uppercase());
            println!("{}", art);
            println!();
        }
    }
}

fn main() {
    let ascii_art = AsciiArt::new();
    
    println!("🧬 MMH-RS ASCII Art Showcase");
    println!("=============================");
    println!();
    
    // Display startup
    ascii_art.display_startup();
    
    // Display all ASCII arts
    println!("🎨 All ASCII Art Designs:");
    println!("{}", "=".repeat(50));
    ascii_art.display_all();
    
    // Display menu
    println!("📋 ASCII Art Menu:");
    ascii_art.display_menu();
    
    // Show command-specific examples
    println!();
    println!("🔧 Command-Specific ASCII Art:");
    println!("{}", "=".repeat(50));
    
    println!("📦 Pack Command:");
    ascii_art.display_for_command("pack");
    
    println!();
    println!("🔄 Unpack Command:");
    ascii_art.display_for_command("unpack");
    
    println!();
    println!("🔍 Verify Command:");
    ascii_art.display_for_command("verify");
    
    println!();
    println!("💎 Generate Command:");
    ascii_art.display_for_command("gen");
    
    println!();
    println!("🧙 Wizard Mode (Easter Egg):");
    ascii_art.display_for_command("wizard");
    
    // Show completion and error states
    println!();
    println!("✅ Completion State:");
    ascii_art.display_completion();
    
    println!();
    println!("❌ Error State:");
    ascii_art.display_error();
    
    // Show a specific ASCII art
    println!();
    println!("🎲 Crystal Seed (Index 1):");
    ascii_art.display_by_index(1);
    
    println!();
    println!("🚀 Ready for world domination!");
    println!("The Universal Digital DNA Format is now legendary.");
} 