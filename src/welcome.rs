//! Neofetch-style welcome screen for FluxPhy
//!
//! Provides two modes:
//! 1. `display_welcome`: The "Intro" screen with typewriter text (for first run/TUI start).
//! 2. `display_fetch`: The "Neofetch" screen with system stats (for --fetch).

use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::execute;
use std::env;
use std::io::{stdout, Write};
use std::process::Command;
use sysinfo::System;

/// Theme Colors
const THEME_GREEN: Color = Color::Rgb { r: 135, g: 242, b: 165 }; // #87f2a5
const THEME_ORANGE: Color = Color::Rgb { r: 255, g: 160, b: 65 }; // #ffa041
const THEME_DIM_GREEN: Color = Color::Rgb { r: 80, g: 160, b: 100 }; // #50a064

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
const AUTHOR: &str = "Argo Navis Research Laboratory";
const LICENSE: &str = "MIT";

// --- Helper Functions to Gather Info ---

fn get_os_info() -> String {
    if cfg!(target_os = "macos") {
        let output = Command::new("sw_vers")
            .arg("-productName")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "macOS".to_string());
        
        let version = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
            
        format!("{} {}", output, version)
    } else {
        System::name().unwrap_or_else(|| "Unknown".to_string()) + " " + &System::os_version().unwrap_or_default()
    }
}

fn get_kernel() -> String {
     System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
}

fn get_cpu_model() -> String {
    // Try sysinfo first
    let mut sys = System::new();
    sys.refresh_cpu_all();
    let cpu = sys.cpus().first();
    
    if let Some(c) = cpu {
        return c.brand().trim().to_string();
    }
    
    // Fallback for macOS if sysinfo fails to get brand string cleanly
    if cfg!(target_os = "macos") {
         return Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout)
                .replace("(R)", "")
                .replace("(TM)", "")
                .trim()
                .to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
    }
    
    "Unknown CPU".to_string()
}

fn get_gpu_model() -> String {
    if cfg!(target_os = "macos") {
         Command::new("sh")
            .arg("-c")
            .arg("system_profiler SPDisplaysDataType 2>/dev/null | grep 'Chipset Model' | head -1 | sed 's/.*: //'")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "N/A".to_string())
    } else {
        // Fallback or Linux/Windows implementation
        "N/A".to_string()
    }
}

fn get_ram_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();
    let total_gb = sys.total_memory() / 1024 / 1024 / 1024;
    let used_gb = sys.used_memory() / 1024 / 1024 / 1024;
    
    // On macOS sysinfo used_memory might not match 'active' pages logic of the script, but it's close enough standard
    format!("{}GB / {}GB", used_gb, total_gb)
}

fn get_shell() -> String {
    env::var("SHELL").ok()
        .map(|s| s.split('/').last().unwrap_or(&s).to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_terminal() -> String {
    env::var("TERM_PROGRAM").or_else(|_| env::var("TERM"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_uptime_pretty() -> String {
    let uptime = System::uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let mins = (uptime % 3600) / 60;
    
    if days > 0 {
        format!("{} days, {} hours, {} mins", days, hours, mins)
    } else if hours > 0 {
        format!("{} hours, {} mins", hours, mins)
    } else {
        format!("{} mins", mins)
    }
}


// --- Display Functions ---

/// Display the "Neofetch" style screen with system stats
pub fn display_fetch() {
    let mut stdout = stdout();
    
    // Gather Data
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    // We will handle status color printing manually in the loop
    let status_text = "Installed ✓"; 
    
    let info_rows = vec![
        ("VERSION", version),
        ("STATUS", status_text.to_string()), 
        ("AUTHOR", AUTHOR.to_string()),
        ("GITHUB", GITHUB_URL.to_string()),
        ("LICENSE", LICENSE.to_string()),
        ("SEP", "".to_string()), // Horizontal rule placeholder
        ("OS", get_os_info()),
        ("KERNEL", get_kernel()),
        ("CPU", get_cpu_model()),
        ("GPU", get_gpu_model()),
        ("RAM", get_ram_info()),
        ("SHELL", get_shell()),
        ("TERMINAL", get_terminal()),
        ("UPTIME", get_uptime_pretty()),
    ];

    println!();

    // Iterate through lines. 
    // If we have logo lines, print logo part | info
    // If we run out of logo lines, print spaces | info
    
    let max_lines = std::cmp::max(LOGO_PARTS.len(), info_rows.len() + 3); // +3 for footer padding
    
    for i in 0..max_lines {
        // 1. Draw Logo Part (or padding)
        if i < LOGO_PARTS.len() {
            let (part1, part2) = LOGO_PARTS[i];
            execute!(stdout, SetForegroundColor(THEME_GREEN), Print(part1), ResetColor).ok();
            execute!(stdout, SetForegroundColor(THEME_ORANGE), Print(part2), ResetColor).ok();
        } else {
            // Padding length matches logo width roughly (30 chars green + 10 orange approx)
            // Logo is constant width: 
            // P1: "██████╗ ██╗     ██╗   ██╗ ██╗  ██╗" (34 chars wide visually?)
            // Actually let's count spaces.
            // P1 length is 30 unicode chars + color codes. 
            // Let's use a fixed padding string
            print!("                                            "); 
        }
        
        print!("   "); // Spacer between logo and info

        // 2. Draw Info Row
        if i < info_rows.len() {
            let (label, value) = &info_rows[i];
            
            if *label == "SEP" {
                 execute!(stdout, SetForegroundColor(THEME_ORANGE), Print("────────────────────────────────────────────────"), ResetColor).ok();
            } else {
                 execute!(stdout, SetForegroundColor(THEME_GREEN), Print(format!("{:>8}", label)), ResetColor).ok();
                 print!(" | ");
                 
                 if *label == "STATUS" {
                     execute!(stdout, SetForegroundColor(THEME_GREEN), Print(value), ResetColor).ok();
                 } else {
                     execute!(stdout, SetForegroundColor(Color::White), Print(value), ResetColor).ok();
                 }
            }
        } 
        // 3. Draw Footer lines (after info rows)
        else if i == info_rows.len() {
             execute!(stdout, SetForegroundColor(THEME_ORANGE), Print("────────────────────────────────────────────────"), ResetColor).ok();
        }
        else if i == info_rows.len() + 1 {
             // 📊 Real-time throughput    ⚛️ Physics metrics
             print!("📊 Real-time throughput    ⚛️ Physics metrics");
        }
        else if i == info_rows.len() + 2 {
            // 📚 Help panel [H]          💾 Save dashboard [S]
             print!("📚 Help panel [H]          💾 Save dashboard [S]");
        }
        
        println!();
    }
    
    println!();
    stdout.flush().ok();
}

/// Display the Intro screen (Typewriter text) - for first run / TUI start
pub fn display_welcome() {
    let mut stdout = stdout();
    
    println!();
    
    // Print Logo Stacked
    for (part1, part2) in LOGO_PARTS.iter() {
        execute!(stdout, SetForegroundColor(THEME_GREEN), Print(part1), ResetColor).ok();
        execute!(stdout, SetForegroundColor(THEME_ORANGE), Print(part2), ResetColor).ok();
        println!();
    }
    
    println!();

    // Intro Text
    let intro_lines = vec![
        "FluxPhy  defines  a  new  standard  in  file",
        "transfer by modeling data flux as a physical",
        "fluid  dynamic  process.  It  combines high-",
        "performance  I/O  with  real-time    physics",
        "instrumentation   to   ensure     stability,",
        "efficiency, and reproducibility.",
    ];

    for line in intro_lines {
        execute!(stdout, SetForegroundColor(Color::DarkGrey)).ok();
        // Italic if terminal supports it
        print!("{}", line); 
        execute!(stdout, ResetColor).ok();
        println!();
    }
    
    println!();
    execute!(stdout, SetForegroundColor(THEME_ORANGE), Print(format!("Version: v{}", env!("CARGO_PKG_VERSION"))), ResetColor).ok();
    println!();
    execute!(stdout, SetForegroundColor(THEME_GREEN), Print(format!("Author: {}", AUTHOR)), ResetColor).ok();
    println!();
    execute!(stdout, SetForegroundColor(Color::Blue), Print(format!("GitHub: {}", GITHUB_URL)), ResetColor).ok();
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
