use crate::data_source::types::ExecutableBlock;
use futures::StreamExt;

use crate::BoxStream;

use super::Executor;
pub struct EchoExecutor {}

impl Executor for EchoExecutor {
    async fn execute(
        &self,
        block_stream: &mut BoxStream<ExecutableBlock>,
    ) -> anyhow::Result<()> {
        while let Some(block) = block_stream.next().await {
            println!("{block:#?}")
        }

        Ok(())
    }
}
