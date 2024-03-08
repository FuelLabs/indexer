pub mod echo;

use tokio::sync::mpsc::UnboundedSender;

use crate::{data_source::types::ExecutableBlock, BoxStream, IndexableType};

// TODO: This should be a Fuel VM (or equivalent) with external call functionality

/// A trait providing functionality for executing transactions as part of a block.
pub trait Executor {
    fn get_stream(&self) -> (UnboundedSender<IndexableType>, BoxStream<IndexableType>);
    fn run(
        &self,
        executable_block_stream: BoxStream<ExecutableBlock>,
        indexed_item_tx: UnboundedSender<IndexableType>,
    ) -> tokio::task::JoinHandle<()>;
}
