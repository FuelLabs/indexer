mod client;
mod queries;

use anyhow::Error;
use fuel_tx::UniqueIdentifier;
use fuel_vm::fuel_types::{canonical::Deserialize, ChainId};

use self::queries::FullBlock;
use super::types::{ExecutableBlock, Header, Transaction};
pub use crate::data_source::fuel_client::client::FuelClientDataSource;

impl TryFrom<FullBlock> for ExecutableBlock {
    type Error = Error;

    fn try_from(full_block: FullBlock) -> Result<Self, anyhow::Error> {
        let id =
            fuel_core_types::blockchain::primitives::BlockId::from(full_block.id.0 .0);
        let header = Header {
            id,
            da_height: full_block.header.da_height.0,
            transactions_count: full_block.header.transactions_count.0,
            message_receipt_count: full_block.header.message_receipt_count.0,
            transactions_root: full_block.header.transactions_root.into(),
            message_receipt_root: full_block.header.message_receipt_root.into(),
            height: full_block.header.height.0,
            prev_root: full_block.header.prev_root.into(),
            time: full_block.header.time.0,
            application_hash: full_block.header.application_hash.into(),
        };
        let consensus = match full_block.consensus {
            fuel_core_client::client::schema::block::Consensus::Genesis(g) => {
                let genesis = fuel_core_types::blockchain::consensus::Genesis {
                    chain_config_hash: g.chain_config_hash.into(),
                    coins_root: g.coins_root.into(),
                    contracts_root: g.contracts_root.into(),
                    messages_root: g.messages_root.into(),
                };
                fuel_core_types::blockchain::consensus::Consensus::Genesis(genesis)
            }
            fuel_core_client::client::schema::block::Consensus::PoAConsensus(p) => {
                let poa = fuel_core_types::blockchain::consensus::poa::PoAConsensus {
                    signature: p.signature.into_signature(),
                };
                fuel_core_types::blockchain::consensus::Consensus::PoA(poa)
            }
            fuel_core_client::client::schema::block::Consensus::Unknown => todo!(),
        };

        let transactions = full_block
            .transactions
            .into_iter()
            .map(|opaque_tx| {
                let receipts = opaque_tx.receipts.map(|r_vec| {
                    r_vec
                        .into_iter()
                        .map(|r| fuel_tx::Receipt::try_from(r).unwrap())
                        .collect::<Vec<fuel_tx::Receipt>>()
                });
                let raw = opaque_tx.raw_payload.clone();
                let tx = fuel_tx::Transaction::from_bytes(&raw.0 .0).unwrap();
                Transaction {
                    id: tx.id(&ChainId::default()),
                    receipts,
                    kind: tx,
                }
            })
            .collect::<Vec<Transaction>>();

        Ok(Self {
            id,
            header,
            consensus,
            transactions,
        })
    }
}
