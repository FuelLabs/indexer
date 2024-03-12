use crate::{
    data_source::types::ExecutableBlock, Block, IndexableType, IntoBoxStream, Transaction,
};
use fuel_tx::{Bytes32, Script};
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::BoxStream;

use super::Executor;
pub struct SimpleExecutor {}

impl Executor for SimpleExecutor {
    fn get_stream(&self) -> (UnboundedSender<IndexableType>, BoxStream<IndexableType>) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<IndexableType>();

        (tx, UnboundedReceiverStream::new(rx).into_boxed())
    }

    fn run(
        &self,
        mut executable_block_stream: BoxStream<ExecutableBlock>,
        indexed_item_tx: UnboundedSender<IndexableType>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            while let Some(block) = executable_block_stream.next().await {
                for ref transaction in block.transactions {
                    if let Some(receipts) = &transaction.receipts {
                        for r in receipts.clone() {
                            let idx_type = IndexableType::Receipt(r.clone());
                            indexed_item_tx.send(idx_type).unwrap();
                        }
                    }

                    // TODO: IndexableType::Transaction needs to include tx ID and kind
                    let idx_tx = Transaction {
                        id: Bytes32::zeroed(),
                        receipts: transaction.receipts.clone().unwrap_or_default(),
                        kind: fuel_tx::Transaction::Script(Script::default()),
                    };

                    indexed_item_tx
                        .send(IndexableType::Transaction(idx_tx))
                        .unwrap()
                }

                // TODO: Does FullBlock have previous block ID?
                let b = Block {
                    id: block.id,
                    height: block.header.height,
                    da_height: block.header.da_height,
                    msg_receipt_count: block.header.message_receipt_count,
                    tx_root: block.header.transactions_root,
                    msg_receipt_root: block.header.message_receipt_root,
                    prev_id: Bytes32::zeroed().into(),
                    prev_root: block.header.prev_root,
                    timestamp: block.header.time,
                    application_hash: block.header.application_hash,
                    transactions: vec![],
                };

                indexed_item_tx.send(IndexableType::Block(b)).unwrap();
            }
        })
    }
}
