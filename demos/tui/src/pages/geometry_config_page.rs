/*
 * File: geometry_config_page.rs
 * Project: pages
 * Created Date: 03/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::Vector3;
use crossterm::event::Event;
use ratatui::prelude::*;

use crate::widgets::geometry_list::{GeometryList, GeometryListState};

pub struct GeometryConfigPage {
    geometry_list_state: GeometryListState,
}

impl GeometryConfigPage {
    pub fn new(geometry: &[(Vector3, Vector3)]) -> Self {
        Self {
            geometry_list_state: GeometryListState::new(geometry),
        }
    }

    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let area = f.size();

        let list = GeometryList::new();
        f.render_stateful_widget(list, area, &mut self.geometry_list_state);
        if let Some((x, y)) = self.geometry_list_state.cursor_position() {
            f.set_cursor(area.x + x as u16, area.y + y);
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.geometry_list_state.handle_event(event);
    }

    pub fn should_exit(&self) -> bool {
        self.geometry_list_state.should_exit()
    }

    pub fn geometry(&self) -> &[(Vector3, Vector3)] {
        self.geometry_list_state.geometry()
    }
}
