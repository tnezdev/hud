use crate::panel::{DashboardState, Panel, PanelState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame<'_>, state: &DashboardState) {
    let area = frame.area();
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);

    draw_header(frame, sections[0], state);
    draw_panels(frame, sections[1], state);
    draw_footer(frame, sections[2], state);
}

fn draw_header(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled(&state.title, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(
            "open -> orient -> refresh -> act",
            Style::default().fg(Color::DarkGray),
        ),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(header, area);
}

fn draw_panels(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    if state.panels.is_empty() {
        let empty = Paragraph::new("No panels configured")
            .block(Block::default().borders(Borders::ALL).title("hud"));
        frame.render_widget(empty, area);
        return;
    }

    let rows = state.panels.len().div_ceil(2);
    let row_constraints = vec![Constraint::Ratio(1, rows as u32); rows];
    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    for row_index in 0..rows {
        let columns_in_row = if state.panels.len() - (row_index * 2) >= 2 {
            2
        } else {
            1
        };
        let column_constraints = vec![Constraint::Ratio(1, columns_in_row as u32); columns_in_row];
        let column_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints)
            .split(row_areas[row_index]);

        for column_index in 0..columns_in_row {
            let panel_index = row_index * 2 + column_index;
            if let Some(panel) = state.panels.get(panel_index) {
                draw_panel(
                    frame,
                    column_areas[column_index],
                    panel,
                    state.focused == panel_index,
                );
            }
        }
    }
}

fn draw_panel(frame: &mut Frame<'_>, area: Rect, panel: &Panel, focused: bool) {
    let border_style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let title = format!(" {} [{}] ", panel.title, panel.state.label());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title);

    let text = match &panel.state {
        PanelState::Idle => "press r to refresh this panel or R for all panels".into(),
        PanelState::Loading => "loading...".into(),
        PanelState::Ready { output } if output.trim().is_empty() => "(no output)".into(),
        PanelState::Ready { output } => output.clone(),
        PanelState::Error(error) => {
            let mut lines = vec![format!("error: {}", error.message)];
            if let Some(detail) = &error.detail {
                lines.push(String::new());
                lines.push(detail.clone());
            }
            lines.join("\n")
        }
    };

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_footer(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let mut footer = vec![
        Span::raw("q quit"),
        Span::raw("  "),
        Span::raw("hjkl/arrow move"),
        Span::raw("  "),
        Span::raw("r refresh"),
        Span::raw("  "),
        Span::raw("R refresh all"),
    ];

    if let Some(panel) = state.focused_panel() {
        for action in &panel.actions {
            footer.push(Span::raw("  "));
            footer.push(Span::styled(
                format!("{} {}", action.key, action.label),
                Style::default().fg(Color::Cyan),
            ));
        }
    }

    if let Some(notice) = &state.notice {
        footer.push(Span::raw("  "));
        footer.push(Span::styled(notice, Style::default().fg(Color::Green)));
    }

    let paragraph = Paragraph::new(Line::from(footer))
        .block(Block::default().borders(Borders::TOP))
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}
