use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    Server,
    Client,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    pub mode: Mode,
}
