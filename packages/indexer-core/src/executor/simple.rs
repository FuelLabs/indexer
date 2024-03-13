use crate::{
    data_source::types::ExecutableBlock, storage::Storage, Block, IndexableType,
    Transaction,
};
use futures::StreamExt;

use crate::BoxStream;

use super::Executor;
pub struct SimpleExecutor {}

impl<S> Executor<S> for SimpleExecutor
where
    S: Storage + Send + 'static,
{
    fn new() -> Self {
        Self {}
    }

    fn run(
        &self,
        mut executable_block_stream: BoxStream<ExecutableBlock>,
        mut storage: S,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>> {
        tokio::task::spawn(async move {
            while let Some(block) = executable_block_stream.next().await {
                for ref transaction in block.transactions {
                    if let Some(receipts) = &transaction.receipts {
                        for r in receipts.clone() {
                            let idx_type = IndexableType::Receipt(r.clone());
                            storage.save(idx_type).await?;
                        }
                    }

                    let idx_tx = Transaction {
                        id: transaction.id,
                        receipts: transaction.receipts.clone().unwrap_or_default(),
                        kind: transaction.kind.clone(),
                    };

                    storage.save(IndexableType::Transaction(idx_tx)).await?;
                }

                let b = Block {
                    id: block.id,
                    height: block.header.height,
                    da_height: block.header.da_height,
                    msg_receipt_count: block.header.message_receipt_count,
                    tx_root: block.header.transactions_root,
                    msg_receipt_root: block.header.message_receipt_root,
                    prev_root: block.header.prev_root,
                    timestamp: block.header.time,
                    application_hash: block.header.application_hash,
                    transactions: vec![],
                };

                storage.save(IndexableType::Block(b)).await?;
            }

            Ok(())
        })
    }
}
