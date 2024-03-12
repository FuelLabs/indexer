


use tokio_stream::StreamExt;

use crate::{BoxStream, IndexableType};

use super::Storage;

// TODO: A distinct task is probably the wrong thing to do here;
// we might as well just instantiate the storage and associate it to the executor
// as we'll need to have some sort of storage part anyways to call things during execution
// and to be able to load things.

pub struct SimpleStorage {
    // pub items: Vec<IndexableType>,
}

impl Storage for SimpleStorage {
    fn run(
        &mut self,
        mut indexed_data_stream: BoxStream<IndexableType>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            while let Some(idx_item) = indexed_data_stream.next().await {
                match idx_item {
                    IndexableType::Block(b) => {
                        if b.height % 10000 == 0 {
                            println!("Processed block #{}", b.height);
                        }
                    }
                    IndexableType::Receipt(r) => {
                        println!("Got a receipt: {r:#?}");
                    }
                    _ => {}
                }
            }
        })
    }

    async fn save(
        &mut self,
        _indexable_data: IndexableType,
    ) -> anyhow::Result<()> {
        // self.items.push(indexable_data);
        Ok(())
    }

    // fn find() -> impl futures::prelude::Future<Output = anyhow::Result<()>> {
    //     todo!()
    // }

    // fn find_many() -> impl futures::prelude::Future<Output = anyhow::Result<()>> {
    //     todo!()
    // }
}
