use std::collections::HashMap;

use macroquad::input::KeyCode;

use crate::{game::Command, widgets::Interaction};

pub struct ControlsWidget {
    key_release_bindings: HashMap<KeyCode, Command>,
}

impl ControlsWidget {
    pub fn new() -> Self {
        Self {
            key_release_bindings: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: KeyCode, command: Command) {
        self.key_release_bindings.insert(key, command);
    }

    pub fn handle_input(&self, interaction: &Interaction) -> Option<Command> {
        for (key, command) in &self.key_release_bindings {
            if interaction.keys_released.contains(&key) {
                return Some(*command);
            }
        }

        None
    }
}
