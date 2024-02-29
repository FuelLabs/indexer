use std::str::FromStr;

use super::queries::ClientExt;
use crate::data_source::types::ExecutableBlock;
use crate::data_source::DataSource;
use crate::{BoxStream, IntoBoxStream};

use fuel_core_client::client::{
    pagination::{PageDirection, PaginationRequest},
    FuelClient,
};
use tokio::task::JoinSet;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct FuelClientDataSource {
    pub task_set: tokio::task::JoinSet<()>,
    pub block_page_size: i32,
}

impl FuelClientDataSource {
    pub fn new(block_page_size: i32) -> Self {
        Self {
            task_set: JoinSet::new(),
            block_page_size,
        }
    }
}

impl DataSource<ExecutableBlock> for FuelClientDataSource {
    fn get_stream(&mut self, start: u32) -> BoxStream<ExecutableBlock> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<ExecutableBlock>();

        let client = FuelClient::from_str("https://beta-5.fuel.network").unwrap();
        let mut cursor = Some(start.to_string());
        let block_page_size = self.block_page_size;

        let _task = self.task_set.spawn(async move {
            loop {
                let request = PaginationRequest {
                    cursor: cursor.clone(),
                    results: block_page_size,
                    direction: PageDirection::Forward,
                };

                let paginated_result = client.full_blocks(request).await.unwrap();

                cursor = paginated_result.cursor;

                for block in paginated_result.results {
                    let exec_block = ExecutableBlock::from(block);
                    tx.send(exec_block).unwrap();
                }
            }
        });

        UnboundedReceiverStream::new(rx).into_boxed()
    }
}
