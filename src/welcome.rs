//! Neofetch-style welcome screen for FluxPhy

use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, Stylize};
use crossterm::{execute, cursor, terminal};
use std::io::{stdout, Write};

/// ANSI color gradient for the logo (cyan to magenta)
const LOGO_COLORS: [Color; 6] = [
    Color::Rgb { r: 0, g: 255, b: 255 },   // Cyan
    Color::Rgb { r: 50, g: 200, b: 255 },  // Light blue
    Color::Rgb { r: 100, g: 150, b: 255 }, // Blue
    Color::Rgb { r: 150, g: 100, b: 255 }, // Purple
    Color::Rgb { r: 200, g: 50, b: 255 },  // Magenta
    Color::Rgb { r: 255, g: 0, b: 200 },   // Pink
];

/// ASCII art logo lines
const LOGO_LINES: [&str; 7] = [
    r"  _____ _            ____  _           ",
    r" |  ___| |_   ___  _|  _ \| |__  _   _ ",
    r" | |_  | | | | \ \/ / |_) | '_ \| | | |",
    r" |  _| | | |_| |>  <|  __/| | | | |_| |",
    r" |_|   |_|\__,_/_/\_\_|   |_| |_|\__, |",
    r"                                 |___/ ",
    r"                                       ",
];

/// Info lines to display on the right
fn get_info_lines() -> Vec<(Color, String, String)> {
    vec![
        (Color::Cyan, "FluxPhy".to_string(), format!("v{}", env!("CARGO_PKG_VERSION"))),
        (Color::White, "".to_string(), "─".repeat(25)),
        (Color::Yellow, "Author".to_string(), "Argo Navis Research Lab".to_string()),
        (Color::Green, "License".to_string(), "MIT".to_string()),
        (Color::White, "".to_string(), "".to_string()),
        (Color::Rgb { r: 0, g: 255, b: 136 }, "📊".to_string(), "Real-time throughput graphs".to_string()),
        (Color::Rgb { r: 0, g: 212, b: 255 }, "⚛️ ".to_string(), "Physics-inspired metrics".to_string()),
        (Color::Rgb { r: 255, g: 100, b: 100 }, "🔍".to_string(), "Bottleneck detection".to_string()),
        (Color::Rgb { r: 255, g: 204, b: 0 }, "📈".to_string(), "HTML dashboard export".to_string()),
        (Color::Rgb { r: 200, g: 100, b: 255 }, "📚".to_string(), "Dummy-friendly help panel".to_string()),
        (Color::White, "".to_string(), "".to_string()),
        (Color::DarkGrey, "Keys".to_string(), "[H] Help  [S] Dashboard  [Q] Quit".to_string()),
    ]
}

/// Display the neofetch-style welcome screen
pub fn display_welcome() {
    let mut stdout = stdout();
    let info_lines = get_info_lines();
    
    println!();
    
    for (i, logo_line) in LOGO_LINES.iter().enumerate() {
        let color = LOGO_COLORS[i % LOGO_COLORS.len()];
        
        // Print colored logo
        execute!(
            stdout,
            SetForegroundColor(color),
            Print(logo_line),
            ResetColor,
        ).ok();
        
        // Print separator
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(" │ "),
            ResetColor,
        ).ok();
        
        // Print info if available
        if let Some((info_color, label, value)) = info_lines.get(i) {
            if !label.is_empty() {
                execute!(
                    stdout,
                    SetForegroundColor(*info_color),
                    Print(label),
                    ResetColor,
                    Print(": "),
                    SetForegroundColor(Color::White),
                    Print(value),
                    ResetColor,
                ).ok();
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(*info_color),
                    Print(value),
                    ResetColor,
                ).ok();
            }
        }
        
        println!();
    }
    
    // Print remaining info lines
    let logo_width = LOGO_LINES[0].len();
    for i in LOGO_LINES.len()..info_lines.len() {
        if let Some((info_color, label, value)) = info_lines.get(i) {
            print!("{}", " ".repeat(logo_width));
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(" │ "),
                ResetColor,
            ).ok();
            
            if !label.is_empty() {
                execute!(
                    stdout,
                    SetForegroundColor(*info_color),
                    Print(label),
                    ResetColor,
                    Print(": "),
                    SetForegroundColor(Color::White),
                    Print(value),
                    ResetColor,
                ).ok();
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(*info_color),
                    Print(value),
                    ResetColor,
                ).ok();
            }
            println!();
        }
    }
    
    // Color palette display
    println!();
    print!("{}", " ".repeat(logo_width / 2));
    for color in &LOGO_COLORS {
        execute!(
            stdout,
            SetForegroundColor(*color),
            Print("███"),
            ResetColor,
        ).ok();
    }
    println!();
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
