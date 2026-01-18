---
description: How to update FluxPhy to the latest version
---

# Updating FluxPhy

Choose the method based on how you originally installed FluxPhy.

## Cargo (Recommended)
```bash
cargo install fluxphy --force
```

## From Source (Local Clone)
```bash
cd /Users/soheilsanati/Downloads/fluxphy
git pull origin main
// turbo
cargo build --release
sudo cp target/release/fluxphy /usr/local/bin/
```

## With Local Modifications
```bash
cd /Users/soheilsanati/Downloads/fluxphy
git stash
git pull origin main
git stash pop
cargo build --release
sudo cp target/release/fluxphy /usr/local/bin/
```

## Homebrew (Once Published)
```bash
brew upgrade fluxphy
```

## pip/uv (Once Published)
```bash
pip install --upgrade fluxphy
# or
uv pip install --upgrade fluxphy
```

## Docker
```bash
docker pull ghcr.io/bgsta9/fluxphy:latest
```

## Check Current Version
```bash
fluxphy --version
```
