/*
 * File: geometry_config_select_page.rs
 * Project: pages
 * Created Date: 03/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

pub enum Option {
    No,
    Yes,
}

pub struct GeometryConfigSelectPage {
    state: ListState,
    should_exit: bool,
    go_next: bool,
}

impl GeometryConfigSelectPage {
    pub fn new() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            should_exit: false,
            go_next: false,
        }
    }

    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(0),
                ]
                .as_ref(),
            )
            .split(size);

        let paragraph =
            Paragraph::new("Press q or ESC to exit.".slow_blink()).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);

        let paragraph =
            Paragraph::new("Configure geometry?".slow_blink()).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[1]);

        let items =
            List::new(vec![ListItem::new("No"), ListItem::new("Yes")]).highlight_symbol("> ");

        f.render_stateful_widget(items, chunks[2], &mut self.state);
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Esc | KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Down => {
                    self.state.select(Some(match self.state.selected() {
                        Some(i) => {
                            if i >= 2 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    }));
                }
                KeyCode::Up => {
                    self.state.select(Some(match self.state.selected() {
                        Some(i) => {
                            if i == 0 {
                                2
                            } else {
                                i - 1
                            }
                        }
                        None => 2,
                    }));
                }
                KeyCode::Enter => self.go_next = true,
                _ => {}
            }
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn go_next(&self) -> bool {
        self.go_next
    }

    pub fn selected(&self) -> Option {
        match self.state.selected().unwrap_or(0) {
            0 => Option::No,
            1 => Option::Yes,
            _ => unreachable!(),
        }
    }

    pub fn reset(&mut self) {
        self.go_next = false;
    }
}
