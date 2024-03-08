use indexer_core::{
    data_source::{fuel_client::FuelClientDataSource, DataSource},
    executor::{echo::EchoExecutor, Executor},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut data_source = FuelClientDataSource::new(100);
    let (data_source_tx, executable_block_stream) = data_source.get_stream();
    let chain_listener_task = data_source.run(data_source_tx, 1);

    let echo_executor = EchoExecutor {};
    let (indexed_item_tx, _indexed_item_stream) = echo_executor.get_stream();
    let executor_task = echo_executor.run(executable_block_stream, indexed_item_tx);

    let (_, _) = tokio::join!(chain_listener_task, executor_task);
    Ok(())
}
