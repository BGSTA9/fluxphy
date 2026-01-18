//! Help panel with "dummy-friendly" metric explanations

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Metric explanation for non-technical users
pub struct MetricHelp {
    pub name: &'static str,
    pub emoji: &'static str,
    pub explanation: &'static str,
    pub analogy: &'static str,
}

/// Get all metric explanations
pub fn get_metric_explanations() -> Vec<MetricHelp> {
    vec![
        MetricHelp {
            name: "Flux Rate",
            emoji: "📊",
            explanation: "How fast data is being copied right now",
            analogy: "Like the speed on your car's speedometer",
        },
        MetricHelp {
            name: "Mean Rate",
            emoji: "📈",
            explanation: "The average speed over the whole transfer",
            analogy: "Your average speed on a road trip",
        },
        MetricHelp {
            name: "Flow Regime",
            emoji: "🌊",
            explanation: "How smooth or bumpy the transfer is",
            analogy: "Laminar = smooth river, Chaotic = rapids",
        },
        MetricHelp {
            name: "CV (Coefficient of Variation)",
            emoji: "📐",
            explanation: "How much the speed changes during transfer",
            analogy: "Low = steady cruise, High = stop-and-go traffic",
        },
        MetricHelp {
            name: "Entropy",
            emoji: "🎲",
            explanation: "How unpredictable the transfer speed is",
            analogy: "Low = predictable weather, High = random storms",
        },
        MetricHelp {
            name: "Thermal Stability",
            emoji: "🌡️",
            explanation: "How consistent the system performance is",
            analogy: "1.0 = perfectly stable, 0.0 = very unstable",
        },
        MetricHelp {
            name: "Bottleneck",
            emoji: "⚠️",
            explanation: "What's slowing down your transfer the most",
            analogy: "Like finding the narrowest part of a pipe",
        },
        MetricHelp {
            name: "ETA",
            emoji: "⏱️",
            explanation: "Estimated time until the transfer finishes",
            analogy: "Like 'Arriving in 5 minutes' on GPS",
        },
    ]
}

/// Render the help overlay
pub fn render_help_overlay(frame: &mut Frame, area: Rect) {
    // Clear the area first
    frame.render_widget(Clear, area);
    
    let explanations = get_metric_explanations();
    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled(
                "📚 FluxPhy Metrics Guide",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Press H to close this help panel",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(""),
    ];

    for help in explanations {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} {}", help.emoji, help.name),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(help.explanation, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  💡 "),
            Span::styled(help.analogy, Style::default().fg(Color::Green)),
        ]));
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
