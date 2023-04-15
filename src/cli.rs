use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum FigCommands {
    /// Records the addition of money
    Add {
        #[clap(value_parser)]
        /// The amount to add
        amount: Option<f64>,

        #[clap(short, long, value_parser)]
        /// A message for the added amount
        message: Option<String>,
    },

    /// Record the taking of money
    Take {
        #[clap(value_parser)]
        /// The amount to add
        amount: Option<f64>,

        #[clap(short, long, value_parser)]
        /// A message for the added amount
        message: Option<String>,
    },

    /// Shows a log of all transactions
    Log,
}

#[derive(Parser)]
#[clap(author, version)]
pub struct FigArgs {
    #[clap(subcommand)]
    pub command: Option<FigCommands>,
}
