//! Example-only Sequence contract. Data in; presentation out. Not a production API.
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame, Terminal,
    backend::TestBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;
use std::{env, fs::File, io::Read, process::ExitCode};

const MAX_BYTES: usize = 65_536;
const HEADER_ROWS: u16 = 6; // outer top padding, title, gap, source, scope, gap

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sequence {
    title: String,
    provenance: String,
    scope: String,
    intent: String,
    relationship: Relationship,
    items: Vec<Item>,
    #[serde(default)]
    notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Relationship {
    Procedure,
    Chronology,
}

impl Relationship {
    fn heading(&self) -> &'static str {
        match self {
            Self::Procedure => "──── Procedure · not execution status",
            Self::Chronology => "──── Chronology · not causation",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Item {
    label: String,
    #[serde(default)]
    detail: Vec<String>,
}

fn validate_text(text: &str) -> Result<(), String> {
    if text.trim().is_empty() || text.chars().count() > 500 {
        return Err("text must contain 1..500 characters, not only whitespace".into());
    }
    if text.chars().any(|c| {
        c.is_control()
            || matches!(c, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
    }) {
        return Err("control characters and bidi controls are not display text".into());
    }
    Ok(())
}

fn validate_lines(lines: &[String]) -> Result<(), String> {
    if lines.len() > 12 {
        return Err("detail and notes allow at most 12 entries".into());
    }
    for line in lines {
        validate_text(line)?;
    }
    Ok(())
}

fn parse(source: &str) -> Result<Sequence, String> {
    if source.len() > MAX_BYTES {
        return Err("sequence exceeds 64 KiB".into());
    }
    let sequence: Sequence = serde_json::from_str(source).map_err(|e| e.to_string())?;
    for text in [&sequence.title, &sequence.provenance, &sequence.scope] {
        validate_text(text)?;
        if Span::raw(text).width() > 60 {
            return Err("title, provenance and scope are limited to 60 display cells each".into());
        }
    }
    validate_text(&sequence.intent)?;
    if !(2..=12).contains(&sequence.items.len()) {
        return Err("Sequence requires 2..12 items".into());
    }
    for item in &sequence.items {
        validate_text(&item.label)?;
        validate_lines(&item.detail)?;
    }
    validate_lines(&sequence.notes)?;
    Ok(sequence)
}

// The only data input edge. Bounded bytes, not elapsed time for arbitrary devices.
fn load(path: &str) -> Result<Sequence, String> {
    let mut source = String::new();
    File::open(path)
        .map_err(|e| format!("{path}: {e}"))?
        .take(MAX_BYTES as u64 + 1)
        .read_to_string(&mut source)
        .map_err(|e| format!("{path}: {e}"))?;
    parse(&source)
}

// A small linear render plan, not an author-facing layout tree. Ratatui alone
// measures and wraps each paragraph inside its text column.
struct Block<'a> {
    text: &'a str,
    item: Option<usize>,
    label: bool,
    // A presentation landmark attached to its first paragraph, never a new
    // authoring field or a separately scrollable heading.
    section: Option<&'static str>,
}

fn gap(item: Option<usize>) -> Block<'static> {
    Block {
        text: "",
        item,
        label: false,
        section: None,
    }
}

fn blocks(sequence: &Sequence) -> Vec<Block<'_>> {
    let mut blocks = vec![
        Block {
            text: &sequence.intent,
            item: None,
            label: false,
            section: None,
        },
        gap(None),
        gap(None),
    ];
    for (index, item) in sequence.items.iter().enumerate() {
        blocks.push(Block {
            text: &item.label,
            item: Some(index),
            label: true,
            section: (index == 0).then_some(sequence.relationship.heading()),
        });
        for detail in &item.detail {
            blocks.push(Block {
                text: detail,
                item: Some(index),
                label: false,
                section: None,
            });
        }
        if index + 1 < sequence.items.len() {
            blocks.push(gap(Some(index)));
        }
    }
    if !sequence.notes.is_empty() {
        blocks.push(gap(None));
        blocks.push(gap(None));
        for (index, note) in sequence.notes.iter().enumerate() {
            blocks.push(Block {
                text: note,
                item: None,
                label: false,
                section: (index == 0).then_some("──── Background and sources"),
            });
            blocks.push(gap(None));
        }
        blocks.pop();
    }
    blocks
}

fn paragraph(block: &Block<'_>) -> Paragraph<'static> {
    let style = if block.label {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let mut lines = Vec::new();
    if let Some(section) = block.section {
        lines.push(Line::styled(
            section,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    }
    lines.push(Line::styled(block.text.to_owned(), style));
    Paragraph::new(lines).wrap(Wrap { trim: false })
}

#[derive(Debug, Default, PartialEq)]
struct Viewport {
    offset: u16,
    max_offset: u16,
    page: u16,
    positions: Vec<u16>,
}

// One positioning rule for keys, arbitrary previews, End, and resize/reflow.
// Fitted paragraphs have only their start as a legal position. Oversized ones
// permit interior positions with one row reserved for explicit earlier context.
fn positions(heights: &[usize], page: u16) -> Vec<u16> {
    let total: usize = heights.iter().sum();
    let mut result = Vec::new();
    let mut start = 0;
    for &height in heights {
        let count = if height > page as usize { height } else { 1 };
        for delta in 0..count {
            let offset = start + delta;
            result.push(offset as u16);
            let available = page as usize - usize::from(delta > 0);
            if total - offset <= available {
                return result;
            }
        }
        start += height;
    }
    result
}

fn draw(frame: &mut Frame, sequence: &Sequence, requested: u16) -> Viewport {
    let area = frame.area();
    if area.width < 64 || area.height < 12 {
        frame.render_widget(
            Paragraph::new("Sequence needs 64 columns x 12 rows. Resize; q quits.")
                .wrap(Wrap { trim: false }),
            area,
        );
        return Viewport::default();
    }
    let [header, content, footer] = Layout::vertical([
        Constraint::Length(HEADER_ROWS),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(area);
    let inset = |r: Rect| Rect::new(r.x + 2, r.y, r.width - 4, r.height);
    let mut content = inset(content);
    let accent = Style::default().fg(Color::Cyan);
    frame.render_widget(
        Paragraph::new(vec![
            Line::default(),
            Line::styled(
                &sequence.title,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Line::default(),
            Line::raw(&sequence.provenance),
            Line::raw(&sequence.scope),
        ]),
        inset(header),
    );
    let blocks = blocks(sequence);
    let body_width = content.width;
    let width = |b: &Block<'_>| body_width - if b.item.is_some() { 6 } else { 0 };
    let heights: Vec<usize> = blocks
        .iter()
        .map(|b| paragraph(b).line_count(width(b)).max(1))
        .collect();
    let total: usize = heights.iter().sum();
    // Validated input bounds keep this well below u16::MAX at the minimum width.
    let positions = positions(&heights, content.height);
    let max_offset = positions.last().copied().unwrap_or(0);
    let offset = positions
        .iter()
        .rev()
        .find(|&&p| p <= requested)
        .copied()
        .unwrap_or(0);
    let mut boundary = 0;
    let earlier = heights.iter().any(|&height| {
        let clipped = offset as usize > boundary && (offset as usize) < boundary + height;
        boundary += height;
        clipped
    });
    if earlier {
        frame.render_widget(
            Paragraph::new("… Partial paragraph: earlier text above; not standalone"),
            Rect::new(content.x, content.y, content.width, 1),
        );
        content.y += 1;
        content.height -= 1;
    }
    let mut start = 0;
    let mut continues = false;
    let mut page = content.height;
    for (block, height) in blocks.iter().zip(heights) {
        let end = start + height;
        let visible_start = start.max(offset as usize);
        let visible_end = end.min(offset as usize + content.height as usize);
        if visible_start < visible_end {
            let skipped = visible_start - start;
            let y = content.y + (visible_start - offset as usize) as u16;
            let shown = (visible_end - visible_start) as u16;
            let gutter = if block.item.is_some() { 6 } else { 0 };
            if block.section.is_some() && skipped == 0 && shown == 1 {
                // Do not strand a section landmark in the last row. PageDown
                // must reach this deferred unit rather than skip its beginning.
                page = (visible_start - offset as usize) as u16;
                break;
            }
            continues |= visible_end < end;
            let heading_rows = usize::from(block.section.is_some());
            let heading_visible = u16::from(heading_rows > 0 && skipped == 0);
            if let Some(section) = block.section.filter(|_| heading_visible > 0) {
                // All section landmarks share the outer margin, not the item
                // text indentation. Generated headings fit even at minimum width.
                frame.render_widget(
                    Paragraph::new(section).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Rect::new(content.x, y, body_width, 1),
                );
            }
            let text_block = Block {
                section: None,
                ..*block
            };
            frame.render_widget(
                paragraph(&text_block).scroll((skipped.saturating_sub(heading_rows) as u16, 0)),
                Rect::new(
                    content.x + gutter,
                    y + heading_visible,
                    width(block),
                    shown - heading_visible,
                ),
            );
            if let Some(index) = block.item {
                for row in 0..shown {
                    let paragraph_row = skipped + row as usize;
                    if paragraph_row < heading_rows {
                        continue;
                    }
                    let marker = if block.label && paragraph_row == heading_rows {
                        format!("{:02} ●", index + 1)
                    } else if y + row == content.y {
                        // Identify detail ownership; an interior fragment is explicit.
                        format!("{:02} {}", index + 1, if earlier { "…" } else { "│" })
                    } else {
                        "   │".into()
                    };
                    frame.render_widget(
                        Paragraph::new(marker).style(accent),
                        Rect::new(content.x, y + row, 5, 1),
                    );
                }
            }
        }
        start = end;
    }
    let more = if offset < max_offset {
        "MORE below"
    } else {
        "END"
    };
    let footer_text = if continues {
        format!(
            "MORE below: paragraph continues | line {}/{} | j/k | q",
            offset + 1,
            total
        )
    } else {
        format!(
            "{more} | line {}/{} | j/k PgDn Home/End | q quit",
            offset + 1,
            total
        )
    };
    frame.render_widget(Paragraph::new(footer_text), inset(footer));
    Viewport {
        offset,
        max_offset,
        page,
        positions,
    }
}

fn scroll(view: &Viewport, key: KeyCode) -> u16 {
    let next = || {
        view.positions
            .iter()
            .find(|&&p| p > view.offset)
            .copied()
            .unwrap_or(view.offset)
    };
    let previous = || {
        view.positions
            .iter()
            .rev()
            .find(|&&p| p < view.offset)
            .copied()
            .unwrap_or(view.offset)
    };
    match key {
        KeyCode::Down | KeyCode::Char('j') => next(),
        KeyCode::Up | KeyCode::Char('k') => previous(),
        KeyCode::PageDown => view
            .positions
            .iter()
            .rev()
            .find(|&&p| p <= view.offset.saturating_add(view.page))
            .copied()
            .unwrap_or(view.offset),
        KeyCode::PageUp => view
            .positions
            .iter()
            .find(|&&p| {
                p >= view.offset.saturating_sub(view.page.saturating_sub(1)) && p < view.offset
            })
            .copied()
            .unwrap_or_else(previous),
        KeyCode::Home => 0,
        KeyCode::End => view.max_offset,
        _ => view.offset,
    }
}

fn preview(sequence: &Sequence, width: u16, height: u16, offset: u16) -> Result<String, String> {
    if !(1..=240).contains(&width) || !(1..=100).contains(&height) {
        return Err("preview dimensions must be 1..240 columns and 1..100 rows".into());
    }
    let mut terminal = Terminal::new(TestBackend::new(width, height)).map_err(|e| e.to_string())?;
    terminal
        .draw(|frame| {
            draw(frame, sequence, offset);
        })
        .map_err(|e| e.to_string())?;
    let buffer = terminal.backend().buffer();
    let mut output = String::new();
    for y in 0..height {
        let mut line = String::new();
        let mut x = 0;
        while x < width {
            let symbol = buffer[(x, y)].symbol();
            line.push_str(symbol);
            x += Span::raw(symbol).width().max(1) as u16;
        }
        output.push_str(line.trim_end());
        output.push('\n');
    }
    Ok(output)
}

fn live(sequence: &Sequence) -> Result<(), String> {
    let mut terminal = ratatui::try_init().map_err(|e| e.to_string())?;
    let result = (|| -> std::io::Result<()> {
        let mut offset = 0;
        loop {
            let mut view = Viewport::default();
            terminal.draw(|frame| {
                view = draw(frame, sequence, offset);
            })?;
            offset = view.offset;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    return Ok(());
                }
                offset = scroll(&view, key.code);
            }
        }
    })();
    ratatui::restore();
    result.map_err(|e| e.to_string())
}

const USAGE: &str = "usage: sequence FILE.json [--check | --preview WIDTH HEIGHT [OFFSET]]";

fn run(args: &[String]) -> Result<(), String> {
    let (path, rest) = args.split_first().ok_or(USAGE)?;
    // Validate all CLI arguments before filesystem or terminal effects.
    let dimensions = match rest {
        [] => None,
        [flag] if flag == "--check" => None,
        [flag, width, height, tail @ ..] if flag == "--preview" && tail.len() <= 1 => {
            let number = |s: &str| s.parse::<u16>().map_err(|_| "expected an unsigned integer");
            let width = number(width)?;
            let height = number(height)?;
            if !(1..=240).contains(&width) || !(1..=100).contains(&height) {
                return Err("preview dimensions must be 1..240 columns and 1..100 rows".into());
            }
            Some((
                width,
                height,
                tail.first().map(|s| number(s)).transpose()?.unwrap_or(0),
            ))
        }
        _ => return Err(USAGE.into()),
    };
    let sequence = load(path)?;
    if let Some((width, height, offset)) = dimensions {
        print!("{}", preview(&sequence, width, height, offset)?);
        Ok(())
    } else if !rest.is_empty() {
        println!("sequence OK (shape only; display data only)");
        Ok(())
    } else {
        live(&sequence)
    }
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("sequence-journey.json");

    #[test]
    fn boundary_rejects_invalid_data_not_only_invalid_json() {
        parse(FIXTURE).unwrap();
        for bad in ["{}", "not json", &" ".repeat(MAX_BYTES + 1)] {
            assert!(parse(bad).is_err());
        }
        for field in ["title", "provenance", "scope", "intent"] {
            for bad in [
                "",
                " ",
                "\u{1b}[31m",
                "\u{202e}fake",
                "a\nb",
                "a\tb",
                &"x".repeat(501),
            ] {
                let mut v: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
                v[field] = bad.into();
                assert!(parse(&v.to_string()).is_err(), "{field}: {bad:?}");
            }
        }
        for count in [0, 1, 13] {
            let mut v: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            v["items"] = serde_json::json!(vec![serde_json::json!({"label":"x"}); count]);
            assert!(parse(&v.to_string()).is_err());
        }
        for change in [
            serde_json::json!({"label":"x", "command":"anything"}),
            serde_json::json!({"label":"x", "detail":[""]}),
            serde_json::json!({"label":"\u{2066}hidden"}),
            serde_json::json!({"label":"x", "detail":vec!["x";13]}),
        ] {
            let mut v: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            v["items"][0] = change;
            assert!(parse(&v.to_string()).is_err());
        }
        for (key, value) in [
            ("command", serde_json::json!("anything")),
            ("relationship", serde_json::json!("ranking")),
            ("notes", serde_json::json!(vec!["x"; 13])),
            ("scope", serde_json::json!("旅".repeat(31))),
        ] {
            let mut v: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            v[key] = value;
            assert!(parse(&v.to_string()).is_err());
        }
    }

    #[test]
    fn optional_content_is_not_compulsory_template_filler() {
        let source = r#"{"title":"Test","provenance":"Supplied","scope":"Selected events","intent":"Understand order, not cause","relationship":"chronology","items":[{"label":"Earlier"},{"label":"Later"}]}"#;
        let sequence = parse(source).unwrap();
        let cells = preview(&sequence, 80, 24, 0).unwrap();
        assert!(cells.contains("──── Chronology · not causation"));
        assert!(!cells.contains("Background and sources"));
        assert!(cells.contains("01 ●  Earlier"));
        assert!(cells.contains("02 ●  Later"));
    }

    #[test]
    fn opening_padding_preserves_authored_text_and_body_height() {
        for (source, totals) in [
            (include_str!("sequence-intake.json"), [29, 27]),
            (include_str!("sequence-observatory.json"), [24, 21]),
        ] {
            let sequence = parse(source).unwrap();
            for ((width, height), total) in [(72, 35), (80, 24)].into_iter().zip(totals) {
                let cells = preview(&sequence, width, height, 0).unwrap();
                let rows: Vec<_> = cells.lines().collect();
                assert!(rows[0].is_empty()); // exactly one outer top-padding row
                assert_eq!(rows[1].trim(), sequence.title);
                assert!(rows[2].is_empty());
                assert_eq!(rows[3].trim(), sequence.provenance);
                assert_eq!(rows[4].trim(), sequence.scope);
                assert!(rows[5].is_empty());
                assert!(cells.contains(sequence.relationship.heading()));
                assert!(rows.last().unwrap().contains(&format!("/{total} |")));
            }
            // The rule is measured at item width but drawn at the outer margin.
            // Its fixed text must remain one row at the minimum supported width.
            assert!(Span::raw(sequence.relationship.heading()).width() <= 54);
            let plan = blocks(&sequence);
            let first = plan.iter().find(|b| b.section.is_some()).unwrap();
            let plain = Block {
                section: None,
                ..*first
            };
            assert_eq!(
                paragraph(first).line_count(54),
                paragraph(&plain).line_count(54) + 1
            );
            let end = preview(&sequence, 64, 12, u16::MAX).unwrap();
            assert!(end.contains(&sequence.provenance));
            assert!(end.contains(&sequence.scope));
        }
    }

    #[test]
    fn fixed_journey_keeps_original_wording() {
        let sequence = parse(FIXTURE).unwrap();
        let old: serde_json::Value =
            serde_json::from_str(include_str!("coach-story.json")).unwrap();
        for (item, original) in sequence
            .items
            .iter()
            .zip(old["route"]["steps"].as_array().unwrap())
        {
            assert_eq!(item.label, original.as_str().unwrap());
        }
        assert_eq!(
            sequence.items.len(),
            old["route"]["steps"].as_array().unwrap().len()
        );
        assert_eq!(
            sequence.notes,
            old["support"]["lines"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn wrapping_stays_in_item_column_and_scroll_reidentifies_continuation() {
        let mut sequence = parse(FIXTURE).unwrap();
        sequence.intent = "Follow the supplied order".into();
        sequence.items[0].label = "After approval, submit this deliberately long label without shortening away any of its important conditions or qualifications".into();
        sequence.items[0].detail = vec!["Elaboration continues under its own item even when the available terminal forces another line of prose to wrap here".into()];
        let top = preview(&sequence, 64, 16, 0).unwrap();
        let rows: Vec<_> = top.lines().collect();
        let label_row = rows
            .iter()
            .position(|s| s.starts_with("  01 ●  After approval"))
            .unwrap();
        assert!(rows[label_row + 1].starts_with("     │  "));
        assert!(rows[label_row + 2].starts_with("     │  "));
        let clipped = preview(&sequence, 64, 12, 4).unwrap();
        assert!(
            clipped
                .lines()
                .nth(HEADER_ROWS as usize)
                .unwrap()
                .contains(sequence.relationship.heading())
        );
        assert!(
            clipped
                .lines()
                .nth(HEADER_ROWS as usize + 1)
                .unwrap()
                .starts_with("  01 ●  After approval")
        );
        assert!(!clipped.contains("earlier text above"));
        let mut terminal = Terminal::new(TestBackend::new(64, 16)).unwrap();
        terminal
            .draw(|f| {
                draw(f, &sequence, 0);
            })
            .unwrap();
        assert!(
            terminal.backend().buffer()[(8, label_row as u16)]
                .modifier
                .contains(Modifier::BOLD)
        );
        assert_eq!(
            terminal.backend().buffer()[(5, label_row as u16 + 1)].fg,
            Color::Cyan
        );
    }

    #[test]
    fn section_rhythm_is_distinct_without_separating_labels_from_detail() {
        let sequence = parse(include_str!("sequence-intake.json")).unwrap();
        let plan = blocks(&sequence);
        let sections: Vec<_> = plan
            .iter()
            .enumerate()
            .filter(|(_, b)| b.section.is_some())
            .collect();
        assert_eq!(sections.len(), 2);
        for (index, _) in &sections {
            assert!(plan[index - 1].text.is_empty() && plan[index - 1].item.is_none());
            assert!(plan[index - 2].text.is_empty() && plan[index - 2].item.is_none());
            assert!(!plan[index - 3].text.is_empty());
        }
        for (index, block) in plan.iter().enumerate().filter(|(_, b)| b.label) {
            assert_eq!(plan[index + 1].item, block.item);
            assert!(!plan[index + 1].text.is_empty()); // no inserted label/detail gap
        }
        let top = preview(&sequence, 72, 40, 0).unwrap();
        let rows: Vec<_> = top.lines().collect();
        let label_y = rows.iter().position(|s| s.contains("01 ●")).unwrap() as u16;
        let notes_y = rows
            .iter()
            .position(|s| s.contains("Background and sources"))
            .unwrap() as u16;
        let mut terminal = Terminal::new(TestBackend::new(72, 40)).unwrap();
        terminal
            .draw(|f| {
                draw(f, &sequence, 0);
            })
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(2, label_y - 1)].fg, Color::Cyan);
        assert_eq!(
            rows[label_y as usize - 1],
            format!("  {}", sequence.relationship.heading())
        );
        assert_eq!(buffer[(8, label_y)].fg, Color::Reset);
        assert!(buffer[(8, label_y)].modifier.contains(Modifier::BOLD));
        assert_eq!(buffer[(2, notes_y)].fg, Color::Cyan);
        for number in ["01 ●", "02 ●", "03 ●"] {
            assert!(top.contains(number));
        }
    }

    #[test]
    fn section_landmarks_are_not_stranded_at_the_bottom() {
        let mut deferred = parse(FIXTURE).unwrap();
        deferred.intent = "Intro ".repeat(11); // two rows + two-row gap
        let cells = preview(&deferred, 64, 12, 0).unwrap();
        assert!(!cells.contains(deferred.relationship.heading()));
        assert!(cells.contains("MORE below"));
        assert!(!cells.contains("paragraph continues")); // no paragraph shown yet
        let mut terminal = Terminal::new(TestBackend::new(64, 12)).unwrap();
        let mut view = Viewport::default();
        terminal
            .draw(|f| {
                view = draw(f, &deferred, 0);
            })
            .unwrap();
        assert_eq!(view.page, 4);
        let next = preview(&deferred, 64, 12, scroll(&view, KeyCode::PageDown)).unwrap();
        assert!(next.contains(deferred.relationship.heading()) && next.contains("01 ●"));
        for source in [
            include_str!("sequence-intake.json"),
            include_str!("sequence-observatory.json"),
        ] {
            let sequence = parse(source).unwrap();
            for (width, height) in [(64, 12), (72, 35), (80, 24)] {
                let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
                let mut end = Viewport::default();
                terminal
                    .draw(|f| {
                        end = draw(f, &sequence, u16::MAX);
                    })
                    .unwrap();
                for requested in 0..=end.max_offset + 1 {
                    let cells = preview(&sequence, width, height, requested).unwrap();
                    let last = cells.lines().nth(height as usize - 2).unwrap();
                    assert!(
                        !last.contains(sequence.relationship.heading())
                            && !last.contains("Background and sources")
                    );
                    let mut view = Viewport::default();
                    terminal
                        .draw(|f| {
                            view = draw(f, &sequence, requested);
                        })
                        .unwrap();
                    if view.offset < view.max_offset {
                        assert!(scroll(&view, KeyCode::PageDown) > view.offset);
                    }
                }
            }
        }
    }

    #[test]
    fn every_body_line_remains_reachable_with_persistent_scope() {
        let mut sequence = parse(FIXTURE).unwrap();
        sequence.items[0].detail = (0..12)
            .map(|n| format!("Detail {n}: {}", "long elaboration ".repeat(25)))
            .collect();
        sequence
            .notes
            .push("FINAL unknown: arrival has not been verified.".into());
        for (width, height) in [(64, 12), (72, 35), (80, 24), (240, 100)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            let mut view = Viewport::default();
            terminal
                .draw(|f| {
                    view = draw(f, &sequence, u16::MAX);
                })
                .unwrap();
            let mut seen = String::new();
            for offset in 0..=view.max_offset {
                let cells = preview(&sequence, width, height, offset).unwrap();
                assert!(cells.contains(&sequence.scope));
                assert!(cells.contains(&sequence.provenance));
                assert_eq!(cells.lines().count(), height as usize);
                seen.push_str(&cells);
            }
            for n in 0..12 {
                assert!(seen.contains(&format!("Detail {n}:")));
            }
            assert!(seen.contains("FINAL unknown:"));
            assert!(seen.contains("END |"));
            assert_eq!(seen.contains("MORE below"), view.max_offset > 0);
        }
    }

    #[test]
    fn unicode_and_shell_looking_prose_are_inert_and_not_corrupted() {
        let mut sequence = parse(FIXTURE).unwrap();
        sequence.items[0].label = "旅の案内 e\u{301} $(whoami) {{command}} `id`".into();
        assert!(
            preview(&sequence, 80, 24, 0)
                .unwrap()
                .contains(&sequence.items[0].label)
        );
    }

    #[test]
    fn review_corrections_preserve_raw_trial_and_restore_only_missing_facts() {
        let original = include_str!("../docs/assets/sequence/author/case-c-final.json");
        let mut expected: serde_json::Value = serde_json::from_str(original).unwrap();
        expected["items"][0]["label"] =
            "Take the batch identifier from the intake sheet and copy it onto the batch sleeve"
                .into();
        let actual: serde_json::Value =
            serde_json::from_str(include_str!("sequence-intake.json")).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(
            include_str!("sequence-observatory.json"),
            include_str!("../docs/assets/sequence/author/case-a-final.json")
        );
        for source in [
            include_str!("sequence-intake.json"),
            include_str!("sequence-observatory.json"),
        ] {
            parse(source).unwrap();
        }
        let prose = include_str!("sequence-suppliers.txt");
        for fact in [
            "fictional",
            "Not live or independently verified",
            "40 USD",
            "55 USD",
            "70 USD",
            "6 May 2026",
            "final delivered total is unknown",
        ] {
            assert!(prose.contains(fact));
        }
    }

    #[test]
    fn fitted_negation_is_never_an_unmarked_leading_fragment() {
        // Exact uncoached artifact that exposed the bug, plus corrected demo.
        for source in [
            include_str!("../docs/assets/sequence/author/case-c-final.json"),
            include_str!("sequence-intake.json"),
        ] {
            let sequence = parse(source).unwrap();
            for (width, height) in [(64, 12), (72, 35), (80, 24)] {
                let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
                let mut view = Viewport::default();
                terminal
                    .draw(|f| {
                        view = draw(f, &sequence, u16::MAX);
                    })
                    .unwrap();
                for request in (0..=view.max_offset + 1).chain([u16::MAX]) {
                    let cells = preview(&sequence, width, height, request).unwrap();
                    let first = cells.lines().nth(HEADER_ROWS as usize).unwrap();
                    assert!(
                        !first.contains("actual identifier is supplied"),
                        "{width}x{height} offset {request}: {first}"
                    );
                    assert!(!first.contains("scanning queue"));
                }
                // Both key directions/page paths feed that same legal-position rule.
                for key in [
                    KeyCode::Down,
                    KeyCode::Up,
                    KeyCode::PageDown,
                    KeyCode::PageUp,
                    KeyCode::Home,
                    KeyCode::End,
                ] {
                    for &offset in &view.positions {
                        let mut at = Viewport::default();
                        terminal
                            .draw(|f| {
                                at = draw(f, &sequence, offset);
                            })
                            .unwrap();
                        assert!(at.positions.contains(&scroll(&at, key)));
                    }
                }
                terminal.backend_mut().resize(64, 12);
                terminal.resize(Rect::new(0, 0, 64, 12)).unwrap();
                terminal
                    .draw(|f| {
                        view = draw(f, &sequence, 5);
                    })
                    .unwrap();
                assert!(view.positions.contains(&view.offset));
            }
        }
    }

    #[test]
    fn oversized_final_paragraph_is_explicit_and_pages_skip_no_body_lines() {
        let mut sequence = parse(FIXTURE).unwrap();
        sequence.notes = vec!["Long detail ".repeat(40)];
        for (width, height) in [(64, 12), (72, 12), (80, 24)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            let mut request = 0;
            let mut previous_end = 0;
            loop {
                let mut view = Viewport::default();
                terminal
                    .draw(|f| {
                        view = draw(f, &sequence, request);
                    })
                    .unwrap();
                assert!(view.offset <= previous_end);
                previous_end = view.offset + view.page;
                if view.offset == view.max_offset {
                    break;
                }
                request = scroll(&view, KeyCode::PageDown);
                assert!(request > view.offset);
            }
            let end = preview(&sequence, width, height, u16::MAX).unwrap();
            if height == 12 {
                assert!(end.contains("Partial paragraph: earlier text above; not standalone"));
            }
            assert!(end.contains("END |"));
        }
        // Fit classification is stable at the exact effective-height boundary.
        assert_eq!(positions(&[5], 5), vec![0]);
        assert_eq!(positions(&[6], 5), vec![0, 1, 2]);
        assert_eq!(positions(&[6], 6), vec![0]);
        assert_eq!(positions(&[7], 6), vec![0, 1, 2]);
        assert_eq!(positions(&[6, 6], 6), vec![0, 6]);
        assert_eq!(positions(&[2, 4, 2], 6), vec![0, 2]);
        sequence.intent = "Long intent ".repeat(40);
        let cells = preview(&sequence, 64, 12, 0).unwrap();
        assert!(cells.contains("MORE below: paragraph continues"));
        let cells = preview(&sequence, 64, 12, 1).unwrap();
        assert!(cells.contains("earlier text above"));
    }

    #[test]
    fn actual_page_paths_expose_every_word_in_both_directions() {
        let mut sequence = parse(FIXTURE).unwrap();
        let tokens: Vec<_> = (0..100).map(|n| format!("TOKEN{n:03}")).collect();
        sequence.notes = tokens.chunks(50).map(|chunk| chunk.join(" ")).collect();
        for (width, height) in [(64, 12), (80, 24), (72, 35)] {
            for key in [KeyCode::PageDown, KeyCode::PageUp] {
                let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
                let mut request = if key == KeyCode::PageUp { u16::MAX } else { 0 };
                let mut seen = std::collections::HashSet::new();
                for attempt in 0..1000 {
                    let mut view = Viewport::default();
                    terminal
                        .draw(|f| {
                            view = draw(f, &sequence, request);
                        })
                        .unwrap();
                    // Observe rendered cells along actual key paths, not just plan offsets.
                    let cells = preview(&sequence, width, height, view.offset).unwrap();
                    seen.extend(cells.split_whitespace().map(str::to_owned));
                    let next = scroll(&view, key);
                    if next == view.offset {
                        break;
                    }
                    assert!(attempt < 999, "page navigation failed to terminate");
                    request = next;
                }
                for token in &tokens {
                    assert!(
                        seen.contains(token),
                        "missing {token}: {width}x{height} {key:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn scrolling_resize_and_argument_errors_are_bounded() {
        let sequence = parse(FIXTURE).unwrap();
        let mut terminal = Terminal::new(TestBackend::new(64, 12)).unwrap();
        let mut view = Viewport::default();
        terminal
            .draw(|f| {
                view = draw(f, &sequence, u16::MAX);
            })
            .unwrap();
        assert_eq!(view.offset, view.max_offset);
        assert!(view.max_offset > 0);
        assert_eq!(scroll(&view, KeyCode::Down), view.max_offset);
        assert_eq!(scroll(&view, KeyCode::Home), 0);
        assert_eq!(scroll(&view, KeyCode::End), view.max_offset);
        assert!(scroll(&view, KeyCode::PageUp) < view.offset);
        assert!(view.positions.contains(&scroll(&view, KeyCode::Up)));
        let start = Viewport {
            offset: 0,
            positions: view.positions.clone(),
            ..view
        };
        assert!(scroll(&start, KeyCode::PageDown) > 0);
        assert!(scroll(&start, KeyCode::PageDown) <= start.page);
        assert_eq!(scroll(&start, KeyCode::Up), 0);
        terminal.backend_mut().resize(240, 100);
        terminal.resize(Rect::new(0, 0, 240, 100)).unwrap();
        terminal
            .draw(|f| {
                view = draw(f, &sequence, start.max_offset);
            })
            .unwrap();
        assert_eq!(view.offset, 0);
        assert!(preview(&sequence, 1, 1, 0).is_ok());
        assert!(preview(&sequence, 63, 12, 0).unwrap().contains("Resize"));
        assert!(preview(&sequence, 0, 24, 0).is_err());
        assert!(preview(&sequence, 80, 101, 0).is_err());
        assert!(run(&[]).is_err());
        assert!(
            run(&["missing".into(), "--bogus".into()])
                .unwrap_err()
                .contains("usage")
        );
        assert!(
            run(&[
                "missing".into(),
                "--preview".into(),
                "0".into(),
                "24".into()
            ])
            .unwrap_err()
            .contains("dimensions")
        );
    }
}
