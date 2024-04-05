// main.rs

mod ui;

use std::io::Result;

fn main() -> Result<()> {
    ui::setup_ui()?;
    Ok(())
}