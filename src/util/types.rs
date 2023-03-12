//! Data structures for collecting, validating and converting bitcoin blockchain data into human readable output.
use crate::util::constant::{
    BLOCK_HEADER, BLOCK_HEADER_VERSION, BLOCK_SIZE, MAGIC_BYTES, MAINNET_HEX, MERKLE_ROOT_HASH,
    NONCE, PREVIOUS_BLOCK_HEADER_HASH, TARGET, TX_COUNT, UNIX_EPOCH_TIME,
};
use anyhow::Result;
use colored::*;
use hex::ToHex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct BlockInfo {
    pub(crate) height: u64,
    magic_bytes: Vec<u8>,
    size: Vec<u8>,
}
impl BlockInfo {
    pub(crate) fn new(height: u64, magic_bytes: Vec<u8>, size: Vec<u8>) -> Self {
        Self {
            height,
            magic_bytes,
            size,
        }
    }
    /// Get the size of a [Block] as a `u32`.
    ///
    /// Formats the size in bytes to little endian order
    /// then converts to hexcode before parsing as `u32`.
    pub(crate) fn size_as_u32(&self) -> u32 {
        u32::from_str_radix(
            self.size
                .clone()
                .into_iter()
                .rev()
                .collect::<Vec<u8>>()
                .encode_hex::<String>()
                .as_str(),
            16,
        )
        .expect("unable to convert hex to u32")
    }
    pub(crate) fn network_hex(&self) -> String {
        self.magic_bytes.encode_hex::<String>()
    }
    pub(crate) fn validate_network(&self) -> Result<()> {
        if self.network_hex().as_str() != MAINNET_HEX {
            anyhow::bail!("network validation failed")
        }
        Ok(())
    }
    pub(crate) fn from_raw_bytes(raw_bytes: &mut Vec<u8>, height: u64) -> Self {
        let magic_bytes = (0..MAGIC_BYTES)
            .map(|_| raw_bytes.pop().expect("expected a value for magic_bytes"))
            .collect::<Vec<u8>>();
        let size: Vec<u8> = (0..BLOCK_SIZE)
            .map(|_| raw_bytes.pop().expect("expected a value for size"))
            .collect::<Vec<u8>>();

        BlockInfo::new(height, magic_bytes, size)
    }
    pub(crate) fn log(&self) {
        println!("{}", self)
    }
}
impl std::fmt::Display for BlockInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\nBlock at Height       : {}\nNetwork               : {}\nBlock Size            : {}\n",
            "> BlockInfo".green(),
            self.height,
            self.network_hex(),
            self.size_as_u32()
        )
    }
}

#[derive(Debug)]
pub(crate) struct Block {
    block_header: BlockHeader,
    tx_count: Vec<u8>,
    tx_data: Vec<u8>,
}
impl Block {
    pub(crate) fn new(block_header: BlockHeader, tx_count: Vec<u8>, tx_data: Vec<u8>) -> Self {
        Self {
            block_header,
            tx_count,
            tx_data,
        }
    }
    pub(crate) fn from_raw_bytes(raw_bytes: &mut Vec<u8>, block_size: u32) -> Self {
        let mut raw_block_header = (0..BLOCK_HEADER)
            .map(|_| raw_bytes.pop().expect("expected a value for block_header"))
            .collect::<Vec<u8>>();
        let block_header = BlockHeader::from_raw_bytes(&mut raw_block_header);
        let tx_count = (0..TX_COUNT)
            .map(|_| raw_bytes.pop().expect("expected a value for tx_count"))
            .collect::<Vec<u8>>();
        let tx_data = (0..block_size - (BLOCK_HEADER + TX_COUNT))
            .map(|_| raw_bytes.pop().expect("expected a value for tx_data"))
            .collect::<Vec<u8>>();

        Block::new(block_header, tx_count, tx_data)
    }
    fn tx_count(&self) -> u32 {
        u32::from_str_radix(self.tx_count.clone().encode_hex::<String>().as_str(), 16)
            .expect("unable to convert hex to u32")
    }
    pub(crate) fn log(&self) {
        println!("{}", self)
    }
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\n{}\nVersion               : {}\nPrev BlockHeader Hash : {}\nMerkle Root Hash      : {}\nUnix Epoch Time       : {}\nTarget                : {}\nNonce                 : {}\n\n{}\nTX Count              : {}\nTX Data               : {:?}",
            "> Block".green(),
            "BlockHeader:".cyan(),
            self.block_header.version(),
            self.block_header.previous_block_header_hash(),
            self.block_header.merkle_root_hash(),
            self.block_header.unix_epoch_time(),
            self.block_header.target(),
            self.block_header.nonce(),
            "Transactions:".cyan(),
            self.tx_count(),
            self.tx_data,
        )
    }
}
#[derive(Debug)]
pub(crate) struct BlockHeader {
    version: Vec<u8>,
    previous_block_header_hash: Vec<u8>,
    merkle_root_hash: Vec<u8>,
    unix_epoch_time: Vec<u8>,
    target: Vec<u8>,
    nonce: Vec<u8>,
}
impl BlockHeader {
    fn new(
        version: Vec<u8>,
        previous_block_header_hash: Vec<u8>,
        merkle_root_hash: Vec<u8>,
        unix_epoch_time: Vec<u8>,
        target: Vec<u8>,
        nonce: Vec<u8>,
    ) -> Self {
        Self {
            version,
            previous_block_header_hash,
            merkle_root_hash,
            unix_epoch_time,
            target,
            nonce,
        }
    }
    fn from_raw_bytes(raw_block_header: &mut Vec<u8>) -> Self {
        let version = (0..BLOCK_HEADER_VERSION)
            .map(|_| {
                raw_block_header
                    .pop()
                    .expect("expected a value for version")
            })
            .collect::<Vec<u8>>();
        let previous_block_header_hash = (0..PREVIOUS_BLOCK_HEADER_HASH)
            .map(|_| {
                raw_block_header
                    .pop()
                    .expect("expected a value for previous hash")
            })
            .collect::<Vec<u8>>();
        let merkle_root_hash = (0..MERKLE_ROOT_HASH)
            .map(|_| {
                raw_block_header
                    .pop()
                    .expect("expected a value for merkle hash")
            })
            .collect::<Vec<u8>>();
        let unix_epoch_time = (0..UNIX_EPOCH_TIME)
            .map(|_| {
                raw_block_header
                    .pop()
                    .expect("expected a value for unix epoch time")
            })
            .collect::<Vec<u8>>();
        let target = (0..TARGET)
            .map(|_| raw_block_header.pop().expect("expected a value for target"))
            .collect::<Vec<u8>>();
        let nonce = (0..NONCE)
            .map(|_| raw_block_header.pop().expect("expected a value for nonce"))
            .collect::<Vec<u8>>();

        BlockHeader::new(
            version,
            previous_block_header_hash,
            merkle_root_hash,
            unix_epoch_time,
            target,
            nonce,
        )
    }
    fn version(&self) -> u32 {
        u32::from_str_radix(self.version.clone().encode_hex::<String>().as_str(), 16)
            .expect("unable to convert hex to u32")
    }
    fn previous_block_header_hash(&self) -> String {
        self.previous_block_header_hash
            .clone()
            .encode_hex::<String>()
    }
    fn merkle_root_hash(&self) -> String {
        self.merkle_root_hash.clone().encode_hex::<String>()
    }
    fn unix_epoch_time(&self) -> u32 {
        u32::from_str_radix(
            self.unix_epoch_time.clone().encode_hex::<String>().as_str(),
            16,
        )
        .expect("unable to convert hex to u32")
    }
    fn target(&self) -> u32 {
        u32::from_str_radix(self.target.clone().encode_hex::<String>().as_str(), 16)
            .expect("unable to convert hex to u32")
    }
    fn nonce(&self) -> u32 {
        u32::from_str_radix(self.nonce.clone().encode_hex::<String>().as_str(), 16)
            .expect("unable to convert hex to u32")
    }
}

#[cfg(test)]
mod tests {
    /// Checks that the network is valid and the block size is accurate.
    #[test]
    fn test_block_info() {
        // the first 8 bytes of blk00000
        let mut raw_bytes: Vec<u8> = vec![249, 190, 180, 217, 29, 1, 0, 0];
        raw_bytes.reverse();

        let block_info = super::BlockInfo::from_raw_bytes(&mut raw_bytes, 0);

        assert!(block_info.validate_network().is_ok());
        assert_eq!(block_info.size_as_u32(), 285);
    }
}
