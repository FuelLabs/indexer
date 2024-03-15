pub mod fuel_client;

use tokio::sync::mpsc::UnboundedSender;

use crate::BoxStream;

/// A trait providing a `BoxStream<T>` for blockchain data.
pub trait DataSource<T> {
    /// Returns a `BoxStream<T>`.
    fn get_stream(&self) -> (UnboundedSender<T>, BoxStream<T>);

    /// Spawns a task that will fetch blocks at a starting height from a data source
    /// and send them downstream.
    fn run(
        &mut self,
        tx: UnboundedSender<T>,
        start: u32,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>>;
}
