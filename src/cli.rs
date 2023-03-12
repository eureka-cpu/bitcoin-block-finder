use clap::{arg, command, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "bitcoin-block-finder",
    about = "A bitcoin block parser API that returns a block at a given height",
    version
)]
pub struct Context {
    /// The height of a block to search for.
    /// Must be a non-negative integer.
    #[arg(long, short = 'b')]
    pub block_at_height: u64,
}
