use indexer_core::{
    data_source::{fuel_client::FuelClientDataSource, DataSource},
    executor::{echo::EchoExecutor, Executor},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut data_source = FuelClientDataSource::new(100);
    let mut block_stream = data_source.get_stream(1);

    let echo_executor = EchoExecutor {};
    echo_executor.execute(&mut block_stream).await?;

    Ok(())
}
