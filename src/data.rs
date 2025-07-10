#![allow(dead_code)]
use std::collections::HashMap;
use uuid::Uuid;

// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃         Data & Storage       ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// --- Meta ---
struct NodeMeta {
  id: Uuid,
  name: String,
  desc: String,
}

impl NodeMeta {
  fn new(name: String, desc: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      name: name,
      desc: desc,
    }
  }
}

// Node
pub enum Node {
  Entry(Entry),
  Container(Container),
}

impl Node {
  pub fn get_entry(&mut self) -> Option<&mut Entry> {
    match self {
      Node::Entry(e) => Some(e),
      _ => None,
    }
  }

  pub fn get_container(&mut self) -> Option<&mut Container> {
    match self {
      Node::Container(c) => Some(c),
      _ => None,
    }
  }

  pub fn is_entry(&self) -> bool {
    matches!(self, Node::Entry(_))
  }

  pub fn is_container(&self) -> bool {
    matches!(self, Node::Container(_))
  }

  // Meta
  fn meta(&self) -> &NodeMeta {
    match self {
      Node::Entry(e) => &e.meta,
      Node::Container(c) => &c.meta,
    }
  }

  fn meta_mut(&mut self) -> &mut NodeMeta {
    match self {
      Node::Entry(e) => &mut e.meta,
      Node::Container(c) => &mut c.meta,
    }
  }

  pub fn get_id(&self) -> &Uuid {
    &self.meta().id
  }

  pub fn get_name(&self) -> &str {
    &self.meta().name
  }

  pub fn set_name<S: Into<String>>(&mut self, name: S) {
    self.meta_mut().name = name.into();
  }

  pub fn get_desc(&self) -> &str {
    &self.meta().desc
  }

  pub fn set_desc<S: Into<String>>(&mut self, desc: S) {
    self.meta_mut().desc = desc.into();
  }
}

// --- Data ---

// Entry state
#[derive(Clone)]
pub enum EntryState {
  Pending,
  InProgress,
  Completed,
  Canceled,
}

impl EntryState {
  fn next(&self) -> Self {
    use EntryState::*;

    match self {
      Pending => InProgress,
      InProgress => Completed,
      Completed => Canceled,
      Canceled => Pending,
    }
  }

  fn prev(&self) -> Self {
    use EntryState::*;

    match self {
      Pending => Canceled,
      InProgress => Pending,
      Completed => InProgress,
      Canceled => Completed,
    }
  }
}

// Entry
pub struct Entry {
  meta: NodeMeta,
  state: EntryState,
}

impl Entry {
  pub fn new<S: Into<String>>(name: S, desc: S) -> Self {
    Self {
      meta: NodeMeta::new(name.into(), desc.into()),
      state: EntryState::Pending,
    }
  }
}

// Container
pub struct Container {
  meta: NodeMeta,
  order: Vec<Uuid>,
}

impl Container {
  pub fn new<S: Into<String>>(name: S, desc: S) -> Self {
    Self {
      meta: NodeMeta::new(name.into(), desc.into()),
      order: Vec::<Uuid>::new(),
    }
  }

  fn add_order(&mut self, id: &Uuid) {
    self.order.push(*id);
  }

  fn remove_order(&mut self, id: &Uuid) {
    self.order.retain(|x| x != id);
  }

  fn move_order(&mut self, id: &Uuid, pos: usize) {
    self.order.retain(|x| x != id);
    self.order.insert(pos, *id);
  }

  fn swap_order(&mut self, id1: &Uuid, id2: &Uuid) {
    if let (Some(i), Some(j)) = (
      self.order.iter().position(|x| x == id1),
      self.order.iter().position(|y| y == id2),
    ) {
      self.order.swap(i, j);
    }
  }
}

// Tree
pub struct Tree {
  root: Uuid,
  nodes: HashMap<Uuid, Node>,
  locations: HashMap<Uuid, Uuid>
}

impl Tree {
  pub fn new(root: Container) -> Self {
    let root_node = Node::Container(root);
    let root_id = *root_node.get_id();

    let mut nodes = HashMap::<Uuid, Node>::new();
    nodes.insert(root_id, root_node);

    Self {
      root: root_id,
      nodes,
      locations: HashMap::<Uuid, Uuid>::new(),
    }
  }

  // Utility
  pub fn get_node(&mut self, node_id: &Uuid) -> Result<&mut Node, String> {
    let node = self.nodes.get_mut(node_id)
      .ok_or_else(|| format!("Node {} not found", node_id))?;

    Ok(node)
  }

  fn get_entry(&mut self, node_id: &Uuid) -> Result<&mut Entry, String> {
    let entry = self.get_node(node_id)?
      .get_entry()
      .ok_or_else(|| format!("Node {} must be an entry", node_id))?;

    Ok(entry)
  }

  fn get_container(&mut self, node_id: &Uuid) -> Result<&mut Container, String> {
    let container = self.get_node(node_id)?
      .get_container()
      .ok_or_else(|| format!("Node {} must be a container", node_id))?;

    Ok(container)
  }

  pub fn get_parent_id(&self, node_id: &Uuid) -> Result<&Uuid, String> {
    if *node_id == self.root {Err("Root node doesn't have parent container")?}

    let parent_id = self.locations.get(node_id)
      .ok_or_else(|| "Parent id not found".to_string())?;

    Ok(parent_id)
  }

  pub fn get_parent_node(&mut self, node_id: &Uuid) -> Result <&mut Node, String> {
    let parent_id = *self.get_parent_id(node_id)?;

    let parent_node = self.get_node(&parent_id)?;
    Ok(parent_node)
  }

  fn get_parent_container(&mut self, node_id: &Uuid) -> Result<&mut Container, String> {
    let parent_id = *self.get_parent_id(node_id)?;

    let container = self.get_container(&parent_id)?;
    Ok(container)
  }

  fn compare_parents(&self, node_id1: &Uuid, node_id2: &Uuid) -> Result<bool, String> {
    let parent_id1 = self.get_parent_id(node_id1)?;
    let parent_id2 = self.get_parent_id(node_id2)?;

    Ok(parent_id1 == parent_id2)
  }

  // --- The interesting part ---

  // Basic Nodes operations
  pub fn add_node(&mut self, parent_id: &Uuid, node: Node) -> Result<(), String> {
    let node_id = *node.get_id();

    let container = self.get_container(parent_id)?;
    container.add_order(&node_id);

    self.nodes.insert(node_id, node);
    self.locations.insert(node_id, *parent_id);

    Ok(())
  }

  pub fn remove_node(&mut self, node_id: &Uuid) -> Result<(), String> {
    let container = self.get_parent_container(node_id)?;
    container.remove_order(node_id);

    self.nodes.remove(node_id);
    self.locations.remove(node_id);

    Ok(())
  }

  pub fn move_node(&mut self, node_id: &Uuid, new_pos: usize) -> Result<(), String> {
    let container = self.get_parent_container(node_id)?;
    container.move_order(node_id, new_pos);

    Ok(())
  }

  pub fn swap_nodes(&mut self, node_id1: &Uuid, node_id2: &Uuid) -> Result<(), String> {
    if !self.compare_parents(node_id1, node_id2)? {Err("Can only swap nodes with the same parent")?}
    if node_id1 == node_id2 {Err("Cannot swap a node with itself")?}

    let container = self.get_parent_container(node_id1)?;
    container.swap_order(node_id1, node_id2);

    Ok(())
  }

  pub fn change_parent(&mut self, new_parent_id: &Uuid, node_id: &Uuid) -> Result<(), String> {
    let parent_id = self.get_parent_id(node_id)?;
    if parent_id == new_parent_id {Err(format!("Node already is inside of container {}", parent_id))?}

    {
      let container = self.get_parent_container(node_id)?;
      container.remove_order(node_id);
    }

    {
      let new_container = self.get_container(new_parent_id)?;
      new_container.add_order(node_id);
    }

    self.locations.remove(node_id);
    self.locations.insert(*node_id, *new_parent_id);

    Ok(())
  }

  pub fn get_children_ids(&mut self, parent_id: &Uuid) -> Result<Vec<&Uuid>, String> {
    let container = self.get_container(parent_id)?;

    let children = container.order.iter().collect();
    Ok(children)
  }

  // Entry Node operation
  pub fn entry_state(&mut self, node_id: &Uuid) -> Result<&EntryState, String> {
    let entry = self.get_entry(node_id)?;

    Ok(&entry.state)
  }

  pub fn set_entry_state(&mut self, node_id: &Uuid, state: &EntryState) -> Result<(), String> {
    let entry = self.get_entry(node_id)?;

    entry.state = state.clone();
    Ok(())
  }

  pub fn entry_state_next(&mut self, node_id: &Uuid) -> Result<&EntryState, String> {
    let entry = self.get_entry(node_id)?;

    entry.state.next();

    Ok(&entry.state)
  }

  pub fn entry_state_prev(&mut self, node_id: &Uuid) -> Result<&EntryState, String> {
    let entry = self.get_entry(node_id)?;

    entry.state.prev();

    Ok(&entry.state)
  }
}
