mod core;
mod cursor_vec;

use crate::core::events::{Event, Events};
use anyhow::anyhow;
use anyhow::Result;
use cursor_vec::CursorVec;
use std::io::{self, Stdout};
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

pub type Term = Terminal<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>;

#[derive(Debug)]
pub struct List {
    pub title: String,
    pub steps: Vec<Step>,
}

impl List {
    pub fn new(steps: Vec<String>, title: String) -> Self {
        let steps = steps.into_iter().map(Step::simple).collect();
        Self { title, steps }
    }
}

#[derive(Debug)]
pub struct Step {
    pub title: String,
    pub body: Option<String>,
}

impl Step {
    pub fn simple(s: String) -> Self {
        Self {
            title: s,
            body: None,
        }
    }
}

fn spawn_terminal() -> Result<Term> {
    let stdout = match io::stdout().into_raw_mode() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("Got error when converting stdout: {}", err);
            return Err(err.into());
        }
    };
    // Clear terminal before drawing for the first time, and restore buffer
    // after exiting.
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    match Terminal::new(backend) {
        Ok(t) => Ok(t),
        Err(err) => {
            eprintln!("Got error when spawning term: {}", err);
            Err(err.into())
        }
    }
}

fn make_paragraph(text: String, title: String) -> Paragraph<'static> {
    Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(Style::default())
        .alignment(Alignment::Center)
}

pub fn step_list(list: List) -> Result<()> {
    let mut term = spawn_terminal()?;
    let input = Events::new();
    let title = &list.title;
    let mut entries = CursorVec::with_items(list.steps);

    loop {
        term.draw(|f| {
            let current = entries.current();
            let text = match &current.body {
                Some(body) => format!("\n\n\n{}\n\n{}", current.title, body),
                None => format!("\n\n\n{}", current.title),
            };
            let paragraph = make_paragraph(
                text,
                format!("{}, {}/{}", &title, entries.cursor + 1, entries.items.len()),
            );
            f.render_widget(paragraph, f.size())
        })?;

        match input.next()? {
            Event::Input(key) => match key {
                Key::Char('j') | Key::Char('n') | Key::Char(' ') => entries.next(),
                Key::Char('k') | Key::Char('p') | Key::Backspace => entries.previous(),
                Key::Char('q') => break,
                Key::Ctrl('c') => return Err(anyhow!("User Interrupt received")),
                _ => {}
            },
        }
    }

    Ok(())
}
