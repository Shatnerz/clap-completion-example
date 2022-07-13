use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]

pub struct Args {
    /// Attempting to get hostname completion to work
    #[clap(long, value_hint = ValueHint::Hostname)]
    pub host: String,
}
