use crate::IndexableType;

use super::Storage;

pub struct SimpleStorage {
    pub items: Vec<IndexableType>,
}

impl Storage for SimpleStorage {
    async fn save(&mut self, indexed_data: IndexableType) -> anyhow::Result<()> {
        self.items.push(indexed_data);
        println!("Length of items: {}", self.items.len());
        Ok(())
    }
}
