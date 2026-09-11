use crate::panel::{
    DashboardState, MetricsContent, Panel, PanelContent, PanelState, RowDetailView, TableContent,
    View,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Clear, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Wrap,
    },
};
use std::env;

const CARD_PADDING_X: u16 = 1;
const CARD_PADDING_TOP: u16 = 1;
const METRIC_BAR_WIDTH: usize = 18;
const CARD_GUTTER_X: u16 = 1;
const CARD_GUTTER_Y: u16 = 1;

pub fn draw(frame: &mut Frame<'_>, state: &DashboardState) {
    let area = frame.area();
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .split(area);

    draw_header(frame, sections[0], state);
    match state.active_view() {
        View::Dashboard => draw_panel_grid(frame, inset(sections[1], 1, 0), state),
        View::PanelDetail => draw_panel_detail(frame, inset(sections[1], 1, 0), state),
        View::RowDetail => draw_row_detail(frame, inset(sections[1], 1, 0), state),
    }
    draw_footer(frame, sections[2], state);
    if state.help_open {
        draw_help_overlay(frame, area, state);
    }
}

fn draw_row_detail(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    if let Some(row_detail) = &state.row_detail {
        draw_row_detail_with_scroll(frame, area, row_detail);
    }
}

fn draw_header(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let separator = if enhanced_symbols() { "›" } else { ">" };
    let header = Paragraph::new(vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled(" HUD ", key_style()),
            Span::raw("  "),
            Span::styled(
                state.title.to_uppercase(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("{separator} {}", active_context(state)),
                accent_style().add_modifier(Modifier::BOLD),
            ),
        ]),
    ])
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

    if state.panels.len() == 4 && use_mission_control_layout(area) {
        draw_four_panel_grid(frame, area, state);
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
                    inset(column_areas[column_index], CARD_GUTTER_X, CARD_GUTTER_Y),
                    panel,
                    state.focused == panel_index,
                );
            }
        }
    }
}

fn draw_four_panel_grid(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);
    let top_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(row_areas[0]);

    for index in 0..3 {
        if let Some(panel) = state.panels.get(index) {
            draw_panel(
                frame,
                inset(top_areas[index], CARD_GUTTER_X, CARD_GUTTER_Y),
                panel,
                state.focused == index,
            );
        }
    }

    if let Some(panel) = state.panels.get(3) {
        draw_panel(
            frame,
            inset(row_areas[1], CARD_GUTTER_X, CARD_GUTTER_Y),
            panel,
            state.focused == 3,
        );
    }
}

fn draw_panel_detail(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    if let Some(panel) = state.focused_panel() {
        draw_panel_with_scroll(frame, area, panel, true);
    }
}

fn draw_panel(frame: &mut Frame<'_>, area: Rect, panel: &Panel, focused: bool) {
    let block = panel_block(panel, focused);

    if let PanelState::Ready {
        content: PanelContent::Metrics(metrics),
    } = &panel.state
    {
        draw_metrics_panel(frame, area, block, metrics);
        return;
    }

    let text = panel_preview_text(panel);

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_panel_with_scroll(frame: &mut Frame<'_>, area: Rect, panel: &Panel, focused: bool) {
    let block = panel_block(panel, focused);
    if let PanelState::Ready {
        content: PanelContent::Table(table),
    } = &panel.state
    {
        draw_table_panel(frame, area, block, panel, table);
        return;
    }

    if let PanelState::Ready {
        content: PanelContent::Metrics(metrics),
    } = &panel.state
    {
        draw_metrics_panel(frame, area, block, metrics);
        return;
    }

    let text_width = area.width.saturating_sub(2 + (CARD_PADDING_X * 2)) as usize;
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
    let widths = table_render_widths(table);
    let header = Row::new(table.columns.iter().map(|column| {
        Cell::from(column.clone()).style(Style::default().add_modifier(Modifier::BOLD))
    }))
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
        .row_highlight_style(selection_style());
    frame.render_stateful_widget(widget, area, &mut table_state);
}

fn draw_metrics_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    block: Block<'_>,
    metrics: &MetricsContent,
) {
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if metrics.metrics.is_empty() || inner.height == 0 {
        return;
    }

    let label_width = metrics
        .metrics
        .iter()
        .map(|metric| metric.label.chars().count())
        .max()
        .unwrap_or(10)
        .clamp(10, 22);
    let available_bar_width = inner.width.saturating_sub(label_width as u16 + 8).max(8) as usize;
    let bar_width = available_bar_width.min(METRIC_BAR_WIDTH);

    let lines = metric_lines(metrics, label_width, bar_width, inner.height as usize);

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, inner);
}

fn draw_row_detail_with_scroll(frame: &mut Frame<'_>, area: Rect, row_detail: &RowDetailView) {
    let title = format!(" {} [{}] ", row_detail.title, row_detail.state.label());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(panel_border(true))
        .border_style(accent_style().add_modifier(Modifier::BOLD))
        .padding(Padding::new(
            CARD_PADDING_X,
            CARD_PADDING_X,
            CARD_PADDING_TOP,
            0,
        ))
        .title(title);
    let text = panel_state_text(&row_detail.state);
    let lines = text
        .lines()
        .map(|line| Line::raw(line.to_string()))
        .collect::<Vec<_>>();
    let content_lines = wrapped_line_count(&lines, area.width.saturating_sub(2) as usize);
    let visible_lines = area.height.saturating_sub(2) as usize;
    let max_scroll = content_lines.saturating_sub(visible_lines);
    let effective_offset = row_detail.scroll_offset.min(max_scroll);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((effective_offset.min(u16::MAX as usize) as u16, 0));
    frame.render_widget(paragraph, area);

    if content_lines > visible_lines {
        let mut scrollbar_state = ScrollbarState::new(content_lines.saturating_sub(visible_lines))
            .position(effective_offset);
        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
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
    panel_state_text(&panel.state)
}

fn panel_state_text(state: &PanelState) -> String {
    match state {
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
        PanelState::Ready {
            content: PanelContent::Metrics(metrics),
        } => metrics_preview_text(metrics),
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
        PanelState::Ready {
            content: PanelContent::Metrics(metrics),
        } => metrics_preview_text(metrics),
        _ => panel_text(panel),
    }
}

fn table_preview_text(table: &TableContent) -> String {
    if table.rows.is_empty() {
        return format!("{} columns, no rows", table.columns.len());
    }

    let widths = table_column_widths(table);
    let mut lines = vec![format_table_row(&table.columns, &widths)];
    lines.extend(
        table
            .rows
            .iter()
            .take(4)
            .map(|row| format_table_row(row, &widths)),
    );
    if table.rows.len() > 4 {
        lines.push(format!("... {} more rows", table.rows.len() - 4));
    }
    lines.join("\n")
}

fn metrics_preview_text(metrics: &MetricsContent) -> String {
    metric_lines(metrics, 10, 12, metrics.metrics.len().saturating_mul(2))
        .into_iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn metric_lines(
    metrics: &MetricsContent,
    label_width: usize,
    bar_width: usize,
    visible_height: usize,
) -> Vec<Line<'static>> {
    let spaced = visible_height >= metrics.metrics.len().saturating_mul(2).saturating_sub(1);
    let mut lines = Vec::new();

    for metric in &metrics.metrics {
        if lines.len() >= visible_height {
            break;
        }
        if spaced && !lines.is_empty() {
            lines.push(Line::raw(""));
        }
        if lines.len() >= visible_height {
            break;
        }
        lines.push(metric_line(metric, label_width, bar_width));
    }

    lines
}

fn metric_line(
    metric: &crate::panel::MetricContent,
    label_width: usize,
    bar_width: usize,
) -> Line<'static> {
    let percent = metric.value.saturating_mul(100) / metric.max;
    let filled = ((metric.value as f64 / metric.max as f64) * bar_width as f64).round() as usize;
    let filled = filled.min(bar_width);
    let (fill, empty) = bar_symbols();
    let filled_bar = fill.repeat(filled);
    let empty_bar = empty.repeat(bar_width.saturating_sub(filled));

    Line::from(vec![
        Span::styled(
            format!("{:<label_width$}", metric.label),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(filled_bar, accent_style().add_modifier(Modifier::BOLD)),
        Span::styled(empty_bar, muted_style()),
        Span::raw("  "),
        Span::styled(format!("{:>3}%", percent), muted_style()),
    ])
}

fn bar_symbols() -> (&'static str, &'static str) {
    if env::var_os("HUD_ASCII_BARS").is_some() || !enhanced_symbols() {
        ("#", "-")
    } else {
        ("█", "░")
    }
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
        PanelState::Ready {
            content: PanelContent::Metrics(metrics),
        } => metrics_preview_text(metrics)
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
        Line::styled(text.to_string(), selection_style())
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
    let mut footer = Vec::new();
    push_key_hint(&mut footer, "q", "quit");
    push_key_hint(&mut footer, "?", "help/actions");
    push_key_hint(&mut footer, "r", "refresh");
    push_key_hint(&mut footer, "R", "refresh all");

    if state.active_view() == View::PanelDetail {
        push_key_hint(&mut footer, "Esc", "grid");
    }

    if state.active_view() == View::RowDetail {
        push_key_hint(&mut footer, "Esc", "back");
    }

    if let Some(notice) = &state.notice {
        footer.push(Span::raw("  "));
        footer.push(Span::styled(
            notice.clone(),
            accent_style().add_modifier(Modifier::BOLD),
        ));
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
            accent_style().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::from(vec![
            Span::styled("Panel: ", muted_style()),
            Span::raw(&panel.title),
            Span::raw("  "),
            Span::styled(
                panel.state.label(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::raw(""),
        Line::styled("Global", accent_style().add_modifier(Modifier::BOLD)),
        Line::raw("? toggle this overlay"),
        Line::raw("r refresh focused panel"),
        Line::raw("R refresh all panels"),
    ];

    match state.active_view() {
        View::Dashboard => {
            lines.push(Line::raw("h/j/k/l or arrows move through cards"));
            lines.push(Line::raw("Enter drill into focused panel"));
            lines.push(Line::raw("q or Esc quit"));
        }
        View::PanelDetail => {
            lines.push(Line::raw("j/k or down/up select output row"));
            lines.push(Line::raw("Enter open configured row detail"));
            lines.push(Line::raw("q, x, or Esc return to dashboard"));
        }
        View::RowDetail => {
            lines.push(Line::raw("j/k or down/up scroll row detail"));
            lines.push(Line::raw("q, x, or Esc return to panel detail"));
        }
    }

    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "Panel Actions",
        accent_style().add_modifier(Modifier::BOLD),
    ));
    if panel.actions.is_empty() {
        lines.push(Line::styled("none", muted_style()));
    } else {
        for action in &panel.actions {
            lines.push(Line::from(vec![
                Span::styled(format!(" {} ", action.key), key_style()),
                Span::raw(format!(" {}", action.label)),
            ]));
        }
    }

    let overlay = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(panel_border(true))
                .border_style(accent_style().add_modifier(Modifier::BOLD))
                .title(" ? help/actions "),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(overlay, popup);
}

fn panel_block(panel: &Panel, focused: bool) -> Block<'static> {
    let mut title = Vec::new();
    if focused {
        title.push(Span::styled(
            format!(" {} ", focus_mark()),
            accent_style().add_modifier(Modifier::BOLD),
        ));
    }
    title.push(Span::styled(
        format!("{} ", panel.title),
        Style::default().add_modifier(Modifier::BOLD),
    ));
    title.push(Span::styled(
        format!("{} ", state_badge(&panel.state)),
        status_style(&panel.state),
    ));
    let title = Line::from(title);
    let border_style = if focused {
        accent_style().add_modifier(Modifier::BOLD)
    } else {
        muted_style()
    };

    Block::default()
        .borders(Borders::ALL)
        .border_set(panel_border(focused))
        .border_style(border_style)
        .padding(Padding::new(
            CARD_PADDING_X,
            CARD_PADDING_X,
            CARD_PADDING_TOP,
            0,
        ))
        .title(title)
}

fn status_style(state: &PanelState) -> Style {
    match state {
        PanelState::Idle | PanelState::Ready { .. } => muted_style(),
        PanelState::Loading => accent_style().add_modifier(Modifier::BOLD),
        PanelState::Error(_) if colors_enabled() => {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        }
        PanelState::Error(_) => Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED),
    }
}

fn state_badge(state: &PanelState) -> String {
    let enhanced = enhanced_symbols();
    match state {
        PanelState::Idle => format!("{} idle", if enhanced { "○" } else { "o" }),
        PanelState::Loading => format!("{} loading", if enhanced { "◌" } else { "~" }),
        PanelState::Ready { .. } => format!("{} ready", if enhanced { "✓" } else { "+" }),
        PanelState::Error(_) => "! error".into(),
    }
}

fn active_context(state: &DashboardState) -> String {
    match state.active_view() {
        View::Dashboard => "DASHBOARD".into(),
        View::PanelDetail => state
            .focused_panel()
            .map(|panel| format!("DETAIL / {}", panel.title.to_uppercase()))
            .unwrap_or_else(|| "DETAIL".into()),
        View::RowDetail => state
            .row_detail
            .as_ref()
            .map(|detail| format!("ROW / {}", detail.title.to_uppercase()))
            .unwrap_or_else(|| "ROW DETAIL".into()),
    }
}

fn use_mission_control_layout(area: Rect) -> bool {
    area.width >= 96
}

fn colors_enabled() -> bool {
    env::var_os("NO_COLOR").is_none_or(|value| value.is_empty())
}

fn enhanced_symbols() -> bool {
    env::var_os("HUD_ASCII_UI").is_none()
}

fn accent_style() -> Style {
    if colors_enabled() {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    }
}

fn muted_style() -> Style {
    if colors_enabled() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().add_modifier(Modifier::DIM)
    }
}

fn key_style() -> Style {
    if colors_enabled() {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .add_modifier(Modifier::REVERSED)
            .add_modifier(Modifier::BOLD)
    }
}

fn selection_style() -> Style {
    key_style().add_modifier(Modifier::BOLD)
}

fn focus_mark() -> &'static str {
    if enhanced_symbols() { "◆" } else { ">" }
}

fn panel_border(focused: bool) -> symbols::border::Set<'static> {
    if !enhanced_symbols() {
        symbols::border::Set {
            top_left: "+",
            top_right: "+",
            bottom_left: "+",
            bottom_right: "+",
            vertical_left: "|",
            vertical_right: "|",
            horizontal_top: "-",
            horizontal_bottom: "-",
        }
    } else if focused {
        symbols::border::THICK
    } else {
        symbols::border::ROUNDED
    }
}

fn push_key_hint(footer: &mut Vec<Span<'static>>, key: &str, label: &str) {
    if !footer.is_empty() {
        footer.push(Span::raw("  "));
    }
    footer.push(Span::styled(format!(" {key} "), key_style()));
    footer.push(Span::raw(format!(" {label}")));
}

fn table_column_widths(table: &TableContent) -> Vec<usize> {
    table
        .columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            table
                .rows
                .iter()
                .filter_map(|row| row.get(index))
                .map(|cell| cell.chars().count())
                .chain(std::iter::once(column.chars().count()))
                .max()
                .unwrap_or(0)
                .min(18)
        })
        .collect()
}

fn table_render_widths(table: &TableContent) -> Vec<Constraint> {
    let expandable_column = table
        .columns
        .iter()
        .position(|column| column.eq_ignore_ascii_case("description"))
        .unwrap_or_else(|| table.columns.len().saturating_sub(1));

    table
        .columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            if index == expandable_column {
                return Constraint::Min(column.chars().count().max(20) as u16);
            }

            let width = table
                .rows
                .iter()
                .filter_map(|row| row.get(index))
                .map(|cell| cell.chars().count())
                .chain(std::iter::once(column.chars().count()))
                .max()
                .unwrap_or(8)
                .clamp(4, 18) as u16;
            Constraint::Length(width)
        })
        .collect()
}

fn format_table_row(row: &[String], widths: &[usize]) -> String {
    row.iter()
        .zip(widths.iter())
        .map(|(cell, width)| format!("{cell:<width$}"))
        .collect::<Vec<_>>()
        .join("  ")
}

fn inset(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    Rect::new(
        area.x.saturating_add(horizontal),
        area.y.saturating_add(vertical),
        area.width.saturating_sub(horizontal.saturating_mul(2)),
        area.height.saturating_sub(vertical.saturating_mul(2)),
    )
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
    use ratatui::layout::Constraint;

    #[test]
    fn wrapped_line_count_includes_visual_wraps() {
        assert_eq!(wrapped_line_count(&[Line::raw("short")], 10), 1);
        assert_eq!(wrapped_line_count(&[Line::raw("12345678901")], 10), 2);
        assert_eq!(
            wrapped_line_count(&[Line::raw("12345678901"), Line::raw("short")], 10),
            3
        );
    }

    #[test]
    fn table_render_widths_expand_description_column() {
        let table = TableContent {
            columns: vec![
                "ID".into(),
                "Project".into(),
                "Pri".into(),
                "Description".into(),
            ],
            rows: vec![vec![
                "114".into(),
                "daemon".into(),
                "H".into(),
                "Long task description should get remaining width".into(),
            ]],
        };

        assert_eq!(
            table_render_widths(&table),
            vec![
                Constraint::Length(4),
                Constraint::Length(7),
                Constraint::Length(4),
                Constraint::Min(20),
            ]
        );
    }

    #[test]
    fn mission_control_layout_requires_room_for_three_instruments() {
        assert!(!use_mission_control_layout(Rect::new(0, 0, 95, 24)));
        assert!(use_mission_control_layout(Rect::new(0, 0, 96, 24)));
    }
}
