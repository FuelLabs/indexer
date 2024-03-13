pub mod fuel_client;

use tokio::sync::mpsc::UnboundedSender;

use crate::BoxStream;


/// A trait providing a `BoxStream<T>` for blockchain data.
pub trait DataSource<T> {
    // TODO: Are blocks the only sort of data that we can get from the client or firehose?
    // TODO: Create an enum of chain data structures that can be indexed so methods can be genericized?

    /// Returns a `BoxStream<T>` starting at the desired chain height.
    fn get_stream(&self) -> (UnboundedSender<T>, BoxStream<T>);
    fn run(
        &mut self,
        tx: UnboundedSender<T>,
        start: u32,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>>;
}
