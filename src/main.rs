#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod args;
mod calc;
mod cli;
mod filecalc;
mod gui;

fn main() {
    let cfg = args::Config::new();
    if cfg.cli {
        cli::handler();
    } else {
        if let Err(e) = gui::handler() {
            eprintln!("Gui Error: {}", e);
            eprintln!("Try with --cli");
        }
    }
}
