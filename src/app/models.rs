use tui::widgets::TableState;

use super::super::event::Key;
use super::ActiveBlock;

#[derive(Clone)]
pub struct KeyBindings {
  pub esc: Key,
  pub quit: Key,
  pub help: Key,
  pub submit: Key,
  pub refresh: Key,
  pub toggle_theme: Key,
  pub jump_to_all_context: Key,
  pub jump_to_current_context: Key,
  pub up: Key,
  pub down: Key,
  pub left: Key,
  pub right: Key,
  pub toggle_info: Key,
  pub select_all_namespace: Key,
  pub jump_to_namespace: Key,
  pub jump_to_pods: Key,
  pub jump_to_services: Key,
  pub jump_to_nodes: Key,
  pub jump_to_deployments: Key,
  pub jump_to_configmaps: Key,
  pub jump_to_statefulsets: Key,
  pub jump_to_replicasets: Key,
}

pub struct StatefulTable<T> {
  pub state: TableState,
  pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
  pub fn new() -> StatefulTable<T> {
    StatefulTable {
      state: TableState::default(),
      items: Vec::new(),
    }
  }

  pub fn set_items(&mut self, items: Vec<T>) {
    self.items = items;
    if !self.items.is_empty() {
      let i = self
        .state
        .selected()
        .map_or(0, |i| if i > 0 { i } else { 0 });
      self.state.select(Some(i));
    }
  }

  pub fn next(&mut self) {
    let i = self.state.selected().map_or(0, |i| {
      if i >= self.items.len().wrapping_sub(1) {
        0
      } else {
        i + 1
      }
    });
    self.state.select(Some(i));
  }

  pub fn previous(&mut self) {
    let i = self.state.selected().map_or(0, |i| {
      if i == 0 {
        self.items.len().wrapping_sub(1)
      } else {
        i - 1
      }
    });
    self.state.select(Some(i));
  }

  pub fn _unselect(&mut self) {
    self.state.select(None);
  }
}

impl<T: Clone> StatefulTable<T> {
  pub fn get_selected_item(&mut self) -> Option<T> {
    self.state.selected().map(|i| self.items[i].clone())
  }
}

pub struct TabsState {
  pub titles: Vec<String>,
  pub index: usize,
  pub active_block_ids: Option<Vec<ActiveBlock>>,
  pub active_block: Option<ActiveBlock>,
}

impl TabsState {
  pub fn new(titles: Vec<String>) -> TabsState {
    TabsState {
      titles,
      index: 0,
      active_block_ids: None,
      active_block: None,
    }
  }
  pub fn with_active_blocks(titles: Vec<String>, blocks: Vec<ActiveBlock>) -> TabsState {
    TabsState {
      titles,
      index: 0,
      active_block: Some(blocks[0]),
      active_block_ids: Some(blocks),
    }
  }
  pub fn set_index(&mut self, index: usize) {
    self.index = index;
    self.set_active();
  }
  pub fn set_active(&mut self) {
    self.active_block = self.active_block_ids.as_ref().map(|ids| ids[self.index]);
  }
  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.titles.len();
    self.set_active();
  }
  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.titles.len() - 1;
    }
    self.set_active();
  }
}