//! Disposable data-only story experiment. Not a production HUD document API.
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame, Terminal,
    backend::TestBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
};
use serde::Deserialize;
use std::{env, fs::File, io::Read, process::ExitCode};

#[path = "support/diagnostic.rs"]
mod diagnostic;

const MAX_BYTES: usize = 65_536;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Story {
    title: String,
    provenance: String,
    lead: Lead,
    route: Route,
    support: Support,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Lead {
    label: String,
    headline: String,
    lines: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Route {
    label: String,
    steps: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Support {
    label: String,
    lines: Vec<String>,
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

fn parse(source: &str) -> Result<Story, String> {
    if source.len() > MAX_BYTES {
        return Err("story exceeds 64 KiB".into());
    }
    let story: Story = serde_json::from_str(source).map_err(|e| e.to_string())?;
    for text in [
        &story.title,
        &story.provenance,
        &story.lead.label,
        &story.lead.headline,
        &story.route.label,
        &story.support.label,
    ] {
        validate_text(text)?;
    }
    // Persistent header and provenance stay readable at the supported minimum.
    if Span::raw(&story.title).width() > 60 || Span::raw(&story.provenance).width() > 60 {
        return Err("title and provenance are limited to 60 display cells each".into());
    }
    for collection in [&story.lead.lines, &story.route.steps, &story.support.lines] {
        if !(1..=12).contains(&collection.len()) {
            return Err("lead lines, route steps, and support lines require 1..12 entries".into());
        }
        for text in collection {
            validate_text(text)?;
        }
    }
    Ok(story)
}

// The only input edge. Bounded read even when a file grows during the read.
fn load(path: &str) -> Result<Story, String> {
    let mut source = String::new();
    File::open(path)
        .map_err(|e| format!("{path}: {e}"))?
        .take(MAX_BYTES as u64 + 1)
        .read_to_string(&mut source)
        .map_err(|e| format!("{path}: {e}"))?;
    parse(&source)
}

fn body(story: &Story, plain: bool) -> Text<'_> {
    let accent = if plain {
        Style::default()
    } else {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };
    let headline = if plain {
        Style::default()
    } else {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };
    let mut lines = vec![
        Line::styled(story.lead.label.as_str(), accent),
        Line::styled(story.lead.headline.as_str(), headline),
    ];
    lines.extend(story.lead.lines.iter().map(|s| Line::raw(s.as_str())));
    lines.push(Line::default());
    lines.push(Line::styled(story.route.label.as_str(), accent));
    for (i, step) in story.route.steps.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:02} {} ", i + 1, if plain { "." } else { "●" }),
                accent,
            ),
            Span::raw(step),
        ]));
        if !plain && i + 1 < story.route.steps.len() {
            lines.push(Line::styled("   │", accent));
        }
    }
    lines.push(Line::default());
    lines.push(Line::styled(
        format!(
            "{}{}",
            if plain { "" } else { "──── " },
            story.support.label
        ),
        accent,
    ));
    lines.extend(story.support.lines.iter().map(|s| Line::raw(s.as_str())));
    Text::from(lines)
}

#[derive(Debug, Default, PartialEq)]
struct Viewport {
    offset: u16,
    max_offset: u16,
    page: u16,
}

fn draw(frame: &mut Frame, story: &Story, plain: bool, requested: u16) -> Viewport {
    let area = frame.area();
    if area.width < 64 || area.height < 12 {
        frame.render_widget(
            Paragraph::new("Story probe needs 64 columns x 12 rows. Resize; q quits.")
                .wrap(Wrap { trim: false }),
            area,
        );
        return Viewport::default();
    }
    let [header, content, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(area);
    let inset = |rect: Rect| Rect::new(rect.x + 2, rect.y, rect.width - 4, rect.height);
    let content = inset(content);
    let header_style = if plain {
        Style::default()
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    };
    frame.render_widget(
        Paragraph::new(vec![
            Line::styled(story.title.as_str(), header_style),
            Line::raw(story.provenance.as_str()),
        ])
        .wrap(Wrap { trim: false }),
        inset(header),
    );
    let paragraph = Paragraph::new(body(story, plain)).wrap(Wrap { trim: false });
    let total = paragraph.line_count(content.width);
    let max_offset = total.saturating_sub(content.height as usize) as u16;
    let offset = requested.min(max_offset);
    frame.render_widget(paragraph.scroll((offset, 0)), content);
    let more = if offset < max_offset {
        "MORE below"
    } else {
        "END"
    };
    frame.render_widget(
        Paragraph::new(format!(
            "{more} | line {}/{} | j/k PgDn Home/End | q quit",
            offset + 1,
            total
        )),
        inset(footer),
    );
    Viewport {
        offset,
        max_offset,
        page: content.height,
    }
}

fn scroll(view: &Viewport, key: KeyCode) -> u16 {
    match key {
        KeyCode::Down | KeyCode::Char('j') => view.offset.saturating_add(1),
        KeyCode::Up | KeyCode::Char('k') => view.offset.saturating_sub(1),
        KeyCode::PageDown => view.offset.saturating_add(view.page),
        KeyCode::PageUp => view.offset.saturating_sub(view.page),
        KeyCode::Home => 0,
        KeyCode::End => view.max_offset,
        _ => view.offset,
    }
    .min(view.max_offset)
}

fn preview(
    story: &Story,
    plain: bool,
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
            draw(frame, story, plain, offset);
        })
        .map_err(|e| e.to_string())?;
    let buffer = terminal.backend().buffer();
    let mut output = String::new();
    for y in 0..height {
        let mut line = String::new();
        let mut x = 0;
        while x < width {
            // Wide grapheme continuation cells are not extra printable spaces.
            let symbol = buffer[(x, y)].symbol();
            line.push_str(symbol);
            x += Span::raw(symbol).width().max(1) as u16;
        }
        output.push_str(line.trim_end());
        output.push('\n');
    }
    Ok(output)
}

fn live(story: &Story, plain: bool) -> Result<(), String> {
    // try_init installs ratatui's restoration panic hook. Restore on ordinary errors too.
    let mut terminal = ratatui::try_init().map_err(|e| e.to_string())?;
    let result = (|| -> std::io::Result<()> {
        let mut offset = 0;
        loop {
            let mut view = Viewport::default();
            terminal.draw(|frame| {
                view = draw(frame, story, plain, offset);
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

const USAGE: &str =
    "usage: expressive_story FILE.json [--plain] [--check | --preview WIDTH HEIGHT [OFFSET]]";

fn run(args: &[String]) -> Result<(), String> {
    let (path, mut rest) = args.split_first().ok_or(USAGE)?;
    let plain = rest.first().is_some_and(|s| s == "--plain");
    if plain {
        rest = &rest[1..];
    }
    // Validate arguments before any terminal initialization or file access.
    let dimensions = match rest {
        [] => None,
        [flag] if flag == "--check" => None,
        [flag, width, height, tail @ ..] if flag == "--preview" && tail.len() <= 1 => {
            let number = |s: &str| s.parse::<u16>().map_err(|_| "expected an unsigned integer");
            Some((
                number(width)?,
                number(height)?,
                tail.first().map(|s| number(s)).transpose()?.unwrap_or(0),
            ))
        }
        _ => return Err(USAGE.into()),
    };
    let story = load(path)?;
    if let Some((width, height, offset)) = dimensions {
        print!("{}", preview(&story, plain, width, height, offset)?);
        Ok(())
    } else if !rest.is_empty() {
        println!("story OK (display data only)");
        Ok(())
    } else {
        live(&story, plain)
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
    const FIXTURE: &str = include_str!("coach-story.json");

    #[test]
    fn rejected_unknown_fields_have_safe_diagnostics_at_every_object_level() {
        const HOSTILE: &str = "\u{1b}[2JFIELD\u{9b}0m\u{202e}\n";
        const ESCAPED_HOSTILE: &str = r"\u{1b}[2JFIELD\u{9b}0m\u{202e}\n";
        for pointer in ["", "/lead", "/route", "/support"] {
            let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            value
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert(HOSTILE.into(), true.into());
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
    }

    #[test]
    fn validates_at_the_boundary_and_rejects_executable_fields() {
        parse(FIXTURE).unwrap();
        for replacement in ["", "\u{1b}[31m", "\u{202e}fake", &"x".repeat(501)] {
            let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            value["lead"]["headline"] = replacement.into();
            assert!(parse(&value.to_string()).is_err());
        }
        for target in ["lead", "route", "support"] {
            let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
            value[target]["command"] = "touch /tmp/not-executed".into();
            assert!(parse(&value.to_string()).is_err());
        }
        let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        value["command"] = "anything".into();
        assert!(parse(&value.to_string()).is_err());
        assert!(parse(&" ".repeat(MAX_BYTES + 1)).is_err());
        assert!(parse("{}").is_err());
        assert!(parse("not json").is_err());
        let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        value["route"]["steps"] = serde_json::json!([]);
        assert!(parse(&value.to_string()).is_err());
        value["route"]["steps"] = serde_json::json!(vec!["step"; 13]);
        assert!(parse(&value.to_string()).is_err());
    }

    #[test]
    fn candidate_preserves_baseline_wording() {
        let story = parse(FIXTURE).unwrap();
        let words = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
        let lead = [
            &[story.lead.label, story.lead.headline][..],
            &story.lead.lines,
        ]
        .concat()
        .join(" ");
        let route = story
            .route
            .steps
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}. {s}", i + 1))
            .collect::<Vec<_>>()
            .join(" ");
        for (actual, original) in [
            (lead, include_str!("coach-baseline/coach-lead.txt")),
            (route, include_str!("coach-baseline/coach-route.txt")),
            (
                story.support.lines.join(" "),
                include_str!("coach-baseline/coach-facts.txt"),
            ),
        ] {
            assert_eq!(words(&actual), words(original));
        }
    }

    #[test]
    fn reviewed_author_examples_keep_facts_and_limits_on_the_overview() {
        for source in [
            include_str!("coach-departure.json"),
            include_str!("coach-arrival.json"),
        ] {
            let story = parse(source).unwrap();
            for (width, height) in [(80, 24), (93, 35), (71, 35)] {
                let cells = preview(&story, false, width, height, 0).unwrap();
                let words = cells.split_whitespace().collect::<Vec<_>>().join(" ");
                for fact in [
                    "09:05",
                    "bay C",
                    "10:15",
                    "east gate",
                    "14 USD",
                    "zero transfers",
                    "No additional walk",
                    "Times are local",
                    "not live",
                    "Not supplied:",
                    "ticket validity",
                    "boarding deadline",
                    "service number",
                    "END |",
                ] {
                    assert!(words.contains(fact), "missing {fact} at {width}x{height}");
                }
                assert!(words.contains("one day") || words.contains("one fictional day only"));
            }
        }
        let frozen: Story = parse(include_str!(
            "../docs/assets/expressive-story/author-round-two/story.json"
        ))
        .unwrap();
        let reviewed = parse(include_str!("coach-arrival.json")).unwrap();
        assert_eq!(reviewed.lead.headline, frozen.lead.headline);
        assert_eq!(reviewed.route.steps, frozen.route.steps);
        assert_eq!(reviewed.support.lines, frozen.support.lines);
        assert_eq!(reviewed.lead.lines[0], frozen.lead.lines[0]);
        assert_eq!(
            reviewed.lead.lines[1],
            "Times are local; one fictional day only; not live."
        );
    }

    #[test]
    fn unicode_preview_has_no_spurious_wide_character_spaces() {
        let mut story = parse(FIXTURE).unwrap();
        story.lead.headline = "旅の案内 e\u{301}".into();
        assert!(
            preview(&story, false, 80, 24, 0)
                .unwrap()
                .contains(&story.lead.headline)
        );
        let mut value: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
        value["provenance"] = "旅".repeat(31).into();
        assert!(parse(&value.to_string()).is_err());
    }

    #[test]
    fn shell_syntax_is_literal_data() {
        let mut story = parse(FIXTURE).unwrap();
        story.lead.headline = "$(touch /tmp/not-executed) `whoami` {{command}}".into();
        let cells = preview(&story, false, 93, 35, 0).unwrap();
        assert!(cells.contains(&story.lead.headline));
    }

    #[test]
    fn journey_is_readable_and_overflow_is_explicit() {
        let story = parse(FIXTURE).unwrap();
        for (width, height) in [(93, 35), (80, 24), (120, 40)] {
            for plain in [false, true] {
                let top = preview(&story, plain, width, height, 0).unwrap();
                let bottom = preview(&story, plain, width, height, u16::MAX).unwrap();
                for fact in [
                    "09:05",
                    "Cedar Quay, bay C",
                    "10:15",
                    "east gate",
                    "$14",
                    "0 transfers",
                    "no additional walk",
                    "Fictional",
                ] {
                    assert!(top.contains(fact), "missing {fact} at {width}x{height}");
                }
                let bottom_words = bottom.split_whitespace().collect::<Vec<_>>().join(" ");
                assert!(bottom_words.contains("Nothing here verifies current service."));
                assert!(bottom.contains("END |"));
                let top_words = top.split_whitespace().collect::<Vec<_>>().join(" ");
                if !top_words.contains("Nothing here verifies current service.") {
                    assert!(top.contains("MORE below"));
                }
                assert_eq!(top.lines().count(), height as usize);
            }
        }
    }

    #[test]
    fn lead_has_real_emphasis_not_only_pattern_names() {
        let story = parse(FIXTURE).unwrap();
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|f| {
                draw(f, &story, false, 0);
            })
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(2, 4)].bg, Color::Cyan);
        assert!(buffer[(2, 4)].modifier.contains(Modifier::BOLD));
        terminal
            .draw(|f| {
                draw(f, &story, true, 0);
            })
            .unwrap();
        assert_eq!(terminal.backend().buffer()[(2, 4)].bg, Color::Reset);
    }

    #[test]
    fn scrolling_and_resize_clamp_without_losing_the_end() {
        let story = parse(FIXTURE).unwrap();
        let mut terminal = Terminal::new(TestBackend::new(64, 12)).unwrap();
        let mut view = Viewport::default();
        terminal
            .draw(|f| {
                view = draw(f, &story, false, u16::MAX);
            })
            .unwrap();
        assert_eq!(view.offset, view.max_offset);
        assert!(view.max_offset > 0);
        assert_eq!(scroll(&view, KeyCode::Down), view.max_offset);
        assert_eq!(scroll(&view, KeyCode::Home), 0);
        assert_eq!(
            scroll(&view, KeyCode::PageUp),
            view.offset.saturating_sub(view.page)
        );
        assert!(preview(&story, false, 1, 1, 0).is_ok());
        assert!(preview(&story, false, 0, 24, 0).is_err());
        assert!(preview(&story, false, 80, 101, 0).is_err());
        assert!(run(&[]).is_err());
        assert!(run(&["missing.json".into(), "--bogus".into()]).is_err());
    }
}
