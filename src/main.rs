mod rotire;

use clap::Parser;
use env_logger;
use log::{error, info};
use crate::rotire::RotireAction;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the directory on which rotire should run.
    #[arg(short('d'), long)]
    directory: String,

    /// How many items to keep, defaults to 4.
    #[arg(short('k'), long, default_value_t = 4)]
    keep_n: i32,

    #[arg(
        default_value = "archive-delete",
        value_parser = ["archive-delete", "delete"],
        help = "Select the action rotire should run."
    )]
    action: String
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let rotire = rotire::Rotire::new(args.directory);
    let mut action: RotireAction = RotireAction::ArchiveAndDelete;
    match args.action.as_str() {
        "delete" => {
            action = RotireAction::Delete
        }
        _ => {}
    }

    let result = rotire.run(args.keep_n, action);
    if let Ok(result) = result {
        info!("Rotire ran successfully: {result}.")
    } else {
        error!("Rotire failed: {result:?}.")
    }
}
