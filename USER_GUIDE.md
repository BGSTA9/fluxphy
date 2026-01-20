# FluxPhy User Guide 📘

Welcome to **FluxPhy**, the file transfer tool that treats data as a physical fluid. This guide is your complete reference for mastering the tool, from basic copying to advanced physics-based optimization.

---

## 🚀 Quick Start

### Basic Copy
Copy a file to a destination:
```bash
fluxphy movie.mkv /Volumes/Backup/
```

### Recursive Copy
Copy an entire directory:
```bash
fluxphy /path/to/source_folder /path/to/destination/ -r
```

---

## 📖 Command Reference

### Arguments
`fluxphy [SOURCES]... [DESTINATION]`
- **SOURCES**: One or more files or directories to copy.
- **DESTINATION**: The final argument. Must be a directory if copying multiple sources.

### Flags & Options

#### General
| Flag | Short | Description |
|------|-------|-------------|
| `--recursive` | `-r` | Required for copying directories. |
| `--force` | `-f` | Overwrite existing files at the destination without prompting. |
| `--verify` | | Calculate SHA-256 checksums after transfer to ensure data integrity. Adds processing time but guarantees safety. |
| `--help` | `-h` | Print help information. |
| `--version` | `-V` | Print version information. |
| `--fetch` | | Display system information in a "neofetch" style. |
| `--welcome` | | Display the introductory welcome screen. |

#### Interface & Output
| Flag | Short | Description |
|------|-------|-------------|
| `--quiet` | `-q` | **Headless Mode**. Disables the TUI. Useful for scripts and cron jobs. Shows a simple progress bar. |
| `--color` | | specific color output mode: `auto`, `always`, or `never`. Default is `auto`. |
| `--analyze` | `-a` | Enable detailed post-transfer analysis reporting. |
| `--physics-verbose` | | Enable verbose logging of physics metrics (thermal stability, entropy) to stdout/stderr. |
| `--metrics-file <FILE>` | | Save JSON metrics to a specific file path instead of the default `fluxphy_metrics_<timestamp>.json`. |

#### ⚡ Performance Tuning
Use these flags to optimize FluxPhy for your specific hardware.

| Flag | Default | Description |
|------|---------|-------------|
| `--buffer-size <MB>` | `8` | Size of the memory buffer in Megabytes. <br>• **Increase (e.g., 64, 128)**: For very fast massive files (NVMe SSDs). Reduces system call overhead.<br>• **Decrease**: For low-memory systems. |
| `--sample-rate <MS>` | `100` | How often (in milliseconds) to update metrics and UI.<br>• **Higher (e.g., 500)**: Reduces CPU usage, slightly better transfer speeds on weak CPUs.<br>• **Lower (e.g., 10)**: Extremely smooth graphs, higher CPU usage. |

**Example - High Performance Copy:**
```bash
# Optimized for NVMe to NVMe transfer of a large ISO
fluxphy large_game.iso /mnt/games/ --buffer-size 64
```

---

## 🖥️ The Graphical Interface (TUI)

When running without `--quiet`, you enter the Interactive Physics Interface.

### Keyboard Controls
| Key | Action |
|-----|--------|
| **`Q`** | **Quit**. Abort the transfer immediately. |
| **`P`** | **Pause**. Temporarily halt the flow. |
| **`R`** | **Resume**. Continue the flow from where it left off. |
| **`S`** | **Snapshot**. Generate a static HTML dashboard of the current metrics. |
| **`H`** | **Help**. Toggle the help overlay. |

### Understanding the Metrics
- **Flow Regime**:
    - 🟢 **Laminar**: Smooth, efficient transfer.
    - 🔴 **Turbulent**: Variable speed. Indicates bottlenecks (disk seeking, CPU limits).
- **Thermal Stability**: A metaphor for consistency. High stability means predictable performance.
- **IOPS**: Estimated Input/Output Operations Per Second.

---

## 🔬 Advanced Usage

### Scientific Verification & Provenance
For archival or research data, FluxPhy automatically generates a `provenance.json` file at the destination. This file adheres to W3C standards and records:
- Who moved the data (Agent).
- When it was moved (Activity).
- The "Physics" of the transfer (Flow Regime).

Combine with `--verify` for maximum confidence:
```bash
fluxphy /data/experiment_results /archive/ -r --verify
```

### Scripting & Automation
For automated backups, use quiet mode and redirect output if needed:
```bash
fluxphy /important/data /backup/drive -r --quiet --force --metrics-file /var/log/fluxphy/daily.json
```

### Troubleshooting
- **"Turbulent" Flow**: If your graph is jagged:
    1. Check for other disk-heavy processes.
    2. If moving many small files, this is normal (file creation overhead).
    3. Try increasing `--buffer-size` if moving large files.
