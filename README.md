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
  <a href="https://pypi.org/project/fluxphy/"><img src="https://img.shields.io/badge/pypi-fluxphy-blue?logo=python" alt="PyPI"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey" alt="Platform">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-green" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-1.75+-orange?logo=rust" alt="Rust Version">
</p>

<!-- Description -->
<p align="center">
  <em>A cross-platform TUI/CLI file transfer tool with deep instrumentation<br/>into the "physics" of data flux.</em>
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-usage">Usage</a> •
  <a href="#-physics-metrics">Metrics</a> •
  <a href="#-data-provenance">Provenance</a>
</p>

---

## ✨ Features

FluxPhy goes beyond simple file copying by treating data transfer as a physical fluid dynamic process.

| Feature | Description |
|---------|-------------|
| 🚀 **High-Performance** | Optimized 8MB buffered I/O with async operations |
| 📊 **Real-Time TUI** | Live visualization with rate graph and "neolfetch" style summary |
| 📦 **Data Provenance** | Generates **W3C PROV-O JSON-LD** records for scientific reproducibility |
| 📉 **Advanced Modelling**| Real-time **Trend Analysis** (Linear Regression) and **Outlier Detection** |
| 🔬 **Physics Metrics** | Flow regime (Laminar/Turbulent), thermal stability, entropy |
| 🛡️ **Validation** | Domain-aware plugins (e.g., Magic Bytes verification for PNG/PDF) |
| 📈 **Dashboards** | Generates HTML reports with Chart.js visualization |
| 🎯 **Cross-Platform** | Validated on Linux, macOS (Intel/ARM), and Windows |

## 📺 Demo

The TUI provides a wealth of real-time information:

```text
┌─────────────────────────────────────────────────────────────┐
│                    FluxPhy Transfer Status                  │
├──────────────────────────┬──────────────────────────────────┤
│  Flux Rate: 234.5 MB/s   │           ╭──╮                   │
│  Trend: ↗ Accel          │         ╭─╯  ╰─╮        ╭─       │
│  Status: Laminar Flow    │       ╭─╯      ╰─╮    ╭─╯        │
│                          │     ╭─╯          ╰─╮╭─╯          │
│  File: dataset.csv       │   ╭─╯              ╰─            │
│  [████████░░░░░░] 67%    │ ╭─╯                              │
│                          └──────────────────────────────────┤
│  ETA: 45s                │  Physics Metrics                 │
│  Stability: 0.98         │  Entropy: 2.1 bits               │
│  Outliers: 0             │  Regime: Laminar (CV < 0.05)     │
├──────────────────────────┴──────────────────────────────────┤
│ [S] Generate Dashboard  [H] Help  [Q] Quit                  │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Installation

See [DISTRIBUTION.md](DISTRIBUTION.md) for a comprehensive guide on setting up your own distribution pipeline.

### Rust (Recommended)
```bash
cargo install fluxphy
```

### Python / Universal
```bash
pip install fluxphy
# or
uv pip install fluxphy
```

### macOS (Homebrew)
```bash
brew tap bgsta9/tap
brew install fluxphy
```

### Docker
```bash
docker run -it ghcr.io/bgsta9/fluxphy
```

### From Source
```bash
git clone https://github.com/BGSTA9/fluxphy
cd fluxphy
cargo build --release
sudo cp target/release/fluxphy /usr/local/bin/
```

## 📖 Usage

### Basic Transfer
```bash
# Copy file to destination
fluxphy source_file.csv /data/dest/

# Copy directory recursively
fluxphy /raw_data/ /processed_data/ -r
```

### Verification & Analysis
```bash
# Analyze first, validation plugins enabled
fluxphy dataset.fastq /backup/ --verify

# Run in quiet mode (scripts)
fluxphy data.bin /server/ --quiet
```

### Keyboard Shortcuts
*   **`S`**: Generate an HTML dashboard Report instantly.
*   **`H`**: Toggle the Help/Legend overlay.
*   **`Q`**: Quit/Cancel transfer.

## 📦 Data Provenance

FluxPhy is designed for scientific workflows where **Reproducibility** is key. Every transfer generates a `provenance.json` in the destination directory following the **W3C PROV-O** standard.

```json
{
  "@context": "http://www.w3.org/ns/prov#",
  "@type": "Activity",
  "label": "FluxPhy Transfer",
  "startTime": "2023-10-27T10:00:00Z",
  "used": [ { "entity": "source_file", "size": 1048576 } ],
  "generated": [ { "entity": "dest_file", "size": 1048576 } ],
  "agent": { "type": "Person", "name": "user" }
}
```

## 🔬 Physics Metrics

FluxPhy treats file transfers as a physical process:

| Metric | Description |
|--------|-------------|
| **Flux Rate** | Instantaneous throughput ($R(t)$). Highlighted **RED** if outlier detected. |
| **Trend** | ↗ Accelerating, → Stable, or ↘ Decelerating based on linear regression. |
| **Flow Regime** | **Laminar** (<5% variance) vs **Turbulent** (>15% variance). |
| **Shannon Entropy** | Measures the "randomness" of the rate distribution. |

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

<p align="center">
  Made with 🔬 by <a href="https://github.com/BGSTA9">Argo Navis Research Laboratory</a>
</p>
