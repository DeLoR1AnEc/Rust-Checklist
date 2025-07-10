use std::io::{self, Write};
use tui::draw;

mod tui;
mod data;

fn main() -> io::Result<()> {
  draw()?;

  Ok(())
}
