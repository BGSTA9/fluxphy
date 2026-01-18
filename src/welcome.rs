//! Neofetch-style welcome screen for FluxPhy

use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, Stylize};
use crossterm::execute;
use std::io::{stdout, Write};

/// Theme Colors
const THEME_GREEN: Color = Color::Green;
const THEME_ORANGE: Color = Color::Rgb { r: 255, g: 165, b: 0 };

/// ASCII art logo parts (Green Part, Orange Part)
const LOGO_PARTS: [(&str, &str); 6] = [
    ("██████╗ ██╗     ██╗   ██╗ ██╗  ██╗", "  ██████╗ "),
    ("██╔═══╝ ██║     ██║   ██║ ╚██╗██╔╝", " ██╔═══██╗"),
    ("█████╗  ██║     ██║   ██║  ╚███╔╝ ", "██████████║"),
    ("██╔══╝  ██║     ██║   ██║  ██╔██╗ ", " ██╔═══██║"),
    ("██║     ███████╗╚██████╔╝ ██╔╝ ██╗", " ╚██████╔╝"),
    ("╚═╝     ╚══════╝ ╚═════╝  ╚═╝  ╚═╝", "  ╚═════╝ "),
];

/// GitHub Repository URL
const GITHUB_URL: &str = "https://github.com/BGSTA9/fluxphy";

/// Author Name
const AU: &str = "Argo Navis Research Laboratory";

/// Info lines to display (Introduction + Metadata)
/// Returns: (Color, Label, Value, IsItalic)
fn get_info_lines() -> Vec<(Color, String, String, bool)> {
    let mut lines = Vec::new();

    // Introduction paragraph
    let intro_lines = vec![
        "FluxPhy  defines  a  new  standard  in  file",
        "transfer by modeling data flux as a physical",
        "fluid  dynamic  process.  It  combines high-",
        "performance  I/O  with  real-time    physics",
        "instrumentation   to   ensure     stability,",
        "efficiency, and reproducibility.",
    ];

    for line in intro_lines {
        // Use DarkGrey and Italic for "typewriter" look
        lines.push((Color::DarkGrey, "".to_string(), line.to_string(), true));
    }

    // Spacer
    lines.push((Color::White, "".to_string(), "".to_string(), false));
    lines.push((Color::DarkGrey, "".to_string(), "─".repeat(44), false));
    lines.push((Color::White, "".to_string(), "".to_string(), false));

    // Metadata
    lines.push((THEME_ORANGE, "Version".to_string(), format!("v{}", env!("CARGO_PKG_VERSION")), true));
    lines.push((THEME_GREEN, "Author".to_string(), AUTHOR.to_string(), true));
    lines.push((Color::Blue, "GitHub".to_string(), GITHUB_URL.to_string(), true));

    lines
}

/// Display the welcome screen (Vertical Layout)
pub fn display_welcome() {
    let mut stdout = stdout();
    
    println!();
    
    // 1. Print Logo (Centered-ish or just left aligned)
    for (part1, part2) in LOGO_PARTS.iter() {
        // Print first part in Green
        execute!(
            stdout,
            SetForegroundColor(THEME_GREEN),
            Print(part1),
            ResetColor,
        ).ok();

        // Print second part in Orange
        execute!(
            stdout,
            SetForegroundColor(THEME_ORANGE),
            Print(part2),
            ResetColor,
        ).ok();
        
        println!();
    }
    
    println!();

    // 2. Print Info Lines (Introduction + Metadata)
    let info_lines = get_info_lines();
    
    for (color, label, value, is_italic) in info_lines {
        // Text is flush-left to match logo alignment 
        
        if !label.is_empty() {
            let mut label_styled = label.clone().with(color);
            let mut value_styled = value.clone().with(Color::White);
            let mut sep = ": ".to_string().stylize();
            
            if is_italic {
                label_styled = label_styled.italic();
                value_styled = value_styled.italic();
                sep = sep.italic();
            }
            
            print!("{}{}{}", label_styled, sep, value_styled);
        } else {
            let mut value_styled = value.clone().with(color);
            if is_italic {
                value_styled = value_styled.italic();
            }
            print!("{}", value_styled);
        }
        println!();
    }
    
    println!();
    
    stdout.flush().ok();
}

/// Display welcome screen with optional delay
pub fn display_welcome_with_delay(delay_ms: u64) {
    display_welcome();
    if delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    }
}
