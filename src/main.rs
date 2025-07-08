use std::io;
use crate::{
  data::{Entry, HasMeta}
};

mod tui;
mod data;

fn main() -> io::Result<()> {
  let entry = Entry::new("A", "B");
  println!("{}", entry.id());

  tui::draw()
}
