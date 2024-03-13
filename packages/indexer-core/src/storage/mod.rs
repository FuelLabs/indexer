pub mod simple;
use futures::Future;

use crate::IndexableType;

pub trait Storage {
    fn save(
        &mut self,
        indexable_data: IndexableType,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    // fn find() -> impl Future<Output = anyhow::Result<()>>;
    // fn find_many() -> impl Future<Output = anyhow::Result<()>>;
}
