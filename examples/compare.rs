//! Example-only Compare contract. Data in; presentation out. Not a production API.
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

#[path = "support/diagnostic.rs"]
mod diagnostic;

const MAX_BYTES: usize = 65_536;
const HEADER_ROWS: u16 = 6; // outer top padding, title, gap, provenance, scope, gap
const MIN_WIDTH: u16 = 64;
const MIN_HEIGHT: u16 = 12;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Comparison {
    title: String,
    provenance: String,
    scope: String,
    intent: String,
    criteria: Vec<Criterion>,
    alternatives: Vec<Alternative>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Criterion {
    name: String,
    question: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Alternative {
    name: String,
    answers: Vec<Answer>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Answer {
    criterion: String,
    state: State,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    qualification: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum State {
    Known,
    Unknown,
    Unavailable,
}

fn validate_text(text: &str) -> Result<(), String> {
    if text.trim().is_empty() || text.chars().count() > 500 {
        return Err("text must contain 1..500 characters, not only whitespace".into());
    }
    if text.chars().any(|c| {
        c.is_control()
            || matches!(
                c,
                '\u{061c}'
                    | '\u{200e}'
                    | '\u{200f}'
                    | '\u{202a}'..='\u{202e}'
                    | '\u{2066}'..='\u{2069}'
            )
    }) {
        return Err("control characters and bidi controls are not display text".into());
    }
    Ok(())
}

fn validate_cell_text(text: &str, limit: usize, field: &str) -> Result<(), String> {
    validate_text(text)?;
    if Span::raw(text).width() > limit {
        return Err(format!("{field} is limited to {limit} display cells"));
    }
    Ok(())
}

fn parse(source: &str) -> Result<Comparison, String> {
    if source.len() > MAX_BYTES {
        return Err("comparison exceeds 64 KiB".into());
    }
    let comparison: Comparison = serde_json::from_str(source).map_err(|e| e.to_string())?;
    for (text, field) in [
        (&comparison.title, "title"),
        (&comparison.provenance, "provenance"),
        (&comparison.scope, "scope"),
    ] {
        validate_cell_text(text, 60, field)?;
    }
    validate_text(&comparison.intent)?;
    if !(2..=8).contains(&comparison.criteria.len()) {
        return Err("Compare requires 2..8 criteria".into());
    }
    if !(2..=6).contains(&comparison.alternatives.len()) {
        return Err("Compare requires 2..6 alternatives".into());
    }
    for criterion in &comparison.criteria {
        validate_cell_text(&criterion.name, 40, "criterion name")?;
        validate_text(&criterion.question)?;
    }
    for (index, criterion) in comparison.criteria.iter().enumerate() {
        if comparison.criteria[..index]
            .iter()
            .any(|other| other.name == criterion.name)
        {
            return Err("criterion names must be unique".into());
        }
    }
    for (index, alternative) in comparison.alternatives.iter().enumerate() {
        validate_cell_text(&alternative.name, 24, "alternative name")?;
        if comparison.alternatives[..index]
            .iter()
            .any(|other| other.name == alternative.name)
        {
            return Err("alternative names must be unique".into());
        }
        if alternative.answers.len() != comparison.criteria.len() {
            return Err("each alternative needs exactly one answer per criterion".into());
        }
        for (answer_index, answer) in alternative.answers.iter().enumerate() {
            validate_text(&answer.criterion)?;
            if !comparison
                .criteria
                .iter()
                .any(|criterion| criterion.name == answer.criterion)
            {
                return Err("answer refers to an unknown criterion".into());
            }
            if alternative.answers[..answer_index]
                .iter()
                .any(|other| other.criterion == answer.criterion)
            {
                return Err("answers must name each criterion once".into());
            }
            match answer.state {
                State::Known => {
                    let value = answer
                        .value
                        .as_deref()
                        .ok_or("known answers require a value")?;
                    validate_text(value)?;
                }
                State::Unknown | State::Unavailable => {
                    if answer.value.is_some() {
                        return Err("unknown and unavailable answers cannot have a value".into());
                    }
                }
            }
            if let Some(qualification) = &answer.qualification {
                validate_text(qualification)?;
            }
        }
        for criterion in &comparison.criteria {
            if !alternative
                .answers
                .iter()
                .any(|answer| answer.criterion == criterion.name)
            {
                return Err("each alternative needs every named criterion".into());
            }
        }
    }
    Ok(comparison)
}

// The only data input edge. Bounded bytes, not elapsed time for arbitrary devices.
fn load(path: &str) -> Result<Comparison, String> {
    let mut source = String::new();
    File::open(path)
        .map_err(|e| format!("{path}: {e}"))?
        .take(MAX_BYTES as u64 + 1)
        .read_to_string(&mut source)
        .map_err(|e| format!("{path}: {e}"))?;
    parse(&source)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlockKind {
    Intent,
    Landmark,
    Criterion,
    Answer,
    Gap,
}

struct Block<'a> {
    kind: BlockKind,
    group: usize,
    criterion: Option<&'a Criterion>,
    alternative: Option<&'a Alternative>,
    answer: Option<&'a Answer>,
}

fn blocks(comparison: &Comparison) -> Vec<Block<'_>> {
    let mut result = vec![
        Block {
            kind: BlockKind::Intent,
            group: 0,
            criterion: None,
            alternative: None,
            answer: None,
        },
        Block {
            kind: BlockKind::Gap,
            group: 0,
            criterion: None,
            alternative: None,
            answer: None,
        },
        Block {
            kind: BlockKind::Gap,
            group: 0,
            criterion: None,
            alternative: None,
            answer: None,
        },
        Block {
            kind: BlockKind::Landmark,
            group: 0,
            criterion: None,
            alternative: None,
            answer: None,
        },
        Block {
            kind: BlockKind::Gap,
            group: 0,
            criterion: None,
            alternative: None,
            answer: None,
        },
    ];
    for (criterion_index, criterion) in comparison.criteria.iter().enumerate() {
        result.push(Block {
            kind: BlockKind::Criterion,
            group: criterion_index + 1,
            criterion: Some(criterion),
            alternative: None,
            answer: None,
        });
        for alternative in &comparison.alternatives {
            let answer = alternative
                .answers
                .iter()
                .find(|answer| answer.criterion == criterion.name)
                .expect("validated answer correspondence");
            result.push(Block {
                kind: BlockKind::Answer,
                group: criterion_index + 1,
                criterion: Some(criterion),
                alternative: Some(alternative),
                answer: Some(answer),
            });
        }
        if criterion_index + 1 < comparison.criteria.len() {
            result.push(Block {
                kind: BlockKind::Gap,
                group: criterion_index + 1,
                criterion: None,
                alternative: None,
                answer: None,
            });
        }
    }
    result
}

fn answer_lines(answer: &Answer) -> Vec<Line<'static>> {
    let state = match answer.state {
        State::Known => answer.value.as_deref().expect("validated value").to_owned(),
        State::Unknown => "UNKNOWN · not supplied".to_owned(),
        State::Unavailable => "UNAVAILABLE · not available".to_owned(),
    };
    let mut lines = vec![Line::raw(state)];
    if let Some(qualification) = &answer.qualification {
        lines.push(Line::raw(qualification.clone()));
    }
    lines
}

fn paragraph(block: &Block<'_>) -> Paragraph<'static> {
    match block.kind {
        BlockKind::Intent => unreachable!("intent uses paragraph_for"),
        BlockKind::Landmark => Paragraph::new(Line::styled(
            "──── Compare · same questions · no ranking",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        BlockKind::Criterion => {
            let criterion = block.criterion.expect("criterion block");
            Paragraph::new(vec![
                Line::styled(
                    format!("──── {}", criterion.name),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Line::raw(criterion.question.clone()),
            ])
        }
        BlockKind::Answer => Paragraph::new(answer_lines(block.answer.expect("answer block"))),
        BlockKind::Gap => Paragraph::new(""),
    }
    .wrap(Wrap { trim: false })
}

fn paragraph_for<'a>(block: &Block<'a>, intent: &'a str) -> Paragraph<'static> {
    if block.kind == BlockKind::Intent {
        Paragraph::new(intent.to_owned()).wrap(Wrap { trim: false })
    } else {
        paragraph(block)
    }
}

fn positions(heights: &[usize], groups: &[usize], page: u16) -> Vec<u16> {
    let total: usize = heights.iter().sum();
    let mut result = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < heights.len() {
        let group = groups[index];
        let end = groups[index..]
            .iter()
            .position(|&current| current != group)
            .map(|length| index + length)
            .unwrap_or(heights.len());
        let group_height: usize = heights[index..end].iter().sum();
        let count = if group_height > page as usize {
            group_height
        } else {
            1
        };
        for delta in 0..count {
            let offset = start + delta;
            result.push(offset as u16);
            let available = page as usize - usize::from(delta > 0);
            if total - offset <= available {
                return result;
            }
        }
        start += group_height;
        index = end;
    }
    result
}

#[derive(Debug, Default, PartialEq)]
struct Viewport {
    offset: u16,
    max_offset: u16,
    page: u16,
    positions: Vec<u16>,
}

fn draw(frame: &mut Frame, comparison: &Comparison, requested: u16) -> Viewport {
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        frame.render_widget(
            Paragraph::new("Compare needs 64 columns x 12 rows. Resize; q quits."),
            area,
        );
        return Viewport::default();
    }
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(HEADER_ROWS),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(area);
    let inset = |rect: Rect| Rect::new(rect.x + 2, rect.y, rect.width - 4, rect.height);
    let mut body = inset(body);
    frame.render_widget(
        Paragraph::new(vec![
            Line::default(),
            Line::styled(
                &comparison.title,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Line::default(),
            Line::raw(&comparison.provenance),
            Line::raw(&comparison.scope),
        ]),
        inset(header),
    );

    let blocks = blocks(comparison);
    let label_width = comparison
        .alternatives
        .iter()
        .map(|alternative| Span::raw(&alternative.name).width())
        .max()
        .unwrap_or(0) as u16
        + 4;
    let value_width = body.width.saturating_sub(label_width).max(1);
    let heights: Vec<usize> = blocks
        .iter()
        .map(|block| {
            let width = if block.kind == BlockKind::Answer {
                value_width
            } else {
                body.width
            };
            paragraph_for(block, &comparison.intent)
                .line_count(width)
                .max(1)
        })
        .collect();
    let total: usize = heights.iter().sum();
    let groups: Vec<usize> = blocks.iter().map(|block| block.group).collect();
    let positions = positions(&heights, &groups, body.height);
    let max_offset = positions.last().copied().unwrap_or(0);
    let offset = positions
        .iter()
        .rev()
        .find(|&&position| position <= requested)
        .copied()
        .unwrap_or(0);
    // A position inside a criterion group lacks its heading. Say so even when
    // the position happens to land exactly at an answer block boundary.
    let mut boundary = 0;
    let mut group_index = 0;
    let earlier = heights.iter().enumerate().any(|(index, &height)| {
        if index > 0 && groups[index] != groups[index - 1] {
            group_index = boundary;
        }
        let group_end = if index + 1 == heights.len() || groups[index + 1] != groups[index] {
            boundary + height
        } else {
            usize::MAX
        };
        boundary += height;
        group_end != usize::MAX && (offset as usize) > group_index && (offset as usize) < group_end
    });
    if earlier {
        let mut start = 0;
        let partial_group = blocks
            .iter()
            .zip(heights.iter())
            .find_map(|(block, height)| {
                let end = start + *height;
                let found = (offset as usize) >= start && (offset as usize) < end;
                start = end;
                found.then_some(block.group)
            });
        let criterion = partial_group.and_then(|group| {
            blocks
                .iter()
                .find(|block| block.group == group && block.kind == BlockKind::Criterion)
                .and_then(|block| block.criterion.map(|criterion| criterion.name.as_str()))
        });
        let warning = criterion
            .map(|name| format!("… {name}: not standalone"))
            .unwrap_or_else(|| {
                "… Partial answer/section: earlier text above; not standalone".into()
            });
        frame.render_widget(
            Paragraph::new(warning),
            Rect::new(body.x, body.y, body.width, 1),
        );
        body.y += 1;
        body.height -= 1;
    }
    let mut start = 0;
    let mut continues = false;
    let mut page = body.height;
    let accent = Style::default().fg(Color::Cyan);
    for (block, height) in blocks.iter().zip(heights) {
        let end = start + height;
        let visible_start = start.max(offset as usize);
        let visible_end = end.min(offset as usize + body.height as usize);
        if visible_start < visible_end {
            let skipped = visible_start - start;
            let y = body.y + (visible_start - offset as usize) as u16;
            let shown = (visible_end - visible_start) as u16;
            if block.kind == BlockKind::Criterion && skipped == 0 && shown == 1 {
                page = (visible_start - offset as usize) as u16;
                break;
            }
            continues |= visible_end < end;
            let width = if block.kind == BlockKind::Answer {
                value_width
            } else {
                body.width
            };
            let paragraph = paragraph_for(block, &comparison.intent);
            frame.render_widget(
                paragraph.scroll((skipped as u16, 0)),
                Rect::new(
                    if block.kind == BlockKind::Answer {
                        body.x + label_width
                    } else {
                        body.x
                    },
                    y,
                    width,
                    shown,
                ),
            );
            if block.kind == BlockKind::Answer {
                let alternative = block.alternative.expect("answer alternative");
                let label = if skipped == 0 {
                    alternative.name.clone()
                } else {
                    format!("… {}", alternative.name)
                };
                frame.render_widget(
                    Paragraph::new(label).style(accent),
                    Rect::new(body.x, y, label_width, 1),
                );
                for row in 1..shown {
                    frame.render_widget(
                        Paragraph::new("│").style(accent),
                        Rect::new(body.x, y + row, label_width, 1),
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
            "MORE below: answer/section continues | line {}/{} | j/k | q",
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
            .find(|&&position| position > view.offset)
            .copied()
            .unwrap_or(view.offset)
    };
    let previous = || {
        view.positions
            .iter()
            .rev()
            .find(|&&position| position < view.offset)
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
            .find(|&&position| position <= view.offset.saturating_add(view.page))
            .copied()
            .unwrap_or(view.offset),
        KeyCode::PageUp => view
            .positions
            .iter()
            .find(|&&position| {
                position >= view.offset.saturating_sub(view.page.saturating_sub(1))
                    && position < view.offset
            })
            .copied()
            .unwrap_or_else(previous),
        KeyCode::Home => 0,
        KeyCode::End => view.max_offset,
        _ => view.offset,
    }
}

fn preview(
    comparison: &Comparison,
    width: u16,
    height: u16,
    offset: u16,
) -> Result<String, String> {
    if !(1..=240).contains(&width) || !(1..=100).contains(&height) {
        return Err("preview dimensions must be 1..240 columns and 1..100 rows".into());
    }
    let mut terminal = Terminal::new(TestBackend::new(width, height)).map_err(|e| e.to_string())?;
    terminal
        .draw(|frame| {
            draw(frame, comparison, offset);
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

fn live(comparison: &Comparison) -> Result<(), String> {
    let mut terminal = ratatui::try_init().map_err(|e| e.to_string())?;
    let result = (|| -> std::io::Result<()> {
        let mut offset = 0;
        loop {
            let mut view = Viewport::default();
            terminal.draw(|frame| {
                view = draw(frame, comparison, offset);
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
    diagnostic::after_restore(result, ratatui::try_restore())
}

const USAGE: &str = "usage: compare FILE.json [--check | --preview WIDTH HEIGHT [OFFSET]]";

fn run(args: &[String]) -> Result<(), String> {
    let (path, rest) = args.split_first().ok_or(USAGE)?;
    let dimensions = match rest {
        [] => None,
        [flag] if flag == "--check" => None,
        [flag, width, height, tail @ ..] if flag == "--preview" && tail.len() <= 1 => {
            let number = |value: &str| {
                value
                    .parse::<u16>()
                    .map_err(|_| "expected an unsigned integer")
            };
            let width = number(width)?;
            let height = number(height)?;
            if !(1..=240).contains(&width) || !(1..=100).contains(&height) {
                return Err("preview dimensions must be 1..240 columns and 1..100 rows".into());
            }
            Some((
                width,
                height,
                tail.first()
                    .map(|value| number(value))
                    .transpose()?
                    .unwrap_or(0),
            ))
        }
        _ => return Err(USAGE.into()),
    };
    let comparison = load(path)?;
    if let Some((width, height, offset)) = dimensions {
        print!("{}", preview(&comparison, width, height, offset)?);
        Ok(())
    } else if !rest.is_empty() {
        println!("comparison OK (shape only; display data only)");
        Ok(())
    } else {
        live(&comparison)
    }
}

fn main() -> ExitCode {
    diagnostic::finish(
        run(&env::args().skip(1).collect::<Vec<_>>()),
        &mut std::io::stderr().lock(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("compare-suppliers.json");
    const HOSTILE: &str = "\u{1b}[2JFIELD\u{9b}0m\u{202e}\n";
    const ESCAPED_HOSTILE: &str = r"\u{1b}[2JFIELD\u{9b}0m\u{202e}\n";

    fn assert_safe_rejection(value: serde_json::Value) {
        let error = parse(&value.to_string()).unwrap_err();
        assert!(error.contains(HOSTILE), "{error:?}");
        let expected = format!("{}\n", error.replace(HOSTILE, ESCAPED_HOSTILE));
        let mut stderr = Vec::new();
        assert_eq!(
            diagnostic::finish(Err(error), &mut stderr),
            ExitCode::FAILURE
        );
        assert_eq!(stderr, expected.as_bytes());
    }

    #[test]
    fn rejected_unknown_fields_have_safe_diagnostics_at_every_object_level() {
        for pointer in [
            "",
            "/criteria/0",
            "/alternatives/0",
            "/alternatives/0/answers/0",
        ] {
            let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            value
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert(HOSTILE.into(), true.into());
            assert_safe_rejection(value);
        }
    }

    #[test]
    fn rejected_state_variants_have_safe_diagnostics() {
        let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        value["alternatives"][0]["answers"][0]["state"] = HOSTILE.into();
        assert_safe_rejection(value);
    }

    #[test]
    fn boundary_rejects_unknown_fields_states_and_bad_correspondence() {
        let parsed = parse(FIXTURE).unwrap();
        assert_eq!(parsed.alternatives.len(), 3);
        for source in ["{}", "not json", &" ".repeat(MAX_BYTES + 1)] {
            assert!(parse(source).is_err());
        }
        let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        value["command"] = "id".into();
        assert!(parse(&value.to_string()).is_err());
        value = serde_json::from_str(FIXTURE).unwrap();
        value["criteria"][0]["extra"] = true.into();
        assert!(parse(&value.to_string()).is_err());
        value = serde_json::from_str(FIXTURE).unwrap();
        value["alternatives"][0]["answers"][0]["state"] = "maybe".into();
        assert!(parse(&value.to_string()).is_err());
        value = serde_json::from_str(FIXTURE).unwrap();
        value["alternatives"][0]["answers"]
            .as_array_mut()
            .unwrap()
            .pop();
        assert!(parse(&value.to_string()).is_err());
        value = serde_json::from_str(FIXTURE).unwrap();
        value["alternatives"][0]["answers"][0]["criterion"] = "other".into();
        assert!(parse(&value.to_string()).is_err());
        value = serde_json::from_str(FIXTURE).unwrap();
        value["alternatives"][2]["answers"][0]["state"] = "unknown".into();
        value["alternatives"][2]["answers"][0]["value"] = "0".into();
        assert!(parse(&value.to_string()).is_err());
        value = serde_json::from_str(FIXTURE).unwrap();
        value["alternatives"][0]["name"] = "\u{202e}hidden".into();
        assert!(parse(&value.to_string()).is_err());
    }

    #[test]
    fn supplier_baseline_facts_and_provenance_are_retained() {
        let baseline = include_str!("sequence-suppliers.txt");
        for fact in [
            "fictional",
            "Not live or independently verified.",
            "100 handouts",
            "6 May 2026",
            "Alder — 40 USD; pickup Friday; recycled paper unavailable.",
            "Birch — 55 USD; pickup Thursday; recycled paper available.",
            "Cedar — 70 USD; delivery Thursday; recycled paper available.",
            "delivery charges are not supplied",
            "final delivered total is unknown",
            "No supplier is selected yet",
            "not a ranked journey",
        ] {
            assert!(baseline.contains(fact), "missing baseline fact: {fact}");
        }
        let comparison = parse(FIXTURE).unwrap();
        let cells = preview(&comparison, 120, 40, 0).unwrap();
        for fact in [
            "Fictional quotes / not live / not independently verified",
            "100 handouts",
            "6 May 2026",
            "Alder",
            "40 USD",
            "Birch",
            "55 USD",
            "Cedar",
            "70 USD",
            "UNKNOWN",
            "UNAVAILABLE",
            "delivery charge is not supplied",
            "final delivered total is unknown",
        ] {
            assert!(
                cells.to_lowercase().contains(&fact.to_lowercase()),
                "missing rendered fact: {fact}"
            );
        }
        assert!(cells.contains("no ranking"));
        assert!(!cells.contains("BEST"));
    }

    #[test]
    fn criterion_grouping_and_narrow_identity_are_preserved() {
        let comparison = parse(FIXTURE).unwrap();
        let wide = preview(&comparison, 120, 40, 0).unwrap();
        for name in ["Quoted amount", "Fulfillment terms", "Recycled paper"] {
            assert!(wide.contains(name));
        }
        let narrow = preview(&comparison, 72, 35, 0).unwrap();
        for name in ["Alder", "Birch", "Cedar", "40 USD", "55 USD", "70 USD"] {
            assert!(narrow.contains(name));
        }
        let end = preview(&comparison, 72, 35, u16::MAX).unwrap();
        let narrow_end = preview(&comparison, 64, 12, u16::MAX).unwrap();
        assert!(narrow_end.contains("Recycled paper: not standalone"));
        let medium_end = preview(&comparison, 80, 24, u16::MAX).unwrap();
        assert!(
            medium_end
                .lines()
                .nth(HEADER_ROWS as usize)
                .unwrap()
                .contains("Fulfillment terms")
        );
        let end_lower = end.to_lowercase();
        for word in ["final", "delivered", "total", "unknown"] {
            assert!(
                end_lower
                    .split_whitespace()
                    .any(|token| token.contains(word))
            );
        }
        assert!(end.contains("UNAVAILABLE"));
        assert!(narrow.contains("MORE below") || narrow.contains("END"));
        assert!(!narrow.contains("01 ●"));
    }

    #[test]
    fn all_body_words_are_reachable_through_page_paths() {
        let mut comparison = parse(FIXTURE).unwrap();
        comparison.alternatives[0].answers[0].qualification = Some(
            "This qualification deliberately has enough words to wrap while remaining attached to Alder's quoted amount".into(),
        );
        for (width, height) in [(64, 12), (72, 35), (80, 24)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            let mut end = Viewport::default();
            terminal
                .draw(|frame| {
                    end = draw(frame, &comparison, u16::MAX);
                })
                .unwrap();
            let mut seen = String::new();
            let mut request = 0;
            for _ in 0..1000 {
                let mut view = Viewport::default();
                terminal
                    .draw(|frame| {
                        view = draw(frame, &comparison, request);
                    })
                    .unwrap();
                seen.push_str(&preview(&comparison, width, height, view.offset).unwrap());
                let next = scroll(&view, KeyCode::PageDown);
                if next == view.offset {
                    break;
                }
                request = next;
            }
            for word in ["final", "delivered", "total", "is", "unknown"] {
                assert!(
                    seen.to_lowercase()
                        .split_whitespace()
                        .any(|token| token.contains(word)),
                    "missing {word}"
                );
            }
            assert!(seen.contains("END"));
            if end.max_offset == 0 {
                assert!(seen.contains("END"));
                continue;
            }
            let mut view = end;
            while view.offset > 0 {
                let previous = scroll(&view, KeyCode::PageUp);
                assert!(previous < view.offset);
                terminal
                    .draw(|frame| {
                        view = draw(frame, &comparison, previous);
                    })
                    .unwrap();
            }
            assert_eq!(view.offset, 0);
        }
    }

    #[test]
    fn inert_text_and_explicit_states_are_not_commands() {
        let mut comparison = parse(FIXTURE).unwrap();
        comparison.intent = "Judge $(whoami) {{command}} `id` as supplied prose".into();
        let cells = preview(&comparison, 80, 24, 0).unwrap();
        assert!(cells.contains("$(whoami)"));
        assert!(cells.contains("{{command}}"));
        let access = parse(include_str!("compare-access.json")).unwrap();
        assert_eq!(access.alternatives.len(), 2);
        let access_cells = preview(&access, 80, 24, u16::MAX).unwrap();
        assert!(access_cells.contains("UNKNOWN"));
        assert!(access_cells.contains("Replay access window is not specified."));
    }

    #[test]
    fn unsupported_dimensions_report_limits_without_terminal_effects() {
        let comparison = parse(FIXTURE).unwrap();
        assert!(preview(&comparison, 0, 24, 0).is_err());
        assert!(preview(&comparison, 241, 24, 0).is_err());
        assert!(preview(&comparison, 80, 101, 0).is_err());
        assert!(preview(&comparison, 63, 12, 0).unwrap().contains("Resize"));
        assert!(run(&[]).is_err());
        assert!(
            run(&["missing".into(), "--bogus".into()])
                .unwrap_err()
                .contains("usage")
        );
    }
}
