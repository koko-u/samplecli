use clap::Clap;
use std::fs::File;
use std::io::{BufReader, BufRead, stdin};
use anyhow::Result;
use samplecli::rpn::RpnCalculator;
use std::path::PathBuf;

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
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()> {

    let opts: Opts = Opts::parse();
    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)?;
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)?;
    }

    Ok(())
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calculator = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        let answer = calculator.eval(&line)?;
        println!("{}", answer);
    }

    Ok(())
}