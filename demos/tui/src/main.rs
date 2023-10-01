use std::{
    io::{self},
    time::Duration,
};

use autd3::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

mod pages;
mod widgets;

use pages::{
    geometry_config_page::GeometryConfigPage, geometry_config_select_page::GeometryConfigSelectPage,
};
use widgets::option_popup::{OptionPopup, OptionPopupState};

enum Page {
    GeometryConfigSelect(GeometryConfigSelectPage),
    GeometryConfig(GeometryConfigPage),
    LinkSelect,
}

fn main_() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut terminals = vec![Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(8),
        },
    )?];

    let mut app = App::new();
    let mut pages = vec![Page::GeometryConfigSelect(GeometryConfigSelectPage::new())];

    loop {
        let tidx = terminals.len() - 1;
        let pidx = pages.len() - 1;

        let event = if event::poll(Duration::from_millis(250))? {
            Some(event::read()?)
        } else {
            None
        };

        match pages[pidx] {
            Page::GeometryConfigSelect(ref mut p) => {
                terminals[tidx].draw(|f| p.ui(f))?;

                if let Some(event) = event {
                    p.handle_event(&event);
                }

                if p.should_exit() {
                    break;
                }

                if p.go_next() {
                    match p.selected() {
                        pages::geometry_config_select_page::Option::No => {
                            pages.pop();
                            pages.push(Page::LinkSelect);
                        }
                        pages::geometry_config_select_page::Option::Yes => {
                            let mut stdout = io::stdout();
                            enable_raw_mode()?;
                            execute!(stdout, EnterAlternateScreen)?;
                            terminals.push(Terminal::new(CrosstermBackend::new(stdout))?);
                            p.reset();
                            pages
                                .push(Page::GeometryConfig(GeometryConfigPage::new(&app.geometry)));
                        }
                    }
                }
            }
            Page::GeometryConfig(ref mut p) => {
                terminals[tidx].draw(|f| p.ui(f))?;
                if let Some(event) = event {
                    p.handle_event(&event);
                }
                if p.should_exit() {
                    execute!(terminals[tidx].backend_mut(), LeaveAlternateScreen)?;
                    terminals.pop();
                    app.geometry = p.geometry().to_vec();
                    pages.pop();
                }
            }

            Page::LinkSelect => {
                terminals[tidx].draw(|f| {
                    let size = f.size();
                    let paragraph =
                        Paragraph::new("Link Config".slow_blink()).wrap(Wrap { trim: true });
                    f.render_widget(paragraph, size);
                })?;

                if let Some(event) = event {
                    app.exit_popup_state.handle_event(&event);

                    if let Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        kind: KeyEventKind::Press,
                        ..
                    }) = event
                    {
                        break;
                    }
                }
            }
        }

        if app.exit_popup_state.should_exit() {
            break;
        }
    }

    disable_raw_mode()?;
    assert!(terminals.len() == 1);
    terminals[0].clear()?;

    Ok(())
}

fn main() {
    if let Err(err) = main_() {
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}

struct App {
    geometry: Vec<(Vector3, Vector3)>,
    exit_popup_state: OptionPopupState,
}

impl App {
    fn new() -> App {
        App {
            exit_popup_state: OptionPopupState::default(),
            geometry: vec![],
        }
    }
}

fn centered_rect(x: u16, y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((r.height - y) / 2),
                Constraint::Length(y),
                Constraint::Length((r.height - y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length((r.width - x) / 2),
                Constraint::Length(x),
                Constraint::Length((r.width - x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
