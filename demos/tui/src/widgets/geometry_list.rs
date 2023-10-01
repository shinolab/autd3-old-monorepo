/*
 * File: geometry_list.rs
 * Project: widgets
 * Created Date: 03/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EditMode {
    PositionSelect(usize),
    RotationSelect(usize),
    PositionEditing(usize),
    RotationEditing(usize),
}

pub struct GeometryListState {
    state: ListState,
    items: Vec<(Vector3, Vector3)>,
    edit_mode: Option<EditMode>,
    input: String,
    cursor_position: usize,
    should_exit: bool,
}

impl GeometryListState {
    pub fn new(geometry: &[(Vector3, Vector3)]) -> GeometryListState {
        GeometryListState {
            state: ListState::default(),
            items: geometry.to_vec(),
            edit_mode: None,
            input: String::new(),
            cursor_position: 0,
            should_exit: false,
        }
    }

    pub fn cursor_position(&self) -> Option<(u16, u16)> {
        let idx = self.state.selected().unwrap_or(0);
        match self.edit_mode {
            Some(EditMode::PositionEditing(i)) => Some((
                8 + self.items[idx]
                    .0
                    .iter()
                    .take(i)
                    .map(|x| x.to_string().len() as u16 + 2)
                    .sum::<u16>()
                    + self.cursor_position as u16
                    + 1,
                (3 * idx as u16) + 2,
            )),
            Some(EditMode::RotationEditing(i)) => Some((
                8 + self.items[idx]
                    .1
                    .iter()
                    .take(i)
                    .map(|x| x.to_string().len() as u16 + 2)
                    .sum::<u16>()
                    + self.cursor_position as u16
                    + 1,
                (3 * idx as u16) + 3,
            )),
            _ => None,
        }
    }

    fn handle_event_unselected(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Char('n') => {
                self.items.push((Vector3::zeros(), Vector3::zeros()));
                self.state.select(Some(self.items.len() - 1));
            }
            KeyCode::Up => {
                if !self.items.is_empty() {
                    self.previous();
                }
            }
            KeyCode::Down | KeyCode::Enter | KeyCode::Right => {
                if !self.items.is_empty() {
                    self.next();
                }
            }
            _ => {}
        }
    }

    fn handle_event_dev_selected(&mut self, idx: usize, code: &KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Char('n') => {
                self.items
                    .insert(idx + 1, (Vector3::zeros(), Vector3::zeros()));
                self.state.select(Some(idx + 1));
            }
            KeyCode::Char('d') => {
                self.items.remove(idx);
                match self.items.len() {
                    0 => self.unselect(),
                    l if l <= idx => self.state.select(Some(l - 1)),
                    _ => self.state.select(Some(idx)),
                }
            }
            KeyCode::Down => {
                self.next();
            }
            KeyCode::Up => {
                self.previous();
            }
            KeyCode::Left => {
                self.unselect();
            }
            KeyCode::Enter | KeyCode::Right => {
                self.edit_mode = Some(EditMode::PositionSelect(0));
            }
            _ => {}
        }
    }

    fn handle_event_pos_selected(&mut self, idx: usize, i: usize, code: &KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.edit_mode = None;
            }
            KeyCode::Down => {
                self.edit_mode = Some(EditMode::RotationSelect(i));
            }
            KeyCode::Up => {
                self.previous();
                self.edit_mode = Some(EditMode::RotationSelect(i));
            }
            KeyCode::Left => {
                if i > 0 {
                    self.edit_mode = Some(EditMode::PositionSelect(i - 1));
                } else {
                    self.previous();
                    self.edit_mode = Some(EditMode::RotationSelect(2));
                }
            }
            KeyCode::Right => {
                if i < 2 {
                    self.edit_mode = Some(EditMode::PositionSelect(i + 1));
                } else {
                    self.edit_mode = Some(EditMode::RotationSelect(0));
                }
            }
            KeyCode::Enter => {
                self.edit_mode = Some(EditMode::PositionEditing(i));
                self.input = self.items[idx].0[i].to_string();
                self.cursor_position = self.input.len();
            }
            _ => {}
        }
    }

    fn handle_event_rot_selected(&mut self, idx: usize, i: usize, code: &KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.edit_mode = None;
            }
            KeyCode::Down => {
                self.next();
                self.edit_mode = Some(EditMode::PositionSelect(i));
            }
            KeyCode::Up => {
                self.edit_mode = Some(EditMode::PositionSelect(i));
            }
            KeyCode::Left => {
                if i > 0 {
                    self.edit_mode = Some(EditMode::RotationSelect(i - 1));
                } else {
                    self.edit_mode = Some(EditMode::PositionSelect(2));
                }
            }
            KeyCode::Right => {
                if i < 2 {
                    self.edit_mode = Some(EditMode::RotationSelect(i + 1));
                } else {
                    self.next();
                    self.edit_mode = Some(EditMode::PositionSelect(0));
                }
            }
            KeyCode::Enter => {
                self.edit_mode = Some(EditMode::RotationEditing(i));
                self.input = self.items[idx].1[i].to_string();
                self.cursor_position = self.input.len();
            }
            _ => {}
        }
    }

    fn handle_event_pos_editing(&mut self, idx: usize, i: usize, code: &KeyCode) {
        match code {
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => {
                if let Some(r) = self.commit_input() {
                    self.items[idx].0[i] = r;
                }
                self.edit_mode = Some(EditMode::PositionSelect(i));
            }
            &KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            _ => {}
        }
    }

    fn handle_event_rot_editing(&mut self, idx: usize, i: usize, code: &KeyCode) {
        match code {
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => {
                if let Some(r) = self.commit_input() {
                    self.items[idx].1[i] = r;
                }
                self.edit_mode = Some(EditMode::RotationSelect(i));
            }
            &KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            _ => {}
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match self.state.selected() {
                Some(idx) => match self.edit_mode {
                    Some(EditMode::PositionSelect(i)) => {
                        self.handle_event_pos_selected(idx, i, code)
                    }
                    Some(EditMode::RotationSelect(i)) => {
                        self.handle_event_rot_selected(idx, i, code)
                    }
                    Some(EditMode::PositionEditing(i)) => {
                        self.handle_event_pos_editing(idx, i, code)
                    }
                    Some(EditMode::RotationEditing(i)) => {
                        self.handle_event_rot_editing(idx, i, code)
                    }
                    None => {
                        self.handle_event_dev_selected(idx, code);
                    }
                },
                None => {
                    self.handle_event_unselected(code);
                }
            }
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let mut new_input = self.input.clone();
        new_input.insert(self.cursor_position, new_char);
        match new_input.parse::<f64>() {
            Ok(_) => {
                self.input = new_input;
                self.move_cursor_right();
            }
            Err(_) => {}
        }
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            let new_input: String = before_char_to_delete.chain(after_char_to_delete).collect();
            match new_input.parse::<f64>() {
                Ok(_) => {
                    self.input = new_input;
                    self.move_cursor_left();
                }
                Err(_) => {}
            }
        }
    }

    fn commit_input(&mut self) -> Option<f64> {
        let res = self.input.parse::<f64>();
        self.input.clear();
        self.reset_cursor();
        match res {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        }));
    }

    fn previous(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.items.len() - 1,
        }));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn geometry(&self) -> &[(Vector3, Vector3)] {
        &self.items
    }
}

#[derive(Default)]
pub struct GeometryList {}

impl GeometryList {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for GeometryList {
    type State = GeometryListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let get_pos_text = |idx: usize, i: usize, val: f64| {
            Span::styled(
                if state.state.selected() == Some(idx) {
                    match state.edit_mode {
                        Some(EditMode::PositionEditing(i_)) if i == i_ => state.input.clone(),
                        _ => format!("{val}"),
                    }
                } else {
                    format!("{val}")
                },
                if state.state.selected() == Some(idx) {
                    match state.edit_mode {
                        Some(EditMode::PositionSelect(i_)) if i == i_ => {
                            Style::default().fg(Color::LightGreen)
                        }
                        _ => Style::default(),
                    }
                } else {
                    Style::default()
                },
            )
        };
        let get_rot_text = |idx: usize, i: usize, val: f64| {
            Span::styled(
                if state.state.selected() == Some(idx) {
                    match state.edit_mode {
                        Some(EditMode::RotationEditing(i_)) if i == i_ => state.input.clone(),
                        _ => format!("{val}"),
                    }
                } else {
                    format!("{val}")
                },
                if state.state.selected() == Some(idx) {
                    match state.edit_mode {
                        Some(EditMode::RotationSelect(i_)) if i == i_ => {
                            Style::default().fg(Color::LightGreen)
                        }
                        _ => Style::default(),
                    }
                } else {
                    Style::default()
                },
            )
        };

        let items: Vec<ListItem> = state
            .items
            .iter()
            .enumerate()
            .map(|(i, g)| {
                let header = Line::styled(
                    format!("Device {i}"),
                    if state.state.selected() == Some(i) {
                        match state.edit_mode {
                            None => Style::default().fg(Color::LightGreen),
                            _ => Style::default(),
                        }
                    } else {
                        Style::default()
                    },
                );

                ListItem::new(vec![
                    header,
                    Line::from(vec![
                        "  Pos: (".into(),
                        get_pos_text(i, 0, g.0.x),
                        ", ".into(),
                        get_pos_text(i, 1, g.0.y),
                        ", ".into(),
                        get_pos_text(i, 2, g.0.z),
                        ")".into(),
                    ]),
                    Line::from(vec![
                        "  Rot: (".into(),
                        get_rot_text(i, 0, g.1.x),
                        ", ".into(),
                        get_rot_text(i, 1, g.1.y),
                        ", ".into(),
                        get_rot_text(i, 2, g.1.z),
                        ")".into(),
                    ]),
                ])
            })
            .collect();

        let helper_text = if state.state.selected().is_some() {
            match state.edit_mode {
                None => {
                    "n : Add new device, d : Delete selected device, ↓↑ : Select device, → or Enter : Edit device, ← : Unselect device, ESC or q : Exit"
                }
                Some(EditMode::PositionSelect(_)) | Some(EditMode::RotationSelect(_))  => {
                    "←↓↑→ : Select parameter, Enter : Enter edit mode, ESC or q : Return to device select"
                }
                Some(EditMode::PositionEditing(_)) | Some(EditMode::RotationEditing(_))  => {
                    "Enter or ESC or q : Exit edit mode"
                }
            }
        } else {
            "n: Add new device, ↓↑→ or Enter: Select device, ESC or q : Exit"
        };

        let items = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Geometry ({}) ", helper_text)),
        );

        ratatui::widgets::StatefulWidget::render(items, area, buf, &mut state.state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_event_unselected_test() {
        let mut state = GeometryListState::new(&[]);

        assert_eq!(state.state.selected(), None);
        assert_eq!(state.items.len(), 0);
        state.handle_event_unselected(&KeyCode::Char('n'));
        assert_eq!(state.state.selected(), Some(0));
        assert_eq!(state.items.len(), 1);

        state.state.select(None);
        state.handle_event_unselected(&KeyCode::Char('n'));
        assert_eq!(state.state.selected(), Some(1));
        assert_eq!(state.items.len(), 2);

        state.state.select(None);
        state.handle_event_unselected(&KeyCode::Up);
        assert_eq!(state.state.selected(), Some(1));

        state.state.select(None);
        state.handle_event_unselected(&KeyCode::Down);
        assert_eq!(state.state.selected(), Some(0));
    }

    #[test]
    fn handle_event_dev_selected() {
        let mut state = GeometryListState::new(&[]);

        state.handle_event_unselected(&KeyCode::Char('n'));
        assert_eq!(state.state.selected(), Some(0));
        state.handle_event_dev_selected(0, &KeyCode::Char('q'));
        assert_eq!(state.state.selected(), None);

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));

        state.items[0].0.x = 1.0;
        state.handle_event_dev_selected(0, &KeyCode::Char('n'));
        assert_eq!(state.state.selected(), Some(1));
        assert_eq!(state.items.len(), 2);
        assert_eq!(state.items[0].0.x, 1.0);
        assert_eq!(state.items[1].0.x, 0.0);

        state.items[1].0.x = 2.0;
        state.handle_event_dev_selected(0, &KeyCode::Char('n'));
        assert_eq!(state.state.selected(), Some(1));
        assert_eq!(state.items.len(), 3);
        assert_eq!(state.items[0].0.x, 1.0);
        assert_eq!(state.items[1].0.x, 0.0);
        assert_eq!(state.items[2].0.x, 2.0);

        state.handle_event_dev_selected(0, &KeyCode::Down);
        assert_eq!(state.state.selected(), Some(2));
        state.handle_event_dev_selected(0, &KeyCode::Down);
        assert_eq!(state.state.selected(), Some(0));

        state.handle_event_dev_selected(0, &KeyCode::Up);
        assert_eq!(state.state.selected(), Some(2));
        state.handle_event_dev_selected(0, &KeyCode::Up);
        assert_eq!(state.state.selected(), Some(1));

        assert!(state.edit_mode.is_none());
        state.handle_event_dev_selected(0, &KeyCode::Right);
        assert!(state.edit_mode.is_some());

        state.edit_mode = None;
        state.handle_event_dev_selected(1, &KeyCode::Char('d'));
        assert_eq!(state.state.selected(), Some(1));
        assert_eq!(state.items.len(), 2);
        assert_eq!(state.items[0].0.x, 1.0);
        assert_eq!(state.items[1].0.x, 2.0);

        state.handle_event_dev_selected(1, &KeyCode::Char('d'));
        assert_eq!(state.state.selected(), Some(0));
        assert_eq!(state.items.len(), 1);
        assert_eq!(state.items[0].0.x, 1.0);

        state.handle_event_dev_selected(0, &KeyCode::Char('d'));
        assert_eq!(state.state.selected(), None);
        assert_eq!(state.items.len(), 0);
    }

    #[test]
    fn handle_event_pos_selected() {
        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);

        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(0)));
        state.handle_event_pos_selected(0, 0, &KeyCode::Char('q'));
        assert_eq!(state.edit_mode, None);

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);

        state.handle_event_pos_selected(0, 0, &KeyCode::Right);
        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(1)));
        state.handle_event_pos_selected(0, 1, &KeyCode::Right);
        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(2)));
        state.handle_event_pos_selected(0, 2, &KeyCode::Right);
        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(0)));

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);

        state.handle_event_pos_selected(0, 0, &KeyCode::Left);
        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(2)));

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);

        state.handle_event_pos_selected(0, 0, &KeyCode::Right);
        state.handle_event_pos_selected(0, 1, &KeyCode::Down);
        assert_eq!(state.state.selected(), Some(1));
        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(1)));

        state.handle_event_pos_selected(0, 1, &KeyCode::Right);
        state.handle_event_pos_selected(0, 2, &KeyCode::Up);
        assert_eq!(state.state.selected(), Some(0));
        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(2)));

        for key in [KeyCode::Enter] {
            let mut state = GeometryListState::new(&[]);
            state.handle_event_unselected(&KeyCode::Char('n'));
            state.handle_event_dev_selected(0, &KeyCode::Right);

            state.handle_event_pos_selected(0, 0, &key);
            assert_eq!(state.edit_mode, Some(EditMode::PositionEditing(0)));
            assert_eq!(state.input, state.items[0].0.x.to_string());
            assert_eq!(state.cursor_position, state.input.len());
        }
    }

    #[test]
    fn handle_event_rot_selected() {
        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);
        state.handle_event_pos_selected(0, 0, &KeyCode::Down);

        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(0)));
        state.handle_event_rot_selected(0, 0, &KeyCode::Char('q'));
        assert_eq!(state.edit_mode, None);

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);
        state.handle_event_pos_selected(0, 0, &KeyCode::Down);

        state.handle_event_rot_selected(0, 0, &KeyCode::Right);
        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(1)));
        state.handle_event_rot_selected(0, 1, &KeyCode::Right);
        assert_eq!(state.edit_mode, Some(EditMode::RotationSelect(2)));
        state.handle_event_rot_selected(0, 2, &KeyCode::Right);
        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(0)));

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);
        state.handle_event_pos_selected(0, 0, &KeyCode::Down);

        state.handle_event_rot_selected(0, 0, &KeyCode::Left);
        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(2)));

        let mut state = GeometryListState::new(&[]);
        state.handle_event_unselected(&KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Char('n'));
        state.handle_event_dev_selected(0, &KeyCode::Right);
        state.handle_event_pos_selected(0, 0, &KeyCode::Down);

        state.handle_event_rot_selected(0, 0, &KeyCode::Right);
        state.handle_event_rot_selected(0, 1, &KeyCode::Down);
        assert_eq!(state.state.selected(), Some(0));
        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(1)));

        state.handle_event_rot_selected(0, 1, &KeyCode::Right);
        state.handle_event_rot_selected(0, 2, &KeyCode::Up);
        assert_eq!(state.state.selected(), Some(0));
        assert_eq!(state.edit_mode, Some(EditMode::PositionSelect(2)));

        for key in [KeyCode::Enter] {
            let mut state = GeometryListState::new(&[]);
            state.handle_event_unselected(&KeyCode::Char('n'));
            state.handle_event_dev_selected(0, &KeyCode::Right);
            state.handle_event_pos_selected(0, 0, &KeyCode::Down);

            state.handle_event_rot_selected(0, 0, &key);
            assert_eq!(state.edit_mode, Some(EditMode::RotationEditing(0)));
            assert_eq!(state.input, state.items[0].0.x.to_string());
            assert_eq!(state.cursor_position, state.input.len());
        }
    }
}
