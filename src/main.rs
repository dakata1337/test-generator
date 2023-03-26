// Hide the console on Windows
#![windows_subsystem = "windows"]

use std::fs;

use clap::Parser;
use data::Project;

pub mod data;
pub mod gui;
pub mod pdf_elements;
pub mod pdf_gen;
pub mod settings;

#[derive(Parser)]
struct Args {
    path: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut state = match args.path {
        Some(path) => {
            let content = fs::read_to_string(path).unwrap();
            toml::from_str(&content)?
        }
        None => Project::default(),
    };

    for q in state.questions.iter_mut() {
        q.update_buf_from_title();
    }

    println!("Starting egui");
    gui::run_gui(state);
    Ok(())
}
