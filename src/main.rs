use std::process::ExitCode;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(short, long, value_enum)]
    which: Wich,
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Wich {
    BanhMi,
    Blt,
    TunaSalad,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match args.which {
        Wich::BanhMi => 81,
        Wich::Blt => 23,
        Wich::TunaSalad => 42,
    }
    .into()
}
