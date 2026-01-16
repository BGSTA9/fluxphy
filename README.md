# FluxPhy - Physics of Flux File Transfer Tool

<p align="center">
  <strong>A file transfer tool with deep instrumentation into the physics of data flux</strong>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#physics-metrics">Physics Metrics</a> •
  <a href="#contributing">Contributing</a>
</p>

## Features

- 🚀 **High-Performance**: Optimized buffered I/O with adaptive sizing
- 📊 **Real-Time Visualization**: Live TUI with plotting of transfer rates
- 🔬 **Physics-Inspired Metrics**: Thermal stability, flux density, entropy analysis
- 📈 **Comprehensive Analytics**: Statistical analysis and bottleneck detection
- 🎯 **Cross-Platform**: Works on Linux, macOS, and Windows
- 💾 **Metrics Export**: Save detailed JSON metrics for every transfer
- 🔍 **Checksum Verification**: Optional SHA-256 verification

## Installation

### Via Cargo (Recommended)
```bash
cargo install fluxphy
```

### Via Homebrew (macOS/Linux)
```bash
brew install fluxphy
```

### Via Package Managers

**Arch Linux (AUR)**:
```bash
paru -S fluxphy
```

**Debian/Ubuntu**:
```bash
sudo apt install fluxphy
```

**Fedora/RHEL**:
```bash
sudo dnf install fluxphy
```

**Windows (WinGet)**:
```bash
winget install fluxphy
```

### Pre-built Binaries
Download from [GitHub Releases](https://github.com/BGSTA9/fluxphy/releases)

### Build from Source
```bash
git clone https://github.com/BGSTA9/fluxphy
cd fluxphy
make
sudo make install
```

## Quick Start

```bash
# Copy a single file
fluxphy video.mp4 /backup/

# Copy a directory recursively
fluxphy /data/ /backup/data/ --recursive

# Run with physics analysis
fluxphy largefile.iso /dest/ --analyze

# Quiet mode for scripting
fluxphy source.txt dest.txt --quiet

# Verify copy with checksum
fluxphy important.zip /backup/ --verify
```

## TUI Interface

```
┌─────────────────────────────────────────────────────────────┐
│                    FluxPhy Transfer Status                  │
├──────────────────────────┬──────────────────────────────────┤
│  ╔════════════════════╗  │         Flux Rate R(t)           │
│  ║ Transfer Metrics   ║  │                                  │
│  ╚════════════════════╝  │    250 ┤           ╭──╮          │
│                          │        │         ╭─╯  ╰─╮        │
│  File: document.pdf      │    200 ┤       ╭─╯      ╰─╮      │
│  Size: 45.2 MB           │        │     ╭─╯          ╰─╮    │
│                          │    150 ┤   ╭─╯              ╰─╮  │
│  [████████░░░░░░] 67%    │        │ ╭─╯                  ╰─ │
│                          │    100 ┤─╯                       │
│  Flux Rate: 234.5 MB/s   │        └─────────────────────────│
│  Mean Rate: 218.3 MB/s   │        0s    5s    10s    15s    │
│                          │                                  │
│  ╔════════════════════╗  │  Flow Regime: Laminar            │
│  ║ Physics Metrics    ║  │  Thermal Stability: 0.92         │
│  ╚════════════════════╝  │  Flux Density: 0.87              │
│                          │                                  │
│  Variance: 12.4 MB²/s²   │                                  │
│  CV: 0.016 (Laminar)     │                                  │
├──────────────────────────┴──────────────────────────────────┤
│ [Q] Quit  [P] Pause  [R] Resume  [S] Save Metrics           │
└─────────────────────────────────────────────────────────────┘
```

## Physics Metrics Explained

FluxPhy treats file transfers as a physical process and measures:

| Metric | Description |
|--------|-------------|
| **Flux Rate R(t)** | Transfer speed over time (MB/s) |
| **Thermal Stability** | S = 1 - CV, higher = more stable |
| **Flow Regime** | Laminar, Transitional, Turbulent, or Chaotic |
| **Shannon Entropy** | Measure of rate distribution randomness |
| **Flux Density** | Ratio of actual to theoretical maximum rate |
| **System Temperature** | Proportional to variance (σ²) |

### Flow Regimes

- **Laminar** (CV < 0.05): Smooth, predictable transfer
- **Transitional** (0.05 ≤ CV < 0.15): Mostly stable with minor fluctuations
- **Turbulent** (0.15 ≤ CV < 0.30): Significant rate fluctuations
- **Chaotic** (CV ≥ 0.30): Highly unpredictable transfer

## Configuration

Create `~/.config/fluxphy/config.toml`:

```toml
[general]
buffer_size = 8          # MB
sample_rate = 100        # ms
save_metrics = true

[ui]
theme = "default"
show_graph = true

[behavior]
force = false
verify = false
max_concurrent = 4
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Created by [Argo Navis Research Laboratory](https://github.com/BGSTA9)
