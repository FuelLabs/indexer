pub mod echo;

use crate::data_source::types::ExecutableBlock;
use crate::BoxStream;

use futures::Future;

// TODO: This should be a Fuel VM (or equivalent) with external call functionality

/// A trait providing functionality for executing transactions as part of a block.
pub trait Executor {
    fn execute(
        &self,
        block_stream: &mut BoxStream<ExecutableBlock>,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
