pub mod simple;
pub mod types;

use crate::{storage::Storage, BoxStream, IndexedBlock, IndexedTransaction};

use self::types::{ExecutableBlock, ExecutableTransaction};

// TODO: This should be a Fuel VM (or equivalent) with external call functionality

/// A trait providing functionality for executing transactions as part of a block.
pub trait Executor<S>
where
    S: Storage,
{
    fn new() -> Self;
    fn run(
        executable_block_stream: BoxStream<ExecutableBlock>,
        storage: S,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>>;
}

impl From<&ExecutableTransaction> for IndexedTransaction {
    fn from(transaction: &ExecutableTransaction) -> Self {
        IndexedTransaction {
            id: transaction.id,
            receipts: transaction.receipts.clone().unwrap_or_default(),
            kind: transaction.kind.clone(),
        }
    }
}

impl From<ExecutableBlock> for IndexedBlock {
    fn from(block: ExecutableBlock) -> Self {
        Self {
            id: block.id,
            height: block.header.height,
            da_height: block.header.da_height,
            msg_receipt_count: block.header.message_receipt_count,
            tx_root: block.header.transactions_root,
            msg_receipt_root: block.header.message_receipt_root,
            prev_root: block.header.prev_root,
            timestamp: block.header.time,
            application_hash: block.header.application_hash,
            transactions: block
                .transactions
                .iter()
                .map(IndexedTransaction::from)
                .collect::<Vec<IndexedTransaction>>(),
        }
    }
}
