use futures::stream::Stream;

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
