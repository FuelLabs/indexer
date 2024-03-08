use crate::{data_source::types::ExecutableBlock, IndexableType, IntoBoxStream};
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::BoxStream;

use super::Executor;
pub struct EchoExecutor {}

impl Executor for EchoExecutor {
    fn get_stream(&self) -> (UnboundedSender<IndexableType>, BoxStream<IndexableType>) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<IndexableType>();

        (tx, UnboundedReceiverStream::new(rx).into_boxed())
    }

    fn run(
        &self,
        mut executable_block_stream: BoxStream<ExecutableBlock>,
        _indexed_item_tx: UnboundedSender<IndexableType>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            while let Some(block) = executable_block_stream.next().await {
                println!("{block:#?}")
            }
        })
    }
}
