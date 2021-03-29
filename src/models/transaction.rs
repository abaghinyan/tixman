use serde::{Deserialize, Serialize};
use quicli::prelude::{IntoParallelRefMutIterator,ParallelIterator};
use crate::error::TixError;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Transaction {
    pub tx: u32,
    #[serde(rename = "type")]
    pub tx_type:TxType,
    pub client: u16,
    pub amount: Option<f64>,
    #[serde(skip_serializing,skip_deserializing)]
    pub is_disputed: bool
}

/// The history of all valid transactions
#[derive(Debug)]
pub struct TxHistory {
    pub content: Vec<Transaction>
}

impl TxHistory {
    pub fn get_tx(&mut self, tx: u32) -> Option<&mut Transaction> {
        self.content.par_iter_mut().find_first(|x|
            x.tx == tx
        )
    }

    pub fn add_tx(&mut self, tx: Transaction) -> Result<(), TixError>{
        match  self.get_tx(tx.tx) {
            Some(t) => Err(TixError::Transaction(t.tx, "Already exist")),
            None => Ok(self.content.push(tx))
        }
    }
}