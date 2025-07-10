#![allow(unused_imports, dead_code, unused_variables)]
use crossterm::{
  execute, queue,
  terminal, cursor, style::{self, Stylize, Print, ResetColor}
};

use std::{
  collections::HashSet,
  io::{self, Write}};
use uuid::Uuid;

use crate::data::{Node, EntryState, Entry, Container, Tree};

pub fn draw() -> io::Result<()> {
  let mut stdout = io::stdout();

  execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        queue!(stdout,
          cursor::MoveTo(x, y),
          style::PrintStyledContent("█".magenta())
        )?;
      }
    }
  }

  stdout.flush()?;
  Ok(())
}

// --- Actual Code ---

// States and data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
  TreeSelect,
  TreeView,
  Settings,
}

pub struct AppState {
  current_screen: AppScreen,
  trees: Vec<Uuid>,
  selected_tree: Option<Uuid>,
  tree_view: TreeViewState
}

pub struct TreeViewState {
  selected: Option<Uuid>,
  collapsed: HashSet<Uuid>,
  pub scroll_offset: usize
}

impl TreeViewState {
  pub fn get_selected(&self) -> &Option<Uuid> {
    &self.selected
  }

  pub fn select(&mut self, id: &Uuid) -> Result<(), String> {
    self.selected = Some(*id);
    Ok(())
  }

  pub fn deselect(&mut self) -> Result<(), String> {
    self.selected = None;
    Ok(())
  }

  pub fn get_collapsed(&self) -> Result<&HashSet<Uuid>, String> {
    Ok(&self.collapsed)
  }

  pub fn add_collapsed(&mut self, tree: &mut Tree, id: &Uuid) -> Result<(), String> {
    if tree.get_node(id)?.is_entry() {Err("Node must be a container to collaps")?}

    self.collapsed.insert(*id);
    Ok(())
  }

  pub fn remove_collapsed(&mut self, id: &Uuid) -> Result<(), String> {
    self.collapsed.remove(id);
    Ok(())
  }

  pub fn is_collapsed(&self, id: &Uuid) -> Result<bool, String> {
    Ok(self.collapsed.contains(id))
  }
}

// --- General Functions ---
pub fn build_visible_nodes(
  tree: &Tree,
  view: &TreeViewState,
  current_id: &Uuid,
  depth: usize,
  out: &mut Vec<(Uuid, usize)>
) -> Result<(), String> {
  out.push((*current_id, depth));

  if view.is_collapsed(current_id)? {return Ok(())}

  let children = tree.get_children_ids(current_id)?;
  for child_id in children {
    build_visible_nodes(tree, view, child_id, depth + 1, out)?;
  }

  Ok(())
}


