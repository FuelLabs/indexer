use indexer_core::{
    data_source::{fuel_client::FuelClientDataSource, DataSource},
    executor::{simple::SimpleExecutor, Executor},
    storage::{simple::SimpleStorage, Storage},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut data_source = FuelClientDataSource::new(1000);
    let (data_source_tx, executable_block_stream) = data_source.get_stream();
    print!("Initializing chain listener...");
    let chain_listener_task = data_source.run(data_source_tx, 1);
    println!("done!");

    let echo_executor = SimpleExecutor {};
    let (indexed_item_tx, indexed_item_stream) = echo_executor.get_stream();
    print!("Initializing executor...");
    let executor_task = echo_executor.run(executable_block_stream, indexed_item_tx);
    println!("done!");

    let mut simple_storage = SimpleStorage {};
    print!("Initializing storage...");
    let storage_task = simple_storage.run(indexed_item_stream);
    println!("done!");

    let (_, _, _) = tokio::join!(chain_listener_task, executor_task, storage_task);
    Ok(())
}
