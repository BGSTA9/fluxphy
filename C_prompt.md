# FluxPhy - Physics of Flux File Transfer Tool

I want you to build a cross-platform TUI/CLI file copy tool called "fluxphy" in Rust that works on Linux, Mac, and Windows. This tool treats file transfer as a physical process and provides deep instrumentation into the "physics" of data flux.

## CORE REQUIREMENTS:

"A terminal plotting dialog tool for visualizing/monitoring real-time data transfer, that is a TUI."

1. **Functionality**:
   - Command to copy files/directories from source to destination
   - Support single files, multiple files, and entire directories
   - Cross-platform compatibility (Linux/Mac/Windows)

2. **Visual Interface**:
   - Real-time progress bar
   - Current file being copied
   - Transfer speed (MB/s, GB/s, auto-scaling)
   - Percentage complete
   - Elapsed time
   - Estimated time remaining (ETA)
   - Total files and size
   - Nice, clean TUI layout with physics-themed terminology
   - **50% of the screen should be a big plotting dialog chart [Line graph showing R(t)]**

3. **Instrumentation & Metrics Collection** (The "Physics" Layer):
   - Track transfer rate every 100ms → store as time series R(t)
   - Calculate statistics: mean rate, peak rate, variance, standard deviation
   - Detect bottlenecks (disk I/O, CPU, etc.)
   - Save detailed metrics to JSON log file after each operation
   - Real-time visualization of rate curve in terminal (live updating line graph)
   - Physics-inspired metrics: flux density, entropy, stability coefficients

4. **Post-Transfer Analysis**:
   - Summary statistics with physics terminology
   - Rate stability analysis (thermal stability analogy)
   - Predicted vs actual time comparison
   - Identify I/O patterns (sequential, random, turbulent, etc.)

## TECHNICAL SPECIFICATIONS:

**Language**: Rust (stable channel, edition 2021)

**Key Crates**:
- `clap` (v4+) - CLI argument parsing with derive macros
- `indicatif` - Progress bars and spinners
- `ratatui` (formerly tui-rs) - Terminal UI framework for the plotting interface
- `crossterm` - Cross-platform terminal manipulation
- `tokio` or `async-std` - Async runtime for concurrent operations
- `serde` + `serde_json` - JSON serialization for metrics
- `walkdir` - Recursive directory traversal
- `fs_extra` - Enhanced file operations
- `sysinfo` - System monitoring (CPU, disk I/O)
- `chrono` - Time handling
- `plotters` or custom ASCII plotting - For the real-time line graph

**Project Structure**:
```
fluxphy/
├── Cargo.toml
├── README.md
├── LICENSE
├── Makefile              # For make install
├── Dockerfile            # For Docker distribution
├── fluxphy.rb            # Homebrew formula
├── PKGBUILD              # Arch Linux package
├── fluxphy.spec          # RPM spec for Fedora/RHEL
├── debian/               # Debian packaging
│   ├── control
│   ├── changelog
│   └── rules
├── snap/                 # Snapcraft configuration
│   └── snapcraft.yaml
├── .github/
│   └── workflows/
│       └── release.yml   # CI/CD for multi-platform releases
├── src/
│   ├── main.rs           # Entry point and CLI setup
│   ├── cli.rs            # CLI argument parsing
│   ├── flux.rs           # Core file copy logic with instrumentation
│   ├── physics.rs        # Physics metrics calculation
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── display.rs    # TUI layout and rendering
│   │   └── plot.rs       # Real-time line graph plotting
│   ├── metrics.rs        # Metrics collection and storage
│   └── utils.rs          # Helper functions
├── tests/
│   └── integration_test.rs
└── examples/
    └── basic_copy.rs
```

## DISTRIBUTION & INSTALLATION METHODS:

FluxPhy should be easily installable via the following package managers and methods:

### Essential (MUST IMPLEMENT):

1. **cargo install** (Primary - Rust native)
   ```bash
   cargo install fluxphy
   ```

2. **brew** (macOS and Linux)
   ```bash
   brew install fluxphy
   ```

3. **winget** (Windows)
   ```bash
   winget install fluxphy
   ```

4. **curl/wget** (Pre-built binaries from GitHub releases)
   ```bash
   curl -L https://github.com/BGSTA9/fluxphy/releases/latest/download/fluxphy-linux-x86_64 -o fluxphy
   chmod +x fluxphy
   sudo mv fluxphy /usr/local/bin/
   ```
   ```bash
   wget https://github.com/BGSTA9/fluxphy/releases/latest/download/fluxphy-linux-x86_64
   chmod +x fluxphy-linux-x86_64
   sudo mv fluxphy-linux-x86_64 /usr/local/bin/fluxphy
   ```

5. **snap** (Cross-platform Linux)
   ```bash
   snap install fluxphy
   ```

### Linux Distribution Packages (MUST IMPLEMENT):

6. **apt/apt-get** (Debian/Ubuntu via PPA or .deb)
   ```bash
   # Via PPA
   sudo add-apt-repository ppa:fluxphy/fluxphy
   sudo apt update
   sudo apt install fluxphy
   
   # Or via .deb file
   wget https://github.com/BGSTA9/fluxphy/releases/latest/download/fluxphy_amd64.deb
   sudo apt install ./fluxphy_amd64.deb
   ```

7. **pacman/paru** (Arch Linux via AUR)
   ```bash
   # With pacman (after publishing to AUR)
   paru -S fluxphy
   # or
   yay -S fluxphy
   ```

8. **dnf** (Fedora/RHEL)
   ```bash
   sudo dnf install fluxphy
   ```

9. **zypper** (openSUSE)
   ```bash
   sudo zypper install fluxphy
   ```

10. **yum** (CentOS/older RHEL)
    ```bash
    sudo yum install fluxphy
    ```

### Container & Build Tools (SHOULD IMPLEMENT):

11. **docker** (Containerized version)
    ```bash
    docker pull ghcr.io/BGSTA9/fluxphy:latest
    docker run -v $(pwd):/data fluxphy /data/source.txt /data/dest.txt
    ```

12. **make** (Build from source)
    ```bash
    git clone https://github.com/BGSTA9/fluxphy
    cd fluxphy
    make
    sudo make install
    ```

## COMMAND-LINE INTERFACE:

```bash
# Basic usage
fluxphy source.txt /path/to/dest/

# Copy directory
fluxphy /source/dir/ /dest/dir/ --recursive

# Multiple files
fluxphy file1.txt file2.txt /dest/

# With physics analysis
fluxphy largefile.mp4 /dest/ --analyze

# Quiet mode (no TUI)
fluxphy source.txt /dest/ --quiet

# Save metrics with custom name
fluxphy source.txt /dest/ --metrics-file metrics.json

# Verbose physics output
fluxphy source.txt /dest/ --physics-verbose
```

## CARGO.TOML DEPENDENCIES:

```toml
[package]
name = "fluxphy"
version = "0.1.0"
edition = "2026"
description = "A file copy tool with deep instrumentation into the physics of data flux"
authors = ["Argo Navis Research Laboratory"]
license = "MIT"
repository = "https://github.com/BGSTA9/fluxphy"
homepage = "https://github.com/BGSTA9/fluxphy"
keywords = ["file-transfer", "tui", "monitoring", "performance"]
categories = ["command-line-utilities", "filesystem"]

[dependencies]
clap = { version = "4.4", features = ["derive"] }
indicatif = "0.17"
ratatui = "0.25"
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
walkdir = "2"
fs_extra = "1.3"
sysinfo = "0.30"
chrono = "0.4"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
tempfile = "3"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

## PACKAGING CONFIGURATIONS:

### 1. Makefile (for `make install`):
```makefile
PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

.PHONY: all build install clean

all: build

build:
	cargo build --release

install: build
	install -Dm755 target/release/fluxphy $(DESTDIR)$(BINDIR)/fluxphy

clean:
	cargo clean

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/fluxphy
```

### 2. Dockerfile:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /usr/src/fluxphy
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/fluxphy/target/release/fluxphy /usr/local/bin/fluxphy
ENTRYPOINT ["fluxphy"]
```

### 3. Homebrew Formula (fluxphy.rb):
```ruby
class Fluxphy < Formula
  desc "File copy tool with deep instrumentation into the physics of data flux"
  homepage "https://github.com/BGSTA9/fluxphy"
  url "https://github.com/BGSTA9/fluxphy/archive/v0.1.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/fluxphy", "--version"
  end
end
```

### 4. PKGBUILD (Arch Linux AUR):
```bash
# Maintainer: Your Name <your.email@example.com>
pkgname=fluxphy
pkgver=0.1.0
pkgrel=1
pkgdesc="A file copy tool with deep instrumentation into the physics of data flux"
arch=('x86_64')
url="https://github.com/BGSTA9/fluxphy"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/BGSTA9/$pkgname/archive/v$pkgver.tar.gz")
sha256sums=('...')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

### 5. Snapcraft (snap/snapcraft.yaml):
```yaml
name: fluxphy
version: '0.1.0'
summary: File copy tool with physics-based monitoring
description: |
  A file copy tool with deep instrumentation into the physics of data flux.
  Features real-time TUI with live graphing and comprehensive metrics.

base: core22
confinement: strict
grade: stable

apps:
  fluxphy:
    command: bin/fluxphy
    plugs:
      - home
      - removable-media

parts:
  fluxphy:
    plugin: rust
    source: .
    rust-channel: stable
```

### 6. RPM Spec (fluxphy.spec):
```spec
Name:           fluxphy
Version:        0.1.0
Release:        1%{?dist}
Summary:        File copy tool with physics-based monitoring

License:        MIT
URL:            https://github.com/BGSTA9/fluxphy
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

%description
A file copy tool with deep instrumentation into the physics of data flux.

%prep
%autosetup

%build
cargo build --release

%install
install -Dm755 target/release/fluxphy %{buildroot}%{_bindir}/fluxphy

%files
%{_bindir}/fluxphy
%license LICENSE
%doc README.md

%changelog
* Thu Jan 16 2026 Your Name <your.email@example.com> - 0.1.0-1
- Initial package
```

### 7. Debian Packaging (debian/control):
```
Source: fluxphy
Section: utils
Priority: optional
Maintainer: Your Name <your.email@example.com>
Build-Depends: debhelper-compat (= 13), cargo, rustc
Standards-Version: 4.6.0
Homepage: https://github.com/BGSTA9/fluxphy

Package: fluxphy
Architecture: any
Depends: ${shlibs:Depends}, ${misc:Depends}
Description: File copy tool with physics-based monitoring
 A file copy tool with deep instrumentation into the physics of data flux.
 Features real-time TUI with live graphing and comprehensive metrics.
```

### 8. WinGet Manifest (manifests/f/FluxPhy/FluxPhy/0.1.0.yaml):
```yaml
PackageIdentifier: FluxPhy.FluxPhy
PackageVersion: 0.1.0
PackageLocale: en-US
Publisher: FluxPhy Team
PackageName: FluxPhy
License: MIT
ShortDescription: File copy tool with physics-based monitoring
Installers:
  - Architecture: x64
    InstallerType: portable
    InstallerUrl: https://github.com/BGSTA9/fluxphy/releases/download/v0.1.0/fluxphy-windows-x86_64.exe
    InstallerSha256: ...
ManifestType: singleton
ManifestVersion: 1.0.0
```

## GITHUB ACTIONS CI/CD (.github/workflows/release.yml):

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build for ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: fluxphy
            asset_name: fluxphy-linux-x86_64
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact_name: fluxphy
            asset_name: fluxphy-linux-x86_64-musl
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: fluxphy.exe
            asset_name: fluxphy-windows-x86_64.exe
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: fluxphy
            asset_name: fluxphy-macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: fluxphy
            asset_name: fluxphy-macos-aarch64

    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload binaries to release
        uses: svenstaro/upload-release-action@v2
        with:
          repo_token: ${{ secrets.GITHUB_TOKEN }}
          file: target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
          asset_name: ${{ matrix.asset_name }}
          tag: ${{ github.ref }}

  publish-crates-io:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}

  publish-docker:
    name: Publish Docker image
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v3
      - name: Build and push Docker image
        run: |
          docker build -t ghcr.io/${{ github.repository }}:latest .
          echo ${{ secrets.GITHUB_TOKEN }} | docker login ghcr.io -u ${{ github.actor }} --password-stdin
          docker push ghcr.io/${{ github.repository }}:latest
```

## FEATURES TO IMPLEMENT:

### Phase 1 - Basic Copy with Progress Bar:
- CLI argument parsing with clap
- Single file copy with indicatif progress bar
- Real-time speed calculation
- ETA calculation
- Cross-platform path handling (std::path::PathBuf)
- Async I/O with tokio

### Phase 2 - TUI with Real-Time Plotting:
- Switch from simple progress bar to ratatui full TUI
- Create split layout: 50% info panel, 50% line graph
- Real-time line graph showing R(t) over time
- Update graph every 100ms with new data points
- Display current metrics in info panel
- Physics-themed styling and labels

### Phase 3 - Advanced Features:
- Directory copying (recursive with walkdir)
- Multiple files support
- Error handling with anyhow/thiserror
- Graceful interrupt handling (Ctrl+C)
- Optional checksum verification (SHA256)

### Phase 4 - Instrumentation (Physics Layer):
- Collect transfer rate samples every 100ms
- Store complete R(t) time series in Vec<(f64, f64)> as (time, rate) pairs
- Calculate statistics: mean, variance, std dev, min, max
- Calculate physics-inspired metrics:
    - Flux density: ρ(t) = R(t) / theoretical_max
    - Shannon entropy of rate distribution
    - Thermal stability coefficient: S = 1 - CV
    - System "temperature": T ∝ σ²
- Detect rate stability and classify flow regime
- Use sysinfo to identify bottlenecks (CPU/disk)
- Save metrics to JSON with serde

### Phase 5 - Post-Transfer Analysis:
- Display comprehensive physics statistics
- Rate variance analysis with flow regime classification
- Pattern detection (laminar vs turbulent)
- Prediction accuracy comparison
- ASCII plot summary in terminal output
- JSON export with full physics metrics

### Phase 6 - Distribution & Packaging:
- Set up GitHub Actions for automated releases
- Create pre-built binaries for all major platforms
- Publish to crates.io
- Create Homebrew formula and submit to homebrew-core
- Create PKGBUILD for AUR
- Create .deb packages for Debian/Ubuntu
- Create .rpm packages for Fedora/RHEL
- Create Snap package
- Create WinGet manifest
- Set up Docker Hub/GHCR publishing
- Write comprehensive installation documentation

## MATHEMATICAL/PHYSICS ANALYSIS TO IMPLEMENT:

1. **Basic Statistics** (in `physics.rs`):
   ```rust
   #[derive(Debug, Clone, Serialize)]
   pub struct FluxStatistics {
       pub mean_rate: f64,           // μ = Σ R(t) / n
       pub variance: f64,            // σ² = Σ(R(t) - μ)² / n
       pub std_dev: f64,             // σ
       pub coefficient_of_variation: f64,  // CV = σ / μ
       pub min_rate: f64,
       pub max_rate: f64,
       pub peak_rate: f64,
   }
   ```

2. **Physics-Inspired Metrics**:
   ```rust
   #[derive(Debug, Clone, Serialize)]
   pub struct PhysicsMetrics {
       pub flux_density: f64,              // ρ = R / R_max (0.0 to 1.0)
       pub thermal_stability: f64,         // S = 1 - CV (0.0 to 1.0)
       pub system_temperature: f64,        // T ∝ σ²
       pub shannon_entropy: f64,           // H = -Σ p(r) log₂ p(r)
       pub flow_regime: FlowRegime,        // Laminar/Transitional/Turbulent/Chaotic
   }

   #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
   pub enum FlowRegime {
       Laminar,       // CV < 0.05 - smooth, predictable
       Transitional,  // 0.05 ≤ CV < 0.15 - mostly stable
       Turbulent,     // 0.15 ≤ CV < 0.30 - significant fluctuations
       Chaotic,       // CV ≥ 0.30 - highly unpredictable
   }
   ```

3. **Bottleneck Detection** (using sysinfo crate):
   ```rust
   #[derive(Debug, Clone, Serialize)]
   pub struct SystemConstraints {
       pub primary_bottleneck: Bottleneck,
       pub cpu_usage: f32,
       pub disk_io_wait: f32,
       pub memory_pressure: f32,
       pub efficiency: f64,  // relative to bottleneck (0.0 to 1.0)
   }

   #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
   pub enum Bottleneck {
       DiskRead,
       DiskWrite,
       CPU,
       Memory,
       Network,
       Unknown,
   }
   ```

4. **Prediction System**:
   ```rust
   #[derive(Debug, Clone, Serialize)]
   pub struct PredictionMetrics {
       pub initial_rate_estimate: f64,  // MB/s from first 0.5-1s
       pub predicted_time: f64,         // seconds
       pub actual_time: f64,            // seconds
       pub error_percentage: f64,
       pub accuracy_class: Accuracy,
   }

   #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
   pub enum Accuracy {
       Excellent,  // < 5% error
       Good,       // 5-15% error
       Fair,       // 15-30% error
       Poor,       // > 30% error
   }
   ```

## REAL-TIME PLOTTING IMPLEMENTATION:

Use `ratatui` with Chart widget:

```rust
// In ui/plot.rs
use ratatui::{
    widgets::{Block, Borders, Chart, Axis, Dataset, GraphType},
    style::{Color, Style},
    symbols,
};

pub fn render_flux_graph(
    frame: &mut Frame,
    area: Rect,
    rate_history: &[(f64, f64)],  // (time, rate) pairs
    max_rate: f64,
) {
    let datasets = vec![
        Dataset::default()
            .name("Flux Rate")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(rate_history)
    ];

    let max_time = rate_history.last().map(|(t, _)| *t).unwrap_or(10.0);

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Flux Rate R(t)")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
        )
        .x_axis(
            Axis::default()
                .title("Time (s)")
                .bounds([0.0, max_time])
                .labels(vec![
                    "0s".into(),
                    format!("{:.1}s", max_time / 2.0).into(),
                    format!("{:.1}s", max_time).into(),
                ])
                .style(Style::default().fg(Color::Gray))
        )
        .y_axis(
            Axis::default()
                .title("Rate (MB/s)")
                .bounds([0.0, max_rate * 1.1])
                .labels(vec![
                    "0".into(),
                    format!("{:.0}", max_rate * 0.5).into(),
                    format!("{:.0}", max_rate).into(),
                ])
                .style(Style::default().fg(Color::Gray))
        );

    frame.render_widget(chart, area);
}
```

## PHYSICS TERMINOLOGY IN CODE:

Use physics-inspired naming throughout:
```rust
struct FluxTransfer {
    flux_rate: f64,           // not "transfer_rate"
    thermal_stability: f64,   // not "stability"
    flow_regime: FlowRegime,  // not "transfer_pattern"
    energy_distribution: f64, // not "variance"
}
```

## ERROR HANDLING:

Use Rust's Result type with custom errors:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FluxError {
    #[error("Source file not found: {0}")]
    SourceNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Disk full: {0}")]
    DiskFull(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Destination is a directory, use --recursive")]
    DestinationIsDirectory,
}

pub type FluxResult<T> = Result<T, FluxError>;
```

## CORE FILE COPY IMPLEMENTATION:

```rust
// In flux.rs - simplified example structure
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::time::{Duration, Instant};

pub struct FluxCopier {
    buffer_size: usize,
    rate_samples: Vec<(f64, f64)>,  // (time, rate)
    sample_interval: Duration,
}

impl FluxCopier {
    pub fn new() -> Self {
        Self {
            buffer_size: 8 * 1024 * 1024,  // 8 MB buffer
            rate_samples: Vec::new(),
            sample_interval: Duration::from_millis(100),
        }
    }

    pub async fn copy_file<P: AsRef<Path>>(
        &mut self,
        source: P,
        dest: P,
        progress_callback: impl Fn(u64, f64),
    ) -> FluxResult<()> {
        let source = source.as_ref();
        let dest = dest.as_ref();

        let file_size = std::fs::metadata(source)?.len();
        let src_file = File::open(source)?;
        let dst_file = File::create(dest)?;

        let mut reader = BufReader::with_capacity(self.buffer_size, src_file);
        let mut writer = BufWriter::with_capacity(self.buffer_size, dst_file);

        let mut buffer = vec![0u8; self.buffer_size];
        let mut total_copied = 0u64;
        let start_time = Instant::now();
        let mut last_sample = start_time;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            writer.write_all(&buffer[..bytes_read])?;
            total_copied += bytes_read as u64;

            let now = Instant::now();
            if now.duration_since(last_sample) >= self.sample_interval {
                let elapsed = now.duration_since(start_time).as_secs_f64();
                let rate = total_copied as f64 / elapsed / (1024.0 * 1024.0); // MB/s
                
                self.rate_samples.push((elapsed, rate));
                progress_callback(total_copied, rate);
                
                last_sample = now;
            }
        }

        writer.flush()?;
        Ok(())
    }

    pub fn get_rate_history(&self) -> &[(f64, f64)] {
        &self.rate_samples
    }
}
```

## ADDITIONAL REQUIREMENTS:

- **Performance**: Use buffered I/O with 8MB buffers for optimal throughput
- **Testing**: Unit tests for physics calculations, integration tests for file operations
- **Documentation**: 
  - Comprehensive README with installation instructions for all platforms
  - Usage examples and physics metrics explanation
  - API documentation with rustdoc comments
  - Architecture overview explaining the "physics" metaphor
- **Accessibility**: Support for colorblind-friendly color schemes
- **Logging**: Optional verbose logging with `env_logger` or `tracing`
- **Configuration**: Support for config file (~/.config/fluxphy/config.toml) to set defaults
- **Benchmarking**: Include criterion benchmarks for copy performance

## TUI LAYOUT SPECIFICATION:

```
┌─────────────────────────────────────────────────────────────┐
│                    FluxPhy Transfer Status                  │
├──────────────────────────┬──────────────────────────────────┤
│                          │                                  │
│  ╔════════════════════╗  │                                  │
│  ║ Transfer Metrics   ║  │         Flux Rate R(t)           │
│  ╚════════════════════╝  │                                  │
│                          │    250 ┤           ╭──╮          │
│  File: document.pdf      │        │         ╭─╯  ╰─╮        │
│  Size: 45.2 MB           │    200 ┤       ╭─╯      ╰─╮      │
│                          │        │     ╭─╯          ╰─╮    │
│  Progress: 67.3%         │    150 ┤   ╭─╯              ╰─╮  │
│  [████████░░░░░░]        │        │ ╭─╯                  ╰─ │
│                          │    100 ┤─╯                       │
│  Flux Rate: 234.5 MB/s   │        │                         │
│  Mean Rate: 218.3 MB/s   │     50 ┤                         │
│  Peak Rate: 267.1 MB/s   │        │                         │
│                          │      0 └─────────────────────────│
│  Elapsed: 00:02:15       │        0s    5s    10s    15s    │
│  ETA: 00:00:45           │                                  │
│                          │                                  │
│  ╔════════════════════╗  │  Flow Regime: Laminar            │
│  ║ Physics Metrics    ║  │  Thermal Stability: 0.92         │
│  ╚════════════════════╝  │  Flux Density: 0.87              │
│                          │                                  │
│  Variance: 12.4 MB²/s²   │                                  │
│  Std Dev: 3.52 MB/s      │                                  │
│  CV: 0.016 (Laminar)     │                                  │
│                          │                                  │
│  Bottleneck: Disk Write  │                                  │
│  System Temp: 0.034      │                                  │
│  Entropy: 2.41 bits      │                                  │
│                          │                                  │
├──────────────────────────┴──────────────────────────────────┤
│ [Q] Quit  [P] Pause  [R] Resume  [S] Save Metrics           │
└─────────────────────────────────────────────────────────────┘
```

## SAMPLE METRICS JSON OUTPUT:

```json
{
  "transfer_id": "20260116_143052_abc123",
  "timestamp": "2026-01-16T14:30:52Z",
  "source": "/home/user/largefile.mp4",
  "destination": "/mnt/backup/largefile.mp4",
  "file_size_bytes": 4738291200,
  "total_time_seconds": 21.45,
  "statistics": {
    "mean_rate_mb_s": 210.34,
    "variance": 156.78,
    "std_dev": 12.52,
    "coefficient_of_variation": 0.0595,
    "min_rate_mb_s": 185.20,
    "max_rate_mb_s": 245.60,
    "peak_rate_mb_s": 245.60
  },
  "physics_metrics": {
    "flux_density": 0.84,
    "thermal_stability": 0.9405,
    "system_temperature": 156.78,
    "shannon_entropy": 2.67,
    "flow_regime": "Laminar"
  },
  "system_constraints": {
    "primary_bottleneck": "DiskWrite",
    "cpu_usage": 12.4,
    "disk_io_wait": 8.7,
    "memory_pressure": 23.1,
    "efficiency": 0.87
  },
  "prediction": {
    "initial_rate_estimate_mb_s": 215.30,
    "predicted_time_seconds": 20.98,
    "actual_time_seconds": 21.45,
    "error_percentage": 2.24,
    "accuracy_class": "Excellent"
  },
  "rate_history": [
    [0.0, 0.0],
    [0.1, 198.45],
    [0.2, 208.32],
    [0.3, 215.67],
    [0.4, 220.12],
    // ... more samples every 100ms
    [21.4, 212.89],
    [21.45, 210.34]
  ]
}
```

## CLI ARGUMENT STRUCTURE:

```rust
// In cli.rs
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "fluxphy")]
#[command(author = "FluxPhy Team")]
#[command(version = "0.1.0")]
#[command(about = "A file copy tool with deep instrumentation into the physics of data flux", long_about = None)]
pub struct Cli {
    /// Source file(s) or directory to copy
    #[arg(required = true)]
    pub sources: Vec<PathBuf>,

    /// Destination path
    #[arg(required = true)]
    pub destination: PathBuf,

    /// Copy directories recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// Quiet mode - no TUI, minimal output
    #[arg(short, long)]
    pub quiet: bool,

    /// Enable verbose physics analysis
    #[arg(long)]
    pub physics_verbose: bool,

    /// Enable detailed analysis and reporting
    #[arg(short, long)]
    pub analyze: bool,

    /// Custom metrics output file (default: fluxphy_metrics_<timestamp>.json)
    #[arg(long, value_name = "FILE")]
    pub metrics_file: Option<PathBuf>,

    /// Verify file integrity with checksum after copy
    #[arg(long)]
    pub verify: bool,

    /// Buffer size in MB (default: 8)
    #[arg(long, default_value = "8")]
    pub buffer_size: usize,

    /// Sample rate in milliseconds (default: 100)
    #[arg(long, default_value = "100")]
    pub sample_rate: u64,

    /// Overwrite existing files without prompting
    #[arg(short, long)]
    pub force: bool,

    /// Color output mode
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorMode,

    /// Show version information
    #[arg(short = 'V', long)]
    pub version: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}
```

## COMPLETE EXAMPLE USAGE SCENARIOS:

### 1. Basic Single File Copy:
```bash
$ fluxphy video.mp4 /backup/

╔═══════════════════════════════════════════════════════════╗
║              FluxPhy Transfer Complete                    ║
╚═══════════════════════════════════════════════════════════╝

File: video.mp4
Size: 1.2 GB
Time: 00:00:15
Mean Rate: 82.3 MB/s
Flow Regime: Laminar
Thermal Stability: 0.94

Metrics saved to: fluxphy_metrics_20260116_143052.json
```

### 2. Directory Copy with Analysis:
```bash
$ fluxphy /data/photos/ /backup/photos/ --recursive --analyze

[Running FluxPhy analysis...]

═══════════════════════════════════════════════════════════
                    PHYSICS ANALYSIS REPORT
═══════════════════════════════════════════════════════════

Transfer Summary:
  Total Files: 1,245
  Total Size: 8.7 GB
  Duration: 00:03:42
  Mean Flux Rate: 40.2 MB/s

Statistical Analysis:
  μ (mean):     40.23 MB/s
  σ (std dev):   2.15 MB/s
  σ² (variance): 4.62 MB²/s²
  CV:            0.053

Physics Metrics:
  Flow Regime:        Laminar
  Thermal Stability:  0.947
  Flux Density:       0.72
  System Temperature: 4.62
  Shannon Entropy:    2.89 bits

System Constraints:
  Primary Bottleneck: Disk Write
  CPU Usage:          8.3%
  Disk I/O Wait:      12.7%
  Memory Pressure:    18.4%
  Efficiency:         0.75

Prediction Accuracy:
  Predicted Time: 00:03:38
  Actual Time:    00:03:42
  Error:          1.8% (Excellent)

Rate Distribution:
  Min: 34.5 MB/s
  Q1:  38.9 MB/s
  Q2:  40.1 MB/s
  Q3:  41.8 MB/s
  Max: 46.2 MB/s

Flow Analysis:
  The transfer exhibited highly stable laminar flow with
  minimal turbulence. The low coefficient of variation
  (0.053) indicates consistent disk I/O performance.
  Bottleneck analysis suggests disk write speed was the
  primary limiting factor, achieving 75% efficiency.

═══════════════════════════════════════════════════════════
```

### 3. Quiet Mode for Scripting:
```bash
$ fluxphy largefile.iso /mnt/usb/ --quiet

Copying: largefile.iso → /mnt/usb/largefile.iso
Progress: [████████████████████████████████] 100%
Complete: 4.5 GB in 00:01:23 (55.3 MB/s)
```

### 4. Multiple Files:
```bash
$ fluxphy file1.txt file2.dat file3.bin /dest/

FluxPhy: Copying 3 files (total: 256.7 MB)

[1/3] file1.txt     [████████████] 100% - Complete
[2/3] file2.dat     [████████████] 100% - Complete
[3/3] file3.bin     [██████░░░░░░]  52% - 45.3 MB/s

Overall Progress: 84% | ETA: 00:00:12
```

## ADVANCED FEATURES:

### 1. Adaptive Buffer Sizing:
```rust
// Automatically adjust buffer size based on file size and system memory
impl FluxCopier {
    fn calculate_optimal_buffer(&self, file_size: u64, available_mem: u64) -> usize {
        let min_buffer = 1 * 1024 * 1024;  // 1 MB
        let max_buffer = 64 * 1024 * 1024; // 64 MB
        
        let suggested = match file_size {
            0..=10_000_000 => 1 * 1024 * 1024,           // 1 MB for small files
            10_000_001..=100_000_000 => 4 * 1024 * 1024, // 4 MB for medium
            100_000_001..=1_000_000_000 => 8 * 1024 * 1024, // 8 MB for large
            _ => 16 * 1024 * 1024,                       // 16 MB for huge
        };
        
        suggested.clamp(min_buffer, max_buffer.min(available_mem as usize / 10))
    }
}
```

### 2. Checksum Verification:
```rust
use sha2::{Sha256, Digest};

async fn verify_checksum(file_path: &Path) -> FluxResult<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0u8; 8 * 1024 * 1024];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}
```

### 3. Resume Support (Future Enhancement):
```rust
// Save transfer state to allow resume after interruption
#[derive(Serialize, Deserialize)]
struct TransferState {
    source: PathBuf,
    destination: PathBuf,
    bytes_copied: u64,
    checksum_partial: String,
    timestamp: String,
}
```

### 4. Parallel File Copying:
```rust
// For directory copies, process multiple files concurrently
use tokio::task::JoinSet;

async fn copy_directory_parallel(
    sources: Vec<PathBuf>,
    dest: PathBuf,
    max_concurrent: usize,
) -> FluxResult<()> {
    let mut tasks = JoinSet::new();
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    
    for source in sources {
        let dest = dest.clone();
        let permit = semaphore.clone().acquire_owned().await?;
        
        tasks.spawn(async move {
            let result = copy_file_async(&source, &dest).await;
            drop(permit);
            result
        });
    }
    
    while let Some(result) = tasks.join_next().await {
        result??;
    }
    
    Ok(())
}
```

## CONFIGURATION FILE SUPPORT:

### Default Config Location: `~/.config/fluxphy/config.toml`

```toml
# FluxPhy Configuration File

[general]
# Default buffer size in MB
buffer_size = 8

# Sample rate in milliseconds
sample_rate = 100

# Always save metrics after transfer
save_metrics = true

# Default metrics directory
metrics_dir = "~/.local/share/fluxphy/metrics"

[ui]
# Color scheme: "default", "nord", "dracula", "solarized"
theme = "default"

# Show real-time graph by default
show_graph = true

# Graph update interval in milliseconds
graph_update_interval = 100

[analysis]
# Enable physics analysis by default
analyze = false

# Verbose physics output
physics_verbose = false

# Automatically detect bottlenecks
detect_bottlenecks = true

[behavior]
# Overwrite files without prompting
force = false

# Verify checksums after copy
verify = false

# Maximum concurrent file copies for directories
max_concurrent = 4

[advanced]
# Use direct I/O (bypass OS cache) for large files
use_direct_io = false

# Enable experimental adaptive buffering
adaptive_buffer = true

# Log level: "error", "warn", "info", "debug", "trace"
log_level = "info"
```

### Loading Configuration:
```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub ui: UiConfig,
    pub analysis: AnalysisConfig,
    pub behavior: BehaviorConfig,
    pub advanced: AdvancedConfig,
}

impl Config {
    pub fn load() -> FluxResult<Self> {
        let config_path = dirs::config_dir()
            .ok_or_else(|| FluxError::ConfigNotFound)?
            .join("fluxphy")
            .join("config.toml");
        
        if config_path.exists() {
            let contents = fs::read_to_string(config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }
}
```

## TESTING STRATEGY:

### 1. Unit Tests:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flux_statistics_calculation() {
        let samples = vec![100.0, 105.0, 98.0, 102.0, 101.0];
        let stats = calculate_statistics(&samples);
        
        assert_eq!(stats.mean_rate, 101.2);
        assert!((stats.std_dev - 2.49).abs() < 0.1);
        assert_eq!(stats.flow_regime, FlowRegime::Laminar);
    }

    #[test]
    fn test_flow_regime_classification() {
        assert_eq!(classify_flow_regime(0.03), FlowRegime::Laminar);
        assert_eq!(classify_flow_regime(0.10), FlowRegime::Transitional);
        assert_eq!(classify_flow_regime(0.20), FlowRegime::Turbulent);
        assert_eq!(classify_flow_regime(0.35), FlowRegime::Chaotic);
    }

    #[test]
    fn test_shannon_entropy() {
        let samples = vec![100.0, 100.0, 100.0, 100.0];
        assert_eq!(calculate_entropy(&samples), 0.0); // No entropy
        
        let samples = vec![50.0, 100.0, 150.0, 200.0];
        assert!(calculate_entropy(&samples) > 0.0); // Has entropy
    }
}
```

### 2. Integration Tests:
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_copy_basic() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source.txt");
        let dest = temp.path().join("dest.txt");
        
        fs::write(&source, b"Hello, FluxPhy!").unwrap();
        
        let mut copier = FluxCopier::new();
        copier.copy_file(&source, &dest, |_, _| {}).await.unwrap();
        
        assert_eq!(fs::read(&source).unwrap(), fs::read(&dest).unwrap());
    }

    #[tokio::test]
    async fn test_directory_copy_recursive() {
        let temp = TempDir::new().unwrap();
        let source_dir = temp.path().join("source");
        let dest_dir = temp.path().join("dest");
        
        fs::create_dir_all(source_dir.join("subdir")).unwrap();
        fs::write(source_dir.join("file1.txt"), b"data1").unwrap();
        fs::write(source_dir.join("subdir/file2.txt"), b"data2").unwrap();
        
        copy_directory(&source_dir, &dest_dir, true).await.unwrap();
        
        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("subdir/file2.txt").exists());
    }

    #[tokio::test]
    async fn test_metrics_generation() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("large.bin");
        let dest = temp.path().join("large_copy.bin");
        
        // Create 10 MB file
        let data = vec![0u8; 10 * 1024 * 1024];
        fs::write(&source, data).unwrap();
        
        let mut copier = FluxCopier::new();
        copier.copy_file(&source, &dest, |_, _| {}).await.unwrap();
        
        let metrics = copier.generate_metrics();
        assert!(metrics.statistics.mean_rate > 0.0);
        assert!(metrics.rate_history.len() > 0);
    }
}
```

### 3. Benchmarks (with Criterion):
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_copy_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_copy");
    
    for size in [1, 10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}MB", size)),
            size,
            |b, &size| {
                b.iter(|| {
                    // Benchmark copy performance for different file sizes
                    let data = vec![0u8; size * 1024 * 1024];
                    black_box(data);
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_copy_performance);
criterion_main!(benches);
```

## README.md TEMPLATE:

```markdown
# FluxPhy - Physics of Flux File Transfer Tool

<p align="center">
  <img src="docs/logo.png" alt="FluxPhy Logo" width="200"/>
</p>

<p align="center">
  <strong>A file copy tool with deep instrumentation into the physics of data flux</strong>
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
```

## Physics Metrics Explained

FluxPhy treats file transfers as a physical process and measures:

- **Flux Rate R(t)**: Transfer speed over time (MB/s)
- **Thermal Stability**: S = 1 - CV, where CV is coefficient of variation
- **Flow Regime**: Classification as Laminar, Transitional, Turbulent, or Chaotic
- **Shannon Entropy**: Measure of rate distribution randomness
- **Flux Density**: Ratio of actual to theoretical maximum rate

## Documentation

Full documentation available at [docs.fluxphy.io](https://docs.fluxphy.io)

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)
```

## BUILD AND RELEASE CHECKLIST:

- [ ] Set up GitHub repository with proper structure
- [ ] Configure Cargo.toml with all dependencies
- [ ] Implement core file copy functionality
- [ ] Add TUI with ratatui and real-time plotting
- [ ] Implement physics metrics calculation
- [ ] Add comprehensive error handling
- [ ] Write unit tests (>80% coverage target)
- [ ] Write integration tests
- [ ] Add benchmarks with criterion
- [ ] Create all packaging files (Makefile, Dockerfile, etc.)
- [ ] Set up GitHub Actions CI/CD
- [ ] Test on all three platforms (Linux, macOS, Windows)
- [ ] Generate pre-built binaries for all architectures
- [ ] Publish to crates.io
- [ ] Submit Homebrew formula
- [ ] Publish to AUR
- [ ] Create Snap package
- [ ] Submit to WinGet
- [ ] Write comprehensive documentation
- [ ] Create usage examples and tutorials
- [ ] Add screenshots and demo GIF to README
- [ ] Set up issue templates and contributing guidelines
- [ ] Create security policy
- [ ] Add changelog
- [ ] Tag v0.1.0 release

## FUTURE ENHANCEMENTS (Post v1.0):

1. **Network Transfer Support**: Extend to support SCP, SFTP, HTTP/S
2. **Compression on the Fly**: Compress during transfer to save bandwidth
3. **Resume Capability**: Save state and resume interrupted transfers
4. **Cloud Storage Integration**: Direct integration with S3, GCS, Azure
5. **GUI Version**: Electron or Tauri-based graphical interface
6. **Batch Operations**: Queue and process multiple transfer jobs
7. **Scheduling**: Cron-like scheduling for automated transfers
8. **Bandwidth Limiting**: Rate limiting for network-friendly transfers
9. **Synchronization Mode**: Rsync-like differential sync
10. **Plugin System**: Allow community-developed extensions

---

**End of Specification Document**