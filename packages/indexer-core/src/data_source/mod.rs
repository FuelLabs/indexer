pub mod fuel_client;

use crate::BoxStream;

pub mod types {
    use fuel_core_types::{
        blockchain::{consensus::Consensus, primitives::BlockId},
        services::txpool::TransactionStatus,
    };
    use fuel_tx::{Bytes32, Receipt};
    use tai64::Tai64;

    #[derive(Debug)]
    pub struct Header {
        pub id: BlockId,
        pub da_height: u64,
        pub transactions_count: u64,
        pub message_receipt_count: u64,
        pub transactions_root: Bytes32,
        pub message_receipt_root: Bytes32,
        pub height: u32,
        pub prev_root: Bytes32,
        pub time: Tai64,
        pub application_hash: Bytes32,
    }

    #[derive(Debug)]
    pub struct Transaction {
        pub raw_payload: Vec<u8>,
        pub receipts: Option<Vec<Receipt>>,
        pub transaction_status: Option<TransactionStatus>,
    }

    // TODO: Implement a "full block" type that each data source's block
    // type will implement a conversion to.
    #[derive(Debug)]
    pub struct ExecutableBlock {
        pub id: BlockId,
        pub header: Header,
        pub consensus: Consensus,
        pub transactions: Vec<Transaction>,
    }
}

/// A trait providing a `BoxStream<T>` for blockchain data.
pub trait DataSource<T> {
    // TODO: Are blocks the only sort of data that we can get from the client or firehose?
    // TODO: Create an enum of chain data structures that can be indexed so methods can be genericized?

    /// Returns a `BoxStream<T>` starting at the desired chain height.
    fn get_stream(&mut self, starting_block_height: u32) -> BoxStream<T>;
}