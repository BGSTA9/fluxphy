# FluxPhy User Guide 📘

Welcome to **FluxPhy**, the file transfer tool that treats data as a physical fluid. This guide will take you from zero to expert.

---

## 🚀 Getting Started

### Installation
Ensure you have FluxPhy installed. Run the following to verify:
```bash
fluxphy --fetch
```
You should see a beautiful system information screen. If not, refer to [README.md](README.md#installation).

---

## 🔰 Tutorial 1: Your First Transfer

FluxPhy works just like `cp` or `rsync` but with superpowers.

**Scenario**: You want to copy a large video file `movie.mkv` to your external drive `/Volumes/Backup`.

```bash
fluxphy movie.mkv /Volumes/Backup/
```

**What happens?**
1.  FluxPhy launches a **Graphical Interface** in your terminal.
2.  You see a real-time graph of the transfer speed.
3.  Metrics like "Thermal Stability" and "Flow Regime" update live.

---

## 📊 Tutorial 2: Mastering the Interface

The interface is divided into two main areas:

### 1. The Rate Graph (Top)
*   **Y-Axis**: Transfer speed in MB/s.
*   **X-Axis**: Time (scrolling right to left).
*   **Trend Line**: Watch for "↗ Accel" (Speeding up) or "↘ Decel" (Slowing down).

### 2. The Metrics Dashboard (Bottom)
*   **Flux Rate**: Current speed. If it blinks **RED**, it means the speed is unstable (an "outlier").
*   **Flow Regime**:
    *   🟢 **Laminar**: Smooth, consistent speed. Ideal.
    *   🔴 **Turbulent/Chaotic**: Highly variable speed. Usually means disk contention or network issues.
*   **Entropy**: A measure of randomness. Lower is better/smoother.

### Keyboard Shortcuts
*   **`H`**: Toggle the **Help Overlay** if you forget what a metric means.
*   **`S`**: Instantly generate an **HTML Dashboard Report** relative to where you ran the command.
*   **`Q`**: Abort the transfer.

---

## 🔬 Tutorial 3: Scientific Verification

If you are moving critical data (e.g., research data, backups), use the `--verify` flag or `--analyze`.

```bash
fluxphy archive.zip /server/backup/ --verify
```

*   **Verification**: FluxPhy calculates the SHA-256 hash of source and destination to ensure bit-perfect integrity.
*   **Post-Transfer**: Check the destination folder for `provenance.json`. This file proves *who* moved *what*, *where*, and *when*.

---

## 🤖 Tutorial 4: Automation & Scripts

FluxPhy is script-friendly. Use `--quiet` to suppress the UI and output only critical info or errors.

```bash
fluxphy data.csv /backup/ --quiet
```

### JSON Output
Every transfer automatically logs detailed metrics to a JSON file (e.g., `fluxphy_metrics_TIMESTAMP.json`). You can use this for your own analysis pipelines:

```json
{
  "statistics": { "mean_rate": 150.5 },
  "physics_metrics": { "flow_regime": "Laminar" }
}
```

---

## ❓ FAQ & Troubleshooting

**Q: My transfer is "Turbulent". Is that bad?**
A: It's not "bad" (data is safe), but it means your transfer isn't efficient. It might be due to:
*   Copying thousands of tiny files (overhead).
*   Another program using the disk.
*   Network congestion.

**Q: Can I copy folders?**
A: Yes! Use the `--recursive` (or `-r`) flag.
```bash
fluxphy /my/folder /backup/ -r
```

**Q: What is `provenance.json`?**
A: It's a "receipt" for your data. It adheres to the W3C PROV-O standard, useful for scientific data management.
