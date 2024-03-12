pub mod simple;
use futures::Future;

use crate::{BoxStream, IndexableType};

pub trait Storage {
    fn run(&mut self, indexed_data_stream: BoxStream<IndexableType>) -> tokio::task::JoinHandle<()>;
    fn save(&mut self, indexable_data: IndexableType) -> impl Future<Output = anyhow::Result<()>>;
    // fn find() -> impl Future<Output = anyhow::Result<()>>;
    // fn find_many() -> impl Future<Output = anyhow::Result<()>>;
}
