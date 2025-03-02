mod rotire;

use clap::Parser;
use env_logger;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the directory on which rotire should run.
    #[arg(short('d'), long)]
    directory: String,

    /// How many items to keep, defaults to 2.
    #[arg(short('k'), long, default_value_t = 2)]
    keep_n: i32,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let rotire = rotire::Rotire::new(args.directory);

    let result = rotire.run(args.keep_n);
}
