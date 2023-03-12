use crate::{Block, BlockInfo};

mod constant;
pub(crate) mod types;

/// Print all values.
/// Written as a util fn to make adding extra printing functionality easier.
pub(crate) fn log(block_info: BlockInfo, block: Block) {
    block_info.log();
    block.log();
}
