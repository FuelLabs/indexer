mod client;
mod queries;

use self::queries::FullBlock;
use super::types::{ExecutableBlock, Header, Transaction};
pub use crate::data_source::fuel_client::client::FuelClientDataSource;

impl From<FullBlock> for ExecutableBlock {
    fn from(full_block: FullBlock) -> Self {
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

        let transactions = full_block.transactions.into_iter().map(|opaque_tx| {
            Transaction {
                // id,
                raw_payload: opaque_tx.raw_payload.to_vec(),
                receipts: opaque_tx.receipts.map(|r_vec| r_vec.into_iter().map(|r| fuel_tx::Receipt::try_from(r).unwrap()).collect::<Vec<fuel_tx::Receipt>>()),
                transaction_status: opaque_tx.status.map(|s| match s {
                    fuel_core_client::client::schema::tx::TransactionStatus::SubmittedStatus(s) => fuel_core_types::services::txpool::TransactionStatus::Submitted { time: s.time.0 },
                    fuel_core_client::client::schema::tx::TransactionStatus::SuccessStatus(s) => fuel_core_types::services::txpool::TransactionStatus::Success { block_id: s.block.id.0.0.into(), time: s.time.0, result: s.program_state.map(|ps| ps.try_into().unwrap()) },
                    fuel_core_client::client::schema::tx::TransactionStatus::SqueezedOutStatus(s) => fuel_core_types::services::txpool::TransactionStatus::SqueezedOut { reason: s.reason },
                    fuel_core_client::client::schema::tx::TransactionStatus::FailureStatus(s) => fuel_core_types::services::txpool::TransactionStatus::Failed { block_id: s.block.id.0.0.into(), time: s.time.0, reason: s.reason, result: s.program_state.map(|ps| ps.try_into().unwrap()) },
                    fuel_core_client::client::schema::tx::TransactionStatus::Unknown => todo!(),
                }),
        }}).collect::<Vec<Transaction>>();

        Self {
            id,
            header,
            consensus,
            transactions,
        }
    }
}
