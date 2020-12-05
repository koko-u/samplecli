use clap::Clap;

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

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}
