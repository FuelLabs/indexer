use crate::IndexableType;

use super::Storage;

pub struct SimpleStorage {
    pub items: Vec<IndexableType>,
}

impl Storage for SimpleStorage {
    async fn save(&mut self, indexable_data: IndexableType) -> anyhow::Result<()> {
        self.items.push(indexable_data);
        println!("Length of items: {}", self.items.len());
        Ok(())
    }

    // fn find() -> impl futures::prelude::Future<Output = anyhow::Result<()>> {
    //     todo!()
    // }

    // fn find_many() -> impl futures::prelude::Future<Output = anyhow::Result<()>> {
    //     todo!()
    // }
}
