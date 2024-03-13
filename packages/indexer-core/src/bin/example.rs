use indexer_core::{
    data_source::{fuel_client::FuelClientDataSource, DataSource},
    executor::{simple::SimpleExecutor, Executor},
    storage::simple::SimpleStorage,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut data_source = FuelClientDataSource::new(1000);
    let (data_source_tx, executable_block_stream) = data_source.get_stream();
    print!("Initializing chain listener...");
    let chain_listener_task = data_source.run(data_source_tx, 2000000);
    println!("done!");

    print!("Initializing storage...");
    let simple_storage = SimpleStorage { items: vec![] };
    println!("done!");

    print!("Initializing executor...");
    let executor_task = SimpleExecutor::run(executable_block_stream, simple_storage);
    println!("done!");

    let (_, _) = tokio::join!(chain_listener_task, executor_task);
    Ok(())
}
