//! HTML Dashboard Generator for FluxPhy
//!
//! Generates a standalone HTML file with rich visualizations of transfer metrics.

use crate::ui::AppState;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Generate an HTML dashboard from the current transfer state
pub fn generate_dashboard(state: &AppState) -> std::io::Result<PathBuf> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("fluxphy_dashboard_{}.html", timestamp);
    let path = PathBuf::from(&filename);

    let html = generate_html(state);
    let mut file = File::create(&path)?;
    file.write_all(html.as_bytes())?;

    Ok(path)
}

fn generate_html(state: &AppState) -> String {
    let rate_data: String = state
        .rate_history
        .iter()
        .map(|(t, r)| format!("[{:.2}, {:.2}]", t, r))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FluxPhy Transfer Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        :root {{
            --bg-primary: #0f0f23;
            --bg-secondary: #1a1a2e;
            --text-primary: #e0e0e0;
            --accent-cyan: #00d4ff;
            --accent-green: #00ff88;
            --accent-yellow: #ffcc00;
            --accent-magenta: #ff00aa;
        }}
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', system-ui, sans-serif;
            background: linear-gradient(135deg, var(--bg-primary), var(--bg-secondary));
            color: var(--text-primary);
            min-height: 100vh;
            padding: 2rem;
        }}
        .container {{ max-width: 1200px; margin: 0 auto; }}
        header {{
            text-align: center;
            margin-bottom: 2rem;
            padding: 2rem;
            background: rgba(255,255,255,0.05);
            border-radius: 16px;
            border: 1px solid rgba(255,255,255,0.1);
        }}
        h1 {{
            font-size: 2.5rem;
            background: linear-gradient(90deg, var(--accent-cyan), var(--accent-magenta));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 0.5rem;
        }}
        .subtitle {{ color: #888; font-size: 1.1rem; }}
        .grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }}
        .card {{
            background: rgba(255,255,255,0.05);
            border-radius: 12px;
            padding: 1.5rem;
            border: 1px solid rgba(255,255,255,0.1);
        }}
        .card h3 {{
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 1px;
            color: #888;
            margin-bottom: 0.5rem;
        }}
        .card .value {{
            font-size: 2rem;
            font-weight: bold;
        }}
        .card .unit {{ font-size: 1rem; color: #666; }}
        .cyan {{ color: var(--accent-cyan); }}
        .green {{ color: var(--accent-green); }}
        .yellow {{ color: var(--accent-yellow); }}
        .magenta {{ color: var(--accent-magenta); }}
        .chart-container {{
            background: rgba(255,255,255,0.05);
            border-radius: 12px;
            padding: 1.5rem;
            border: 1px solid rgba(255,255,255,0.1);
            margin-bottom: 2rem;
        }}
        .help-section {{
            background: rgba(255,255,255,0.03);
            border-radius: 12px;
            padding: 1.5rem;
            border: 1px solid rgba(255,255,255,0.05);
        }}
        .help-section h2 {{ margin-bottom: 1rem; color: var(--accent-cyan); }}
        .help-item {{
            margin-bottom: 1rem;
            padding: 0.75rem;
            background: rgba(255,255,255,0.02);
            border-radius: 8px;
        }}
        .help-item strong {{ color: var(--accent-yellow); }}
        .help-item .analogy {{ color: var(--accent-green); font-style: italic; }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>⚡ FluxPhy Dashboard</h1>
            <p class="subtitle">Physics of Data Transfer Analysis</p>
        </header>

        <div class="grid">
            <div class="card">
                <h3>📁 File</h3>
                <div class="value cyan">{current_file}</div>
            </div>
            <div class="card">
                <h3>📊 Mean Rate</h3>
                <div class="value green">{mean_rate:.2} <span class="unit">MB/s</span></div>
            </div>
            <div class="card">
                <h3>🚀 Peak Rate</h3>
                <div class="value magenta">{peak_rate:.2} <span class="unit">MB/s</span></div>
            </div>
            <div class="card">
                <h3>⏱️ Duration</h3>
                <div class="value yellow">{elapsed:.1} <span class="unit">seconds</span></div>
            </div>
        </div>

        <div class="grid">
            <div class="card">
                <h3>🌊 Flow Regime</h3>
                <div class="value cyan">{flow_regime}</div>
            </div>
            <div class="card">
                <h3>📈 CV (Variation)</h3>
                <div class="value">{cv:.4}</div>
            </div>
            <div class="card">
                <h3>🎲 Entropy</h3>
                <div class="value">{entropy:.2} <span class="unit">bits</span></div>
            </div>
            <div class="card">
                <h3>⚠️ Bottleneck</h3>
                <div class="value yellow">{bottleneck}</div>
            </div>
        </div>

        <div class="chart-container">
            <canvas id="rateChart"></canvas>
        </div>

        <div class="help-section">
            <h2>📚 What do these metrics mean?</h2>
            <div class="help-item">
                <strong>Flow Regime:</strong> How smooth the transfer is. 
                <span class="analogy">💡 Laminar = calm river, Chaotic = white water rapids</span>
            </div>
            <div class="help-item">
                <strong>CV (Coefficient of Variation):</strong> How much the speed changes. 
                <span class="analogy">💡 Low = steady cruise control, High = stop-and-go traffic</span>
            </div>
            <div class="help-item">
                <strong>Entropy:</strong> Unpredictability of transfer speed. 
                <span class="analogy">💡 Low = predictable weather, High = random storms</span>
            </div>
            <div class="help-item">
                <strong>Bottleneck:</strong> What's limiting your transfer speed. 
                <span class="analogy">💡 Like finding the narrowest part of a water pipe</span>
            </div>
        </div>
    </div>

    <script>
        const ctx = document.getElementById('rateChart').getContext('2d');
        const data = [{rate_data}];
        new Chart(ctx, {{
            type: 'line',
            data: {{
                labels: data.map(d => d[0].toFixed(1) + 's'),
                datasets: [{{
                    label: 'Transfer Rate (MB/s)',
                    data: data.map(d => d[1]),
                    borderColor: '#00d4ff',
                    backgroundColor: 'rgba(0, 212, 255, 0.1)',
                    fill: true,
                    tension: 0.3
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{ labels: {{ color: '#e0e0e0' }} }}
                }},
                scales: {{
                    x: {{ ticks: {{ color: '#888' }}, grid: {{ color: 'rgba(255,255,255,0.1)' }} }},
                    y: {{ ticks: {{ color: '#888' }}, grid: {{ color: 'rgba(255,255,255,0.1)' }} }}
                }}
            }}
        }});
    </script>
</body>
</html>"#,
        current_file = state.current_file,
        mean_rate = state.mean_rate,
        peak_rate = state.peak_rate,
        elapsed = state.elapsed_secs,
        flow_regime = state.flow_regime,
        cv = state.cv,
        entropy = state.entropy,
        bottleneck = state.bottleneck,
        rate_data = rate_data,
    )
}
