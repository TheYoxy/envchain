#![feature(lazy_cell)]

use std::io::Result;

use clap::Parser;
use log::info;

use crate::{cli::Cli, commands::handle_command, utils::initialize_logging};

mod cli;
mod commands;
mod utils;

fn main() -> Result<()> {
  initialize_logging()?;

  let args = Cli::parse();

  let command = &args.command;
  info!("Got action: {command:?}");

  let result = handle_command(command);
  match result {
    Ok(result) => {
      println!("{result}");
    },
    Err(e) => {
      eprintln!("{e}");
      std::process::exit(1);
    },
  }

  Ok(())
}
