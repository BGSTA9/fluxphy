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

use sysinfo::System;

/// Info lines to display on the right
fn get_info_lines() -> Vec<(Color, String, String)> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or("Unknown".to_string());
    let os_version = System::os_version().unwrap_or("".to_string());
    let host_name = System::host_name().unwrap_or("localhost".to_string());
    let uptime = System::uptime();
    let uptime_hours = uptime / 3600;
    let uptime_mins = (uptime % 3600) / 60;
    
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    
    // CPU info
    let cpu_brand = sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or("Unknown CPU".to_string());
    let cpu_usage = sys.global_cpu_usage();

    vec![
        (Color::Cyan, "FluxPhy".to_string(), format!("v{}", env!("CARGO_PKG_VERSION"))),
        (Color::White, "".to_string(), "─".repeat(30)),
        (Color::Green, "User".to_string(), format!("{}@{}", whoami::username().unwrap_or("user".to_string()), host_name)),
        (Color::Yellow, "OS".to_string(), format!("{} {}", os_name, os_version)),
        (Color::Blue, "Kernel".to_string(), System::kernel_version().unwrap_or("unknown".to_string())),
        (Color::Magenta, "Uptime".to_string(), format!("{}h {}m", uptime_hours, uptime_mins)),
        (Color::Red, "CPU".to_string(), format!("{} ({:.1}%)", cpu_brand.trim(), cpu_usage)),
        (Color::Rgb { r: 255, g: 100, b: 0 }, "Memory".to_string(), format!("{:.2} GiB / {:.2} GiB", used_mem, total_mem)),
        (Color::White, "".to_string(), "".to_string()),
        (Color::DarkGrey, "Terminal".to_string(), std::env::var("TERM").unwrap_or("unknown".to_string())),
        (Color::DarkGrey, "Shell".to_string(), std::env::var("SHELL").unwrap_or("unknown".to_string())),
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
