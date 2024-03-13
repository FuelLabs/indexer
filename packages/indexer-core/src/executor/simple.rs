use crate::{storage::Storage, IndexableType, IndexedBlock, IndexedTransaction};

use super::types::ExecutableBlock;

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
        mut executable_block_stream: BoxStream<ExecutableBlock>,
        mut storage: S,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>> {
        tokio::task::spawn(async move {
            while let Some(block) = executable_block_stream.next().await {
                for transaction in &block.transactions {
                    if let Some(receipts) = &transaction.receipts {
                        for r in receipts.clone() {
                            let idx_type = IndexableType::Receipt(r);
                            storage.save(idx_type).await?;
                        }
                    }

                    storage
                        .save(IndexableType::Transaction(IndexedTransaction::from(
                            transaction,
                        )))
                        .await?;
                }

                storage
                    .save(IndexableType::Block(IndexedBlock::from(block)))
                    .await?;
            }

            Ok(())
        })
    }
}
