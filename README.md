# 🌀 fluxphy

**fluxphy** (Flux + Physics) is a Terminal User Interface (TUI) designed to treat data transfer not as a progress bar, but as a physical event. 

Most tools tell you *how much* has moved; **fluxphy** shows you *how* it is moving by plotting the mathematical relationship between speed, latency, and throughput on a live coordinate plane.

### ✨ Key Features
* **The Ghost Simulation:** Before a transfer begins, `fluxphy` probes your network to generate a "Predictive Path"—a mathematical ghost of the expected performance.
* **Vector Field Visualization:** Watch real-time telemetry draw over the simulation. Deviations create visual interference patterns, revealing "noise" or bottlenecks.
* **High-Resolution TUI:** Built using Braille-character plotting for a smooth, scientific "cyberdeck" aesthetic.
* **Diagnostic Art:** Turn mundane file moves into generative mathematical signatures.

### 🛠 How it Works
1. **Probe:** Sends scout packets to analyze the "gravity" of the network.
2. **Model:** Generates a vector field based on the probability density of your hardware's throughput.
3. **Execute:** Transfers the file while plotting reality against the model.

### 🚀 Quick Start (Concept)
```bash
pip install fluxphy
fluxphy move ./large-model.bin remote:/data/
