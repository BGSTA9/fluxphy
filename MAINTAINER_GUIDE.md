# FluxPhy Maintainer Guide 🛠️

This guide is for developers working on the FluxPhy codebase. It covers architecture, testing, and release procedures.

---

## 🏗️ Architecture Overview

FluxPhy is built on **Tokio** (async runtime) and **Ratatui** (TUI).

### Core Modules
*   **`src/flux.rs`**: The heart of the application. Contains `FluxCopier` struct and the async copy loop logic. It samples transfer rates and sends `ProgressUpdate` messages to the UI channel.
*   **`src/physics.rs`**: Calculates flow regimes, Reynolds-analogy metrics (CV), and entropy.
*   **`src/analysis/mod.rs`**: Implements the `TimeSeriesModel` (linear regression for trends, sigma rules for outliers).
*   **`src/provenance.rs`**: Handles W3C PROV-O JSON-LD generation.
*   **`src/validation/mod.rs`**: Plugin system for file integrity checks (Magic Bytes, etc.).
*   **`src/main.rs`**: Entry point. Sets up CLI, TUI loop, and orchestrates the copy thread vs UI thread.

---

## 🧪 Development Workflow

### Building
```bash
cargo build           # fast debug build
cargo build --release # optimized build
```

### Testing
Run the full suite, including unit tests for physics calculations and validators:
```bash
cargo test
```

### Adding a New Validator
1.  Open `src/validation/mod.rs`.
2.  Define a new struct (e.g., `pub struct MyCustomValidator;`).
3.  Implement the `Validator` trait for it.
4.  Register it in `src/main.rs` inside the `perform_transfer` loop.

---

## 📦 Release Process (CI/CD)

Releases are fully automated via GitHub Actions (`.github/workflows/release.yml`).

1.  **Bump Version**: Update `version` in `Cargo.toml` and `pyproject.toml`.
2.  **Commit**: `git commit -m "Bump version to x.y.z"`
3.  **Tag**: `git tag vx.y.z`
4.  **Push**: `git push origin vx.y.z`

The CI will automatically:
*   Build binaries for all OSes.
*   Publish to Crates.io and PyPI.
*   Update the Homebrew Tap.
*   Push Docker image.

---

## 🔐 Secrets Management

To keep the CD pipeline working, ensure these Secrets are set in the GitHub Repo:
*   `CARGO_TOKEN`: Crates.io API token.
*   `PYPI_API_TOKEN`: PyPI API token.
*   `TAP_GITHUB_TOKEN`: Personal Access Token with repo scope (for updating Homebrew).

---

## 📐 Design Philosophy

*   **Accuracy over Speed**: FluxPhy is fast, but its primary goal is *observability*. We prefer accurate metrics even if it costs 1% throughput.
*   **Scientific Rigor**: Terms like "Laminar" and "Entropy" are not marketing fluff; they are calculated using actual statistical formulas. Maintain this rigor.
