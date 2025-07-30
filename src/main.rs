
mod rotire;

use crate::rotire::RotireAction;
use clap::Parser;
use env_logger;
use log::{error, info};
use self::rotire::filter::RotireFilter;

/// Simple program to rotate files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the directory on which rotire should run.
    #[arg(short('d'), long)]
    directory: String,

    /// How many items to keep, defaults to 4.
    #[arg(short('k'), long, default_value_t = 4)]
    keep_n: i32,

    /// The action to perform when running rotire.
    #[arg(
        default_value = "archive",
        value_parser = ["archive", "delete"],
        help = "Select the action rotire should run."
    )]
    action: String,

    /// Only apply action on the file names matching the prefix.
    #[arg(short('p'), long, default_value = None)]
    prefix_filter: Option<String>,

    /// Only apply action on the file names matching the suffix.
    #[arg(short('s'), long, default_value = None)]
    suffix_filter: Option<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let mut rotire = rotire::Rotire::new(args.directory);

    // Prepare action
    let mut action: RotireAction = RotireAction::Archive;
    match args.action.as_str() {
        "delete" => action = RotireAction::Delete,
        _ => {}
    }
    // Prepare filters
    if let Some(filter) = args.prefix_filter {
        rotire.add_filter(RotireFilter::Prefix { value: filter })
    }
    if let Some(filter) = args.suffix_filter {
        rotire.add_filter(RotireFilter::Suffix { value: filter })
    }

    let result = rotire.run(args.keep_n, action);
    if let Ok(result) = result {
        info!("Operation completed successfully: {result}")
    } else {
        error!("Operation failed: {result:?}")
    }
}
