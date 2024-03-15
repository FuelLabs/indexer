pub mod simple;
use futures::Future;

use crate::IndexableType;

/// A trait for providing methods for a storage solution. An executor will use
/// a type that implements this trait in order to persist indexed information.
pub trait Storage {
    /// Saves indexed data to storage.
    fn save(
        &mut self,
        indexed_data: IndexableType,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
