use crate::util::types::{Block, BlockInfo};
use anyhow::Result;
use clap::Parser;
use cli::Context;
use std::fs::read;
use util::log;

mod cli;
mod util;

fn main() -> Result<()> {
    let Context {
        block_at_height: height,
    } = Context::parse();
    // To make this more dynamic, we could add a path feature
    // to Context and if one isn't provided it will look in
    // the current directory. For now, hard coding the file is good enough.
    let mut raw_bytes = read("blk00000.dat")?;
    find_block(&mut raw_bytes, height)
}

fn find_block(mut raw_bytes: &mut Vec<u8>, height: u64) -> Result<()> {
    raw_bytes.reverse();
    // Get BlockInfo and Blocks, print on success
    //
    // This could be done less destructively by other indexing means,
    // taking the brute force approach for convenience.
    let mut block_height = 0;
    while !raw_bytes.is_empty() {
        let block_info = BlockInfo::from_raw_bytes(&mut raw_bytes, block_height);
        block_info.validate_network()?;
        let block = Block::from_raw_bytes(&mut raw_bytes, block_info.size_as_u32());
        if block_info.height == height {
            return Ok(log(block_info, block));
        }
        block_height += 1;
    }

    Err(anyhow::anyhow!("failed to find block"))
}
