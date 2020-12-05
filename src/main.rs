use clap::Clap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clap, Debug)]
#[clap(
  name = "My RPN Program",
  version = "1.0.0",
  author = "KOZAKI Tsuneaki",
  about = "Sper awesome PRN caluculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name ="FILE")]
    formula_file: Option<String>,
}

fn main() {

    let opts: Opts = Opts::parse();
    if let Some(path) = opts.formula_file {
        if let Ok(f) = File::open(path) {
            let reader = BufReader::new(f);
            run(reader, opts.verbose);
        }
    } else {
        println!("no file is specified");
    }
}

fn run(reader: BufReader<File>, verbose: bool) {
    for line in reader.lines() {
        println!("{}", line.unwrap_or_default());
    }
}