# UX & Interaction Requirements: FluxPhy TUI and Dashboard

## Layout and Screen Structure

- The output window must be divided **horizontally into two floors**.  
- **First (top) floor:**  
  - Displays one or more **line graphs** (e.g., throughput over time, latency over time).  
- **Second (bottom) floor:**  
  - Divided into **four vertical sections**.  
  - Each vertical section displays a subset of the **remaining metrics** (non-line‑graph metrics), grouped logically for readability.

---

## Keyboard Interaction and Dashboard Handoff

- When the file transfer is finished, pressing the **`S` key** performs the following actions:  
  - Sends or mirrors all collected metrics into a **high‑profile external data analysis dashboard window**.  
  - The external dashboard must present these metrics in a **more readable, visual, and user‑friendly format** suitable for an average user.  
- After this handoff occurs, the user must be **explicitly notified** about:  
  - **Where** the dashboard is located (e.g., URL, path, or application name).  
  - **How** they can re‑open or revisit it later.

---

## Branding and On‑Screen Messaging

- The terminal interface must **display the product logo** prominently.  
  - This can be ASCII/Unicode art or a stylized text logo, adapted to the terminal environment.  
- On startup, the tool should display a **welcoming/introductory message accompanied by the logo** that:  
  - Briefly **advertises the tool’s capabilities**.  
  - Uses a **fashionable and engaging style** while remaining clear and professional.  
  - Sets expectations about what the user is about to see (metrics, graphs, analytics).

---

## Dummy‑Friendly User Guide (In‑Tool and Dashboard)

- The tool must include a **“dummy” user guide**, designed so that even an **8‑year‑old** could understand:  
  - What the various **math/physics‑related metrics** on the screen mean.  
  - **Why** these metrics exist and why they are important.  
- This explanatory content must be available:  
  - **Inside the terminal interface itself** (e.g., via a help panel, toggle key, or onboarding walkthrough).  
  - **Inside the external dashboard** (e.g., tooltips, sidebars, “What does this mean?” info boxes).  
- Explanations should:  
  - Use **plain language and simple analogies**.  
  - Avoid or clearly decode jargon.  
  - Clarify the **purpose** and **practical impact** of each metric (e.g., “This tells you if your computer is waiting on the network.”).

---

## Metrics Visualization and Graphics

- The tool must use **appropriate and legible visualizations** for **all metrics**, not just line graphs.  
  - Examples: bar charts, gauges, spark lines, heatmaps, or other terminal‑friendly graphics.  
- Each metric group should use the **most suitable graph/visual form** for:  
  - Showing trends over time.  
  - Highlighting extremes, bottlenecks, or anomalies.  
  - Making quick comparisons intuitive, even for non‑experts.  
- Visual design should aim for:  
  - High contrast and readability.  
  - Minimal cognitive overload.  
  - Consistent color and layout conventions between the TUI and the dashboard.

---

## Transfer Completion Notifications

- Once the transfer operation is finished, the tool must **explicitly notify** the user of the outcome:  
  - **Success:** a clear success message, optionally including a brief summary (duration, size, key metrics).  
  - **Failure:** a clear failure message, including:  
    - Basic error reason (if known).  
    - Suggested next steps or where to find more detailed logs.  
- This completion notification should be visible in both:  
  - The **terminal interface**.  
  - The **external dashboard** (e.g., in a “Run summary”).

---

## Bottleneck Metric Issue

- The **“Bottleneck” metric** is currently **non‑functional**, always displaying `"unknown"`.  
- Requirements:  
  - Investigate and **fix the bottleneck detection logic** so that it can correctly identify the limiting resource (e.g., CPU, disk, network, remote endpoint).  
  - If the bottleneck truly cannot be determined for a run, display a more informative explanation than `"unknown"` (e.g., “insufficient data to determine bottleneck”).  
  - Ensure the bottleneck status is accurately reflected both in:  
    - The TUI metrics panel.  
    - The external dashboard.

---

## Open / Incomplete Requirement Note

- There is an incomplete requirement mentioning **“When the ‘Physcise’”**.  
- This likely refers to a future or partially specified feature (e.g., “Physics mode”, “Physicist view”, or a specific named metric).  
- Action item:  
  - Treat this as a **placeholder** and request clarification/specification before implementation.  
  - Reserve space in the UI and configuration for a future **“Physcise” feature or mode**, but do not ship a broken or misleading control.

# Design Roadmap: From Advanced File Transfer to Research‑Grade Data Lifecycle Instrument (FluxPhy)

## System / Instruction Context
Act as a **senior research‑software architect** designing the evolution of a data‑transfer and analysis tool (“FluxPhy”) into a **MIT‑level scientific instrument**.  
Produce detailed conceptual or architectural outputs that align with best practices in data provenance, time‑series analysis, integrity validation, and scientific reproducibility.

## Core Objective
Transform FluxPhy from a “fancy copier with stats” into a **data‑lifecycle and performance‑analysis instrument** with rigorous mathematics, provenance tracking, automation, and extensibility.

---

## 1. Strong Data Provenance & Lab Integration
Make FluxPhy part of the scientific record, not just a transport layer.

- **PROV‑compliant provenance models:**  
  Use standards like W3C PROV‑DM / PROV‑O or RO‑Crate JSON‑LD.  
- **Immutable IDs:**  
  Include sample, experiment, instrument, and pipeline run identifiers.  
- **System Integration:**  
  Link with ELN/LIMS/workflow systems via APIs to automatically update experiment entries and emit RO‑Crate records.  
- **Cryptographic Audit Trail:**  
  Support signed checksum manifests (e.g., lab PKI) and append‑only logs.  
- **Goal:** Move from file copying to scientifically **provable data lineage**.

---

## 2. Mathematically Serious Performance Modelling
Replace metaphors (“flux/laminar/chaotic”) with **time‑series and statistical rigor**.

- **Time‑Series Modelling:**  
  Apply ARIMA, VAR, spectral, or state‑space models for throughput prediction.  
- **Variability & Mixture Analysis:**  
  Use Gaussian mixtures or similar models to identify multiple performance regimes; select model complexity with BIC/AIC.  
- **Entropy & Dispersion:**  
  Keep entropy/CV but ground them in formal statistical frameworks (e.g., statistical process control).  
- **Predictive Optimization:**  
  Learn optimal buffer sizes, concurrency, and scheduling from historical logs.  
- **Goal:** Enable **quantitative modeling and prediction** of I/O performance.

---

## 3. Domain‑Aware Integrity & Structural Checks
Move beyond “bits match” toward verifying **scientific data structure integrity**.

- **Format‑Specific Validators:**  
  Add plugins for HDF5, NetCDF, DICOM, NIIfTI, FASTQ, BAM, and VCF formats.  
- **Replication Semantics:**  
  Support multi‑target replication with QUORUM verification (e.g., 2 of 3 copies).  
- **Provenance Integration:**  
  Record replication health and structural validation results in provenance logs.  
- **Goal:** Incorporate **data QA/QC** directly into transport workflows.

---

## 4. Smarter TUI for Exploratory Analysis
Transform the TUI into a **live analysis console** for diagnostics and experimentation.

- **Interactive Panels:**  
  Display time‑series, latency, mixture regimes, and control‑chart alerts.  
- **Exploratory Tools:**  
  Hypothesis‑driven tests (e.g., buffer‑size impact on performance).  
- **Comparative Analysis:**  
  Side‑by‑side visualization of different storage paths or settings.  
- **Exportable Reports:**  
  Generate JSON, PDF, or HTML summaries attachable to lab notebooks.  
- **Goal:** A **mini performance‑lab GUI** for data‑system behavior analysis.

---

## 5. Research‑Grade APIs & Extensibility
Turn FluxPhy into a **platform** usable for reproducible research and extension.

- **Programmatic APIs:**  
  Expose Python/R/CLI interfaces for streaming metrics, attaching labels, and integrating external analytics.  
- **Plugin Framework:**  
  Support new metrics, back‑ends, and visualization extensions.  
- **Reproducibility Support:**  
  Embed configuration fingerprints, versions, and environment metadata into collected outputs.  
- **Goal:** Make FluxPhy an **open, extensible research platform**.

---

## 6. Evaluation and Scoring Rationale (~99/100 Readiness)
To justify a “99/100” evaluation level, FluxPhy must:

- Provide **provable integrity and FAIR provenance**.  
- Employ **quantitative, time‑series‑based performance modeling**.  
- Include **domain‑specific structural validation**.  
- Feature a **diagnostic‑grade TUI**.  
- Offer **research‑ready APIs and plugin support** for extensibility.  
- Integrate smoothly into **HPC and autonomous lab environments**.

---

## Optional Next Step for the AI Model
> Sketch a detailed architecture or subsystem design for one of the roadmap areas — for example, the time‑series/mixture‑model analysis engine or the provenance schema — including data flow, component interfaces, and expected outputs.