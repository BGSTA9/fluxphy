<!-- Logo -->
<p align="center">
  <a href="https://github.com/BGSTA9/fluxphy">
    <img src="./logo/FLUXPHY_LOGO.svg" alt="FluxPhy Logo" width="500"/>
  </a>
</p>

<!-- Title -->
<h1 align="center">FluxPhy</h1>

<!-- Tagline -->
<p align="center">
  <strong>🔬 Physics of Flux File Transfer Tool</strong>
</p>

<!-- Badges -->
<p align="center">
  <a href="https://crates.io/crates/fluxphy"><img src="https://img.shields.io/badge/crates.io-fluxphy-orange?logo=rust" alt="Crates.io"></a>
  <a href="#installation"><img src="https://img.shields.io/badge/cargo-install-blue?logo=rust" alt="Cargo Install"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey" alt="Platform">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-green" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-1.75+-orange?logo=rust" alt="Rust Version">
</p>

<!-- Description -->
<p align="center">
  <em>A cross-platform TUI/CLI file transfer tool with deep instrumentation<br/>into the "physics" of data flux</em>
</p>

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🚀 **High-Performance** | Optimized 8MB buffered I/O with async operations |
| 📊 **Real-Time TUI** | Live visualization with rate graph (50% of screen) |
| 🔬 **Physics Metrics** | Flow regime, thermal stability, entropy analysis |
| 📈 **Analytics** | Statistical analysis and bottleneck detection |
| 🎯 **Cross-Platform** | Works on Linux, macOS, and Windows |
| 💾 **JSON Export** | Detailed metrics saved after every transfer |
| 🔍 **Verification** | Optional SHA-256 checksum verification |

## 📺 Demo

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
│  Progress: 67.3%         │        │ ╭─╯                  ╰─ │
│  [████████░░░░░░]        │    100 ┤─╯                       │
│                          │        └─────────────────────────│
│  Flux Rate: 234.5 MB/s   │        0s    5s    10s    15s    │
│  Mean Rate: 218.3 MB/s   │                                  │
│                          │  Flow Regime: Laminar            │
│  Elapsed: 00:02:15       │  Thermal Stability: 0.92         │
│  ETA: 00:00:45           │  Flux Density: 0.87              │
├──────────────────────────┴──────────────────────────────────┤
│ [Q] Quit  [P] Pause  [R] Resume  [S] Save Metrics           │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Installation

### Cargo (Recommended)
```bash
cargo install fluxphy
```

### Homebrew (macOS/Linux)
```bash
brew tap BGSTA9/tap
brew install fluxphy
```

### pip / uv
```bash
pip install fluxphy
# or
uv pip install fluxphy
```

### Docker
```bash
docker pull ghcr.io/bgsta9/fluxphy:latest
docker run --rm -v $(pwd):/data ghcr.io/bgsta9/fluxphy /data/source.txt /data/dest/
```

### From Source
```bash
git clone https://github.com/BGSTA9/fluxphy
cd fluxphy
cargo build --release
sudo cp target/release/fluxphy /usr/local/bin/
```

### Pre-built Binaries
Download from [GitHub Releases](https://github.com/BGSTA9/fluxphy/releases)

## 📖 Usage

```bash
# Basic file copy
fluxphy source.txt /destination/

# Directory copy (recursive)
fluxphy /data/ /backup/ --recursive

# With checksum verification
fluxphy important.zip /backup/ --verify

# Full physics analysis report
fluxphy largefile.iso /dest/ --analyze

# Quiet mode (for scripts)
fluxphy file.txt /dest/ --quiet
```

## 🔬 Physics Metrics

FluxPhy treats file transfers as a physical process:

| Metric | Formula | Description |
|--------|---------|-------------|
| **Flux Rate** | `R(t)` | Transfer speed over time (MB/s) |
| **Thermal Stability** | `S = 1 - CV` | Higher = more stable transfer |
| **Flow Regime** | `CV thresholds` | Laminar → Chaotic classification |
| **Shannon Entropy** | `H = -Σ p(r) log₂ p(r)` | Rate distribution randomness |
| **Flux Density** | `ρ = R / R_max` | Efficiency vs theoretical max |

### Flow Regimes

| Regime | CV Range | Behavior |
|--------|----------|----------|
| 🟢 **Laminar** | < 0.05 | Smooth, predictable |
| 🟡 **Transitional** | 0.05 – 0.15 | Minor fluctuations |
| 🟠 **Turbulent** | 0.15 – 0.30 | Significant variations |
| 🔴 **Chaotic** | ≥ 0.30 | Highly unpredictable |

## ⚙️ Configuration

Create `~/.config/fluxphy/config.toml`:

```toml
[general]
buffer_size = 8      # MB
sample_rate = 100    # ms

[ui]
theme = "default"
show_graph = true

[behavior]
verify = false
force = false
```

## 📊 Metrics Output

Every transfer generates a JSON file with complete analytics:

```json
{
  "statistics": {
    "mean_rate_mb_s": 210.34,
    "coefficient_of_variation": 0.059
  },
  "physics_metrics": {
    "flow_regime": "Laminar",
    "thermal_stability": 0.94,
    "shannon_entropy": 2.67
  },
  "rate_history": [[0.1, 198.4], [0.2, 208.3], ...]
}
```

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with 🔬 by <a href="https://github.com/BGSTA9">Argo Navis Research Laboratory</a>
</p>
