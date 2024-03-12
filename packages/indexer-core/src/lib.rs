use fuel_core_types::blockchain::primitives::BlockId;
use fuel_tx::Bytes32;
use futures::stream::Stream;
use tai64::Tai64;

pub mod data_source;
pub mod executor;
pub mod storage;

/// A boxed stream.
///
/// This allows for the exposure of a stream without requiring downstream
/// users to know/match the underlying implementation details.
pub type BoxStream<T> = core::pin::Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>;

/// A boxed future.
pub type BoxFuture<'a, T> =
    core::pin::Pin<Box<dyn futures::Future<Output = T> + Send + Sync + 'a>>;

/// A trait providing methods for converting types implementing the `Stream` trait
/// into a `BoxStream<T>`.
pub trait IntoBoxStream: Stream {
    /// Converts T into `BoxStream<T>`
    fn into_boxed(self) -> BoxStream<Self::Item>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Box::pin(self)
    }
}

impl<S> IntoBoxStream for S where S: Stream + Send + Sync + 'static {}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: fuel_tx::TxId,
    pub receipts: Vec<fuel_tx::Receipt>,
    pub kind: fuel_tx::Transaction,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: BlockId,
    pub height: u32,
    pub da_height: u64,
    pub msg_receipt_count: u64,
    pub tx_root: Bytes32,
    pub msg_receipt_root: Bytes32,
    pub prev_id: BlockId,
    pub prev_root: Bytes32,
    pub timestamp: Tai64,
    pub application_hash: Bytes32,
    pub transactions: Vec<Transaction>,
}

/// Represents the different types of data that can be indexed from the chain.
///
/// It is intended that instances of this type will be persisted into storage
/// through the `save()` method of the Storage trait. Thus, any implementation of
/// the trait should cover all variants of this type.
#[derive(Debug, Clone)]
pub enum IndexableType {
    Block(Block),
    Transaction(Transaction),
    Input(fuel_tx::Input),
    Output(fuel_tx::Output),
    StorageSlot(fuel_tx::StorageSlot),
    UtxoId(fuel_tx::UtxoId),
    TxPointer(fuel_tx::TxPointer),
    Policies(fuel_tx::policies::Policies),
    PanicInstruction(fuel_tx::PanicInstruction),
    Receipt(fuel_tx::Receipt),
    // TODO: Figure out associated data and rename variant
    ArbitrarySwayStruct,
}
