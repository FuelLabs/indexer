use futures::Future;

pub trait StoragePort {
    fn save() -> impl Future<Output = anyhow::Result<()>>;
    fn find() -> impl Future<Output = anyhow::Result<()>>;
    fn find_many() -> impl Future<Output = anyhow::Result<()>>;
}
