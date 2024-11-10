use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Parse {
        #[structopt(parse(from_os_str), short, long)]
        file: PathBuf,
    },
    Help,
    Credits,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "JSON Parser",
    about = "A simplified JSON parser written in Rust using the Pest parsing library"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
