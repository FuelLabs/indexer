use fuel_core_types::blockchain::{consensus::Consensus, primitives::BlockId};
use fuel_tx::{Bytes32, Receipt};
use tai64::Tai64;

#[derive(Debug)]
pub struct Header {
    pub id: BlockId,
    pub da_height: u64,
    pub transactions_count: u64,
    pub message_receipt_count: u64,
    pub transactions_root: Bytes32,
    pub message_receipt_root: Bytes32,
    pub height: u32,
    pub prev_root: Bytes32,
    pub time: Tai64,
    pub application_hash: Bytes32,
}

#[derive(Debug)]
pub struct ExecutableTransaction {
    pub id: fuel_tx::TxId,
    pub receipts: Option<Vec<Receipt>>,
    pub kind: fuel_tx::Transaction,
}

/// Contains all of the necessary information for an executor to process a block.
#[derive(Debug)]
pub struct ExecutableBlock {
    pub id: BlockId,
    pub header: Header,
    pub consensus: Consensus,
    pub transactions: Vec<ExecutableTransaction>,
}
