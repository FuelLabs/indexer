pub mod simple;

use crate::{data_source::types::ExecutableBlock, storage::Storage, BoxStream};

// TODO: This should be a Fuel VM (or equivalent) with external call functionality

/// A trait providing functionality for executing transactions as part of a block.
pub trait Executor<S>
where
    S: Storage,
{
    fn new() -> Self;
    fn run(
        &self,
        executable_block_stream: BoxStream<ExecutableBlock>,
        storage: S,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>>;
}
