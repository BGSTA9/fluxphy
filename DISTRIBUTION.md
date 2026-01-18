# FluxPhy Distribution Guide

Your project is configured with a fully automated **Continuous Delivery (CD)** pipeline using GitHub Actions. This allows you to publish `fluxphy` to millions of users worldwide across multiple platforms by simply creating a Release on GitHub.

## 🚀 Supported Platforms

| Platform | Install Command | Status |
|----------|----------------|--------|
| **Rust (Crates.io)** | `cargo install fluxphy` | ✅ Configured |
| **Python (PyPI)** | `pip install fluxphy` | ✅ Configured (Maturin) |
| **macOS (Homebrew)** | `brew install bgsta9/tap/fluxphy` | ✅ Configured (Custom Tap) |
| **Docker** | `docker pull ghcr.io/bgsta9/fluxphy` | ✅ Configured |
| **Linux/Win/Mac** | *Download binary from GitHub Releases* | ✅ Configured |

---

## 🛠 One-Time Setup (Secrets)

To enable the automation, you must add the following **Secrets** to your GitHub Repository settings (`Settings` -> `Secrets and variables` -> `Actions` -> `New repository secret`):

### 1. Crates.io Token (`CARGO_TOKEN`)
*   Log in to [crates.io](https://crates.io/me)
*   Go to Account Settings -> API Tokens -> New Token
*   Copy the token and save as `CARGO_TOKEN`

### 2. PyPI Token (`PYPI_API_TOKEN`)
*   Log in to [pypi.org](https://pypi.org/manage/account/token/)
*   Create a generic token (scope: "Entire account" for first publish, then limit to "Project: fluxphy")
*   Copy the token (starts with `pypi-`) and save as `PYPI_API_TOKEN`

### 3. Homebrew Tap Token (`TAP_GITHUB_TOKEN`)
*   You need a separate repository named `homebrew-tap` (e.g., `github.com/bgsta9/homebrew-tap`).
*   Create a [Personal Access Token (Classic)](https://github.com/settings/tokens) with `repo` (full control of private repositories) scope.
*   Save it as `TAP_GITHUB_TOKEN`.
*   *Note: The workflow automatically updates the formula in that repository.*

---

## 📦 How to Publish a New Version

1.  **Update Version**:
    Edit `Cargo.toml` and `pyproject.toml` to the new version number (e.g., `0.2.0`).
    ```bash
    # Update version in Cargo.toml
    vim Cargo.toml
    # Update version in pyproject.toml
    vim pyproject.toml
    
    git add .
    git commit -m "Bump version to 0.2.0"
    git push origin main
    ```

2.  **Trigger Release**:
    Create a new **Tag** starting with `v` and push it.
    ```bash
    git tag v0.2.0
    git push origin v0.2.0
    ```
    
    *Alternatively, go to GitHub Releases -> Draft a new release -> Choose tag `v0.2.0` -> Create.*

3.  **Watch the Magic**:
    Go to the **Actions** tab on your GitHub repository. You will see the `Release` workflow running. It will:
    *   Build binaries for Linux, Windows, macOS (Intel & Apple Silicon).
    *   Upload binaries to the GitHub Release page.
    *   Publish to Crates.io.
    *   Publish wheels to PyPI.
    *   Build and push the Docker image.
    *   Update your Homebrew Tap formula.

---

## 🌍 How Users Install It (The "Simple" Explanation)

Once published, anyone can install `fluxphy` using their preferred tool:

### Rust Users
```bash
cargo install fluxphy
```

### Python Users / Universal
*Since `pip` is on almost every system, this is a great universal installer.*
```bash
pip install fluxphy
# or with uv
uv pip install fluxphy
```

### macOS Users
```bash
brew tap bgsta9/tap
brew install fluxphy
```

### Docker Users
```bash
docker run -it ghcr.io/bgsta9/fluxphy
```

### Manual Installation
Users can go to your **GitHub Releases** page and download the single binary for their OS (no dependencies required).
