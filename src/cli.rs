use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    VerifyDb {},
    ClearDb {},
    GenerateData {},
}
