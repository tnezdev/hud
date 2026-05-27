use crate::panel::{DashboardState, Panel, PanelContent, PanelState, TableContent, View};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Wrap,
    },
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
    match state.view {
        View::Dashboard => draw_panel_grid(frame, sections[1], state),
        View::PanelDetail => draw_panel_detail(frame, sections[1], state),
    }
    draw_footer(frame, sections[2], state);
    if state.help_open {
        draw_help_overlay(frame, area, state);
    }
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

fn draw_panel_grid(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
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

fn draw_panel_detail(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    if let Some(panel) = state.focused_panel() {
        draw_panel_with_scroll(frame, area, panel, true);
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

    let text = panel_preview_text(panel);

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_panel_with_scroll(frame: &mut Frame<'_>, area: Rect, panel: &Panel, focused: bool) {
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
    if let PanelState::Ready {
        content: PanelContent::Table(table),
    } = &panel.state
    {
        draw_table_panel(frame, area, block, panel, table);
        return;
    }

    let text_width = area.width.saturating_sub(2) as usize;
    let text = panel_detail_lines(panel);
    let selected_visual_offset = row_visual_offset(&text, panel.selected_row, text_width);
    let content_lines = wrapped_line_count(&text, text_width);
    let visible_lines = area.height.saturating_sub(2) as usize;
    let max_scroll = content_lines.saturating_sub(visible_lines);
    let effective_offset = selection_following_scroll_offset(
        panel.scroll_offset,
        selected_visual_offset,
        visible_lines,
        max_scroll,
    );
    let scroll_offset = effective_offset.min(u16::MAX as usize) as u16;
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));
    frame.render_widget(paragraph, area);

    if content_lines > visible_lines {
        let mut scrollbar_state = ScrollbarState::new(content_lines.saturating_sub(visible_lines))
            .position(effective_offset);
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

fn draw_table_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    block: Block<'_>,
    panel: &Panel,
    table: &TableContent,
) {
    let widths = table
        .columns
        .iter()
        .map(|_| Constraint::Min(8))
        .collect::<Vec<_>>();
    let header = Row::new(
        table
            .columns
            .iter()
            .map(|column| Cell::from(column.clone()).style(Style::default().fg(Color::Yellow))),
    )
    .bottom_margin(1);
    let rows = table.rows.iter().map(|row| {
        Row::new(
            row.iter()
                .map(|cell| Cell::from(cell.clone()))
                .collect::<Vec<_>>(),
        )
    });
    let selected = (!table.rows.is_empty()).then_some(panel.selected_row.min(table.rows.len() - 1));
    let mut table_state = TableState::default().with_selected(selected);
    let widget = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_stateful_widget(widget, area, &mut table_state);
}

fn wrapped_line_count(lines: &[Line<'_>], width: usize) -> usize {
    if width == 0 {
        return lines.len();
    }

    lines
        .iter()
        .map(|line| wrapped_line_height(line, width))
        .sum()
}

fn panel_text(panel: &Panel) -> String {
    match &panel.state {
        PanelState::Idle => "press r to refresh this panel or R for all panels".into(),
        PanelState::Loading => "loading...".into(),
        PanelState::Ready {
            content: PanelContent::Text(output),
        } if output.trim().is_empty() => "(no output)".into(),
        PanelState::Ready {
            content: PanelContent::Text(output),
        } => output.clone(),
        PanelState::Ready {
            content: PanelContent::Table(table),
        } => table_preview_text(table),
        PanelState::Error(error) => {
            let mut lines = vec![format!("error: {}", error.message)];
            if let Some(detail) = &error.detail {
                lines.push(String::new());
                lines.push(detail.clone());
            }
            lines.join("\n")
        }
    }
}

fn panel_preview_text(panel: &Panel) -> String {
    match &panel.state {
        PanelState::Ready {
            content: PanelContent::Table(table),
        } => table_preview_text(table),
        _ => panel_text(panel),
    }
}

fn table_preview_text(table: &TableContent) -> String {
    if table.rows.is_empty() {
        return format!("{} columns, no rows", table.columns.len());
    }

    let mut lines = vec![table.columns.join("  ")];
    lines.extend(table.rows.iter().take(4).map(|row| row.join("  ")));
    if table.rows.len() > 4 {
        lines.push(format!("... {} more rows", table.rows.len() - 4));
    }
    lines.join("\n")
}

fn panel_detail_lines(panel: &Panel) -> Vec<Line<'static>> {
    match &panel.state {
        PanelState::Ready {
            content: PanelContent::Text(output),
        } if output.is_empty() => {
            vec![selected_line("(no output)", panel.selected_row == 0)]
        }
        PanelState::Ready {
            content: PanelContent::Text(output),
        } => output
            .lines()
            .enumerate()
            .map(|(index, line)| selected_line(line, index == panel.selected_row))
            .collect(),
        PanelState::Ready {
            content: PanelContent::Table(table),
        } => table_preview_text(table)
            .lines()
            .map(|line| Line::raw(line.to_string()))
            .collect(),
        _ => panel_text(panel)
            .lines()
            .map(|line| Line::raw(line.to_string()))
            .collect(),
    }
}

fn selected_line(text: &str, selected: bool) -> Line<'static> {
    if selected {
        Line::styled(
            text.to_string(),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Line::raw(text.to_string())
    }
}

fn selection_following_scroll_offset(
    current_offset: usize,
    selected_visual_offset: usize,
    visible_lines: usize,
    max_scroll: usize,
) -> usize {
    if visible_lines == 0 {
        return 0;
    }

    let offset = current_offset.min(max_scroll);
    if selected_visual_offset < offset {
        selected_visual_offset.min(max_scroll)
    } else if selected_visual_offset >= offset.saturating_add(visible_lines) {
        selected_visual_offset
            .saturating_add(1)
            .saturating_sub(visible_lines)
            .min(max_scroll)
    } else {
        offset
    }
}

fn row_visual_offset(lines: &[Line<'_>], selected_row: usize, width: usize) -> usize {
    lines
        .iter()
        .take(selected_row)
        .map(|line| wrapped_line_height(line, width))
        .sum()
}

fn wrapped_line_height(line: &Line<'_>, width: usize) -> usize {
    if width == 0 {
        1
    } else {
        line.width().max(1).div_ceil(width)
    }
}

fn draw_footer(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let mut footer = vec![
        Span::raw("q quit"),
        Span::raw("  "),
        Span::raw("? help/actions"),
        Span::raw("  "),
        Span::raw("r refresh"),
        Span::raw("  "),
        Span::raw("R refresh all"),
    ];

    if state.view == View::PanelDetail {
        footer.push(Span::raw("  "));
        footer.push(Span::styled(
            "q/x/Esc grid",
            Style::default().fg(Color::Yellow),
        ));
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

fn draw_help_overlay(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let popup = centered_rect(area, 64, 18);
    frame.render_widget(Clear, popup);

    let Some(panel) = state.focused_panel() else {
        return;
    };

    let mut lines = vec![
        Line::styled(
            "Help / Actions",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::from(vec![
            Span::styled("Panel: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&panel.title),
            Span::raw("  "),
            Span::styled(panel.state.label(), Style::default().fg(Color::Green)),
        ]),
        Line::raw(""),
        Line::styled("Global", Style::default().fg(Color::Yellow)),
        Line::raw("? toggle this overlay"),
        Line::raw("r refresh focused panel"),
        Line::raw("R refresh all panels"),
    ];

    match state.view {
        View::Dashboard => {
            lines.push(Line::raw("h/j/k/l or arrows move through cards"));
            lines.push(Line::raw("Enter drill into focused panel"));
            lines.push(Line::raw("q or Esc quit"));
        }
        View::PanelDetail => {
            lines.push(Line::raw("j/k or down/up select output row"));
            lines.push(Line::raw("q, x, or Esc return to dashboard"));
        }
    }

    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "Panel Actions",
        Style::default().fg(Color::Cyan),
    ));
    if panel.actions.is_empty() {
        lines.push(Line::styled("none", Style::default().fg(Color::DarkGray)));
    } else {
        for action in &panel.actions {
            lines.push(Line::raw(format!("{} {}", action.key, action.label)));
        }
    }

    let overlay = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" ? help/actions "),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(overlay, popup);
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width.saturating_sub(2));
    let height = height.min(area.height.saturating_sub(2));
    Rect::new(
        area.x + area.width.saturating_sub(width) / 2,
        area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapped_line_count_includes_visual_wraps() {
        assert_eq!(wrapped_line_count(&[Line::raw("short")], 10), 1);
        assert_eq!(wrapped_line_count(&[Line::raw("12345678901")], 10), 2);
        assert_eq!(
            wrapped_line_count(&[Line::raw("12345678901"), Line::raw("short")], 10),
            3
        );
    }
}
