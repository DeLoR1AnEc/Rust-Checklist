#![allow(dead_code)]
use std::collections::HashMap;
use uuid::Uuid;

// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃         Data & Storage       ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// --- Meta ---
pub struct StructMeta {
  id: Uuid,
  name: String,
  desc: String,
}

impl StructMeta {
  fn new(name: String, desc: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      name: name,
      desc: desc,
    }
  }

  fn id(&self) -> &Uuid {
    &self.id
  }

  fn name(&self) -> &str {
    &self.name
  }

  fn set_name(&mut self, name: String) {
    self.name = name;
  }

  fn desc(&self) -> &str {
    &self.desc
  }

  fn set_desc(&mut self, desc: String) {
    self.desc = desc;
  }
}

pub trait HasMeta {
  fn meta(&self) -> &StructMeta;
  fn meta_mut(&mut self) -> &mut StructMeta;

  fn id(&self) -> &Uuid {
    self.meta().id()
  }

  fn name(&self) -> &str {
    self.meta().name()
  }

  fn set_name<S: Into<String>>(&mut self, name: S) {
    self.meta_mut().set_name(name.into());
  }

  fn desc(&self) -> &str {
    self.meta().desc()
  }

  fn set_desc<S: Into<String>>(&mut self, desc: S) {
    self.meta_mut().set_desc(desc.into())
  }
}


// --- Data ---

pub enum Node {
  Container(Container),
  Entry(Entry),
}

impl HasMeta for Node {
  fn meta(&self) -> &StructMeta {
    match self {
      Node::Entry(entry) => entry.meta(),
      Node::Container(container) => container.meta(),
    }
  }

  fn meta_mut(&mut self) -> &mut StructMeta {
    match self {
      Node::Entry(entry) => entry.meta_mut(),
      Node::Container(container) => container.meta_mut(),
    }
  }
}

// Entry
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

pub struct Entry {
  meta: StructMeta,
  state: EntryState,
}

impl Entry {
  pub fn new<S: Into<String>>(name: S, desc: S) -> Self {
    Self {
      meta: StructMeta::new(name.into(), desc.into()),
      state: EntryState::Pending,
    }
  }

  pub fn state(&self) -> &EntryState {
    &self.state
  }

  pub fn set_state(&mut self, state: EntryState) {
    self.state = state;
  }

  pub fn next_state(&mut self) {
    self.state = self.state.next();
  }

  pub fn prev_state(&mut self) {
    self.state = self.state.prev();
  }
}

impl HasMeta for Entry {
  fn meta(&self) -> &StructMeta {
    &self.meta
  }

  fn meta_mut(&mut self) -> &mut StructMeta{
    &mut self.meta
  }
}

// Container
pub struct Container {
  meta: StructMeta,
  items: HashMap<Uuid, Node>,
  order: Vec<Uuid>,
}

impl Container {
  pub fn new<S: Into<String>>(name: S, desc: S) -> Self {
    Self {
      meta: StructMeta::new(name.into(), desc.into()),
      items: HashMap::new(),
      order: Vec::new(),
    }
  }

  pub fn node(&mut self, id: &Uuid) -> Option<&mut Node> {
    self.items.get_mut(id)
  }

  // {...}

}

impl HasMeta for Container {
  fn meta(&self) -> &StructMeta {
    &self.meta
  }

  fn meta_mut(&mut self) -> &mut StructMeta {
    &mut self.meta
  }
}
