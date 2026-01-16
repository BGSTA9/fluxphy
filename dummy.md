# FluxPhy - Complete Usage Guide & Examples

A comprehensive guide showcasing every functionality of the FluxPhy file transfer tool.

---

## 📋 Table of Contents

- [Basic File Operations](#-basic-file-operations)
- [Directory Operations](#-directory-operations)
- [Multiple Files](#-multiple-files)
- [Checksum Verification](#-checksum-verification)
- [Physics Analysis](#-physics-analysis)
- [Custom Buffer & Sample Rate](#️-custom-buffer--sample-rate)
- [Quiet Mode](#-quiet-mode)
- [Force Overwrite](#-force-overwrite)
- [Custom Metrics Output](#-custom-metrics-output)
- [Color Output Control](#-color-output-control)
- [Configuration File](#️-configuration-file)
- [Understanding Metrics Output](#-understanding-metrics-output)
- [TUI Keyboard Controls](#-tui-keyboard-controls)
- [Real-World Scenarios](#-real-world-scenarios)

---

## 📁 Basic File Operations

### Copy a single file to a directory
```bash
fluxphy document.pdf /backups/
```

### Copy a single file with a new name
```bash
fluxphy report.txt /archive/report_backup.txt
```

### Copy to current directory
```bash
fluxphy /data/important.zip ./
```

---

## 📂 Directory Operations

### Copy entire directory recursively
```bash
fluxphy /projects/myapp/ /backups/myapp_backup/ --recursive
```

### Short form for recursive copy
```bash
fluxphy -r /photos/2024/ /external/photos_backup/
```

### Nested directory structures
```bash
fluxphy /home/user/Documents/ /mnt/nas/Documents/ -r
```

---

## 📑 Multiple Files

### Copy multiple files to a destination
```bash
fluxphy file1.txt file2.txt file3.txt /destination/
```

### Copy various file types
```bash
fluxphy data.csv report.pdf image.png /archive/project/
```

### Combine with verification
```bash
fluxphy important1.zip important2.zip /backup/ --verify
```

---

## ✅ Checksum Verification

### Verify file integrity after copy
```bash
fluxphy sensitive_data.db /secure_backup/ --verify
```

### Verify with physics analysis
```bash
fluxphy critical_file.iso /backup/ --verify --analyze
```

> **Note**: Uses SHA-256 checksums to ensure data integrity.

---

## 🔬 Physics Analysis

### Enable detailed physics analysis
```bash
fluxphy largefile.mp4 /destination/ --analyze
```

### Verbose physics output
```bash
fluxphy dataset.tar.gz /backup/ --physics-verbose
```

### Combine physics options for maximum insight
```bash
fluxphy massive_archive.tar /storage/ --analyze --physics-verbose
```

### Physics Metrics Explained:

| Metric | Description |
|--------|-------------|
| **Flux Rate R(t)** | Instantaneous transfer speed over time (MB/s) |
| **Thermal Stability** | Consistency of transfer (1 = perfectly stable) |
| **Flow Regime** | Transfer pattern classification |
| **Shannon Entropy** | Randomness in rate distribution |
| **Flux Density** | Efficiency vs theoretical maximum |
| **Coefficient of Variation** | Normalized measure of dispersion |

### Flow Regime Classifications:

| Regime | CV Range | Meaning |
|--------|----------|---------|
| 🟢 Laminar | < 0.05 | Smooth, predictable transfer |
| 🟡 Transitional | 0.05 – 0.15 | Minor fluctuations |
| 🟠 Turbulent | 0.15 – 0.30 | Significant variations |
| 🔴 Chaotic | ≥ 0.30 | Highly unpredictable |

---

## ⚙️ Custom Buffer & Sample Rate

### Use larger buffer for better throughput
```bash
fluxphy huge_file.iso /dest/ --buffer-size 16
```

### Use smaller buffer for low-memory systems
```bash
fluxphy file.zip /dest/ --buffer-size 2
```

### Faster sampling rate for detailed graphs
```bash
fluxphy video.mp4 /dest/ --sample-rate 50
```

### Slower sampling for reduced overhead
```bash
fluxphy archive.tar /dest/ --sample-rate 200
```

### Combined performance tuning
```bash
fluxphy massive_backup.tar.gz /external/ --buffer-size 32 --sample-rate 50 --analyze
```

---

## 🤫 Quiet Mode

### Silent operation for scripts
```bash
fluxphy data.db /backup/ --quiet
```

### Short form quiet mode
```bash
fluxphy file.txt /dest/ -q
```

### Quiet with verification (still reports errors)
```bash
fluxphy important.zip /backup/ -q --verify
```

### Use in shell scripts
```bash
#!/bin/bash
fluxphy /daily_backup/* /nas/backups/ -q -r
echo "Backup completed: $(date)"
```

---

## 💪 Force Overwrite

### Overwrite existing files
```bash
fluxphy updated_config.yaml /etc/app/ --force
```

### Short form
```bash
fluxphy new_version.bin /firmware/ -f
```

### Force recursive copy
```bash
fluxphy /updated_project/ /deployment/ -r -f
```

---

## 📊 Custom Metrics Output

### Specify custom metrics filename
```bash
fluxphy data.zip /backup/ --metrics-file transfer_metrics.json
```

### Organized metrics by date
```bash
fluxphy backup.tar.gz /archive/ --metrics-file /logs/backup_$(date +%Y%m%d).json
```

### Metrics in specific directory
```bash
fluxphy project.zip /dest/ --metrics-file /var/log/fluxphy/project_transfer.json
```

---

## 🎨 Color Output Control

### Force color output
```bash
fluxphy file.txt /dest/ --color always
```

### Disable colors
```bash
fluxphy file.txt /dest/ --color never
```

### Auto-detect (default)
```bash
fluxphy file.txt /dest/ --color auto
```

### Useful for piping output
```bash
fluxphy file.txt /dest/ --color never 2>&1 | tee transfer.log
```

---

## 🛠️ Configuration File

Create `~/.config/fluxphy/config.toml` for persistent settings:

```toml
[general]
buffer_size = 8      # Buffer size in MB (default: 8)
sample_rate = 100    # Sample interval in ms (default: 100)

[ui]
theme = "default"    # UI theme
show_graph = true    # Display real-time graph

[behavior]
verify = false       # SHA-256 verification
force = false        # Overwrite existing files
```

### Custom configuration locations:
- **Linux/macOS**: `~/.config/fluxphy/config.toml`
- **Windows**: `%APPDATA%\fluxphy\config.toml`

---

## 📈 Understanding Metrics Output

After each transfer, FluxPhy generates a JSON metrics file:

```json
{
  "transfer_id": "20260116_194500_abc123",
  "timestamp": "2026-01-16T19:45:00Z",
  "source": "/home/user/video.mp4",
  "destination": "/backup/video.mp4",
  "file_size_bytes": 4738291200,
  "total_time_seconds": 21.45,
  "statistics": {
    "mean_rate_mb_s": 210.34,
    "variance": 156.78,
    "std_dev": 12.52,
    "coefficient_of_variation": 0.059,
    "min_rate_mb_s": 185.20,
    "max_rate_mb_s": 245.60,
    "peak_rate_mb_s": 245.60
  },
  "physics_metrics": {
    "flux_density": 0.84,
    "thermal_stability": 0.94,
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
  "rate_history": [
    [0.1, 198.45],
    [0.2, 208.32],
    [0.3, 215.67]
  ]
}
```

---

## ⌨️ TUI Keyboard Controls

During transfer, use these keyboard shortcuts:

| Key | Action |
|-----|--------|
| `Q` | Quit transfer |
| `P` | Pause transfer |
| `R` | Resume transfer |
| `S` | Save metrics snapshot |

---

## 🚀 Real-World Scenarios

### Scenario 1: Daily Backup Script
```bash
#!/bin/bash
# daily_backup.sh
DATE=$(date +%Y%m%d)

fluxphy /home/user/Documents/ /mnt/backup/docs_$DATE/ \
  -r \
  --verify \
  --metrics-file /var/log/backup_$DATE.json \
  -q

if [ $? -eq 0 ]; then
  echo "Backup successful"
else
  echo "Backup failed!"
fi
```

### Scenario 2: Large File Transfer with Monitoring
```bash
fluxphy largefile.iso /destination/ \
  --analyze \
  --physics-verbose \
  --buffer-size 16 \
  --sample-rate 50 \
  --verify
```

### Scenario 3: Project Deployment
```bash
fluxphy /build/release/ /var/www/production/ \
  -r \
  -f \
  --verify \
  --metrics-file /logs/deploy_$(date +%Y%m%d_%H%M%S).json
```

### Scenario 4: Archival with Maximum Integrity
```bash
fluxphy /critical_data/ /long_term_storage/ \
  -r \
  --verify \
  --analyze \
  --physics-verbose \
  --metrics-file /audit/archive_transfer.json
```

### Scenario 5: Scripted Multi-File Transfer
```bash
for file in *.mp4; do
  fluxphy "$file" /processed/ \
    -q \
    --verify \
    --metrics-file "/logs/${file%.mp4}_metrics.json"
done
```

### Scenario 6: Network Storage Migration
```bash
fluxphy /local/media_library/ /mnt/nas/media/ \
  -r \
  -f \
  --buffer-size 32 \
  --analyze \
  --metrics-file /var/log/nas_migration.json
```

---

## 📚 Quick Reference Card

```
USAGE:
  fluxphy [OPTIONS] <SOURCE>... <DESTINATION>

COMMON OPTIONS:
  -r, --recursive        Copy directories recursively
  -q, --quiet            No TUI, minimal output
  -f, --force            Overwrite existing files
  -a, --analyze          Enable physics analysis
      --verify           SHA-256 checksum verification
      --physics-verbose  Detailed physics output

PERFORMANCE OPTIONS:
      --buffer-size <MB>      Buffer size (default: 8)
      --sample-rate <MS>      Sample interval (default: 100)

OUTPUT OPTIONS:
      --metrics-file <FILE>   Custom metrics output path
      --color <MODE>          Color mode: auto|always|never

HELP:
  fluxphy --help           Show help message
  fluxphy --version        Show version information
```

---

## 🎯 Pro Tips

1. **For large files**: Use `--buffer-size 16` or higher for better throughput
2. **For SSDs**: Default buffer size usually works best
3. **For HDDs**: Consider `--buffer-size 32` to minimize seeks
4. **For scripts**: Always use `-q` to prevent TUI initialization
5. **For critical data**: Always use `--verify` to ensure integrity
6. **For debugging**: Use `--analyze --physics-verbose` to identify bottlenecks
7. **For logs**: Always specify `--metrics-file` with meaningful names

---

<p align="center">
  <em>Made with 🔬 by <a href="https://github.com/BGSTA9">Argo Navis Research Laboratory</a></em>
</p>
