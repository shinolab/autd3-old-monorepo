/*
 * File: option_popup.rs
 * Project: widgets
 * Created Date: 02/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

#[derive(Clone, Copy)]
pub struct OptionPopupState {
    pub show: bool,
    ok_selected: bool,
    should_exit: bool,
}

impl Default for OptionPopupState {
    fn default() -> Self {
        Self {
            show: false,
            ok_selected: true,
            should_exit: false,
        }
    }
}

impl OptionPopupState {
    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Esc => self.show = true,
                KeyCode::Enter => {
                    if self.show {
                        if self.ok_selected {
                            self.should_exit = true;
                        } else {
                            self.ok_selected = true;
                            self.show = false;
                        }
                    }
                }
                KeyCode::Right => {
                    if self.show && self.ok_selected {
                        self.ok_selected = false;
                    }
                }
                KeyCode::Left => {
                    if self.show && !self.ok_selected {
                        self.ok_selected = true;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}

#[derive(Default)]
pub struct OptionPopup<'a> {
    title: &'a str,
}

impl<'a> StatefulWidget for OptionPopup<'a> {
    type State = OptionPopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().title(self.title).borders(Borders::ALL);

        Clear {}.render(area, buf);

        block.render(area, buf);

        let area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(area.height - 2),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(area);
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length((area[1].width - 20) / 2),
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length((area[1].width - 20) / 2),
                ]
                .as_ref(),
            )
            .split(area[1]);

        let mut ok_par = Paragraph::new("Ok")
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let mut cancel_par = Paragraph::new("Cancel")
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        if state.ok_selected {
            ok_par = ok_par.style(Style::default().fg(Color::Black).bg(Color::White));
        } else {
            cancel_par = cancel_par.style(Style::default().fg(Color::Black).bg(Color::White));
        }
        let ok_par = ok_par.dim();

        ok_par.render(area[1], buf);
        cancel_par.render(area[2], buf);
    }
}

impl<'a> OptionPopup<'a> {
    pub fn new(title: &'a str) -> OptionPopup<'a> {
        OptionPopup { title }
    }
}
