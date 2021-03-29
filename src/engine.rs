use std::io;
use std::error::Error;

use crate::models::client::{Clients, Client};
use crate::models::transaction::{TxHistory, Transaction, TxType};
use crate::error::TixError;

/// Engine is the transaction manager
#[derive(Debug)]
pub struct Engine<'a> {
    pub clients: &'a mut Clients,
    pub tx_history: &'a mut TxHistory
}

impl Engine<'_> {
    /// Add and apply the transaction
    ///
    /// * `tx` - the transaction that we want to add.
    pub fn add(&mut self, tx: &mut Transaction) -> Result<(), TixError> {
        let client_option = self.clients.get(tx.client);
        let client: &mut Client;
        match client_option {
            Some(c) => {
                client = c;
            },
            None => {
                client = self.clients.new(tx.client);
            }
        }

        // Action for every type of transaction
        match &tx.tx_type {
            TxType::Deposit => {
                self.tx_history.add_tx(*tx)?;
                match tx.amount {
                    Some(amount) => client.deposit(amount),
                    None => Err(TixError::Transaction(tx.tx,"Invalid transaction"))
                }
            },
            TxType::Withdrawal => {
                self.tx_history.add_tx(*tx)?;
                match tx.amount {
                    Some(amount) => client.withdrawal(amount),
                    None => Err(TixError::Transaction(tx.tx,"Invalid transaction"))
                }
            },
            TxType::Dispute => {
                match self.tx_history.get_tx(tx.tx) {
                    Some(x) => {
                        x.is_disputed = true;
                        match x.amount {
                            Some(amount) => client.dispute(amount),
                            None => Err(TixError::Transaction(tx.tx,"Invalid transaction"))
                        }
                    },
                    None => return Err(TixError::Transaction(tx.tx,"Transaction not exist"))
                }
            },
            TxType::Resolve => {
                match self.tx_history.get_tx(tx.tx) {
                    Some(x) => {
                        if !x.is_disputed {
                            return Err(TixError::Transaction(tx.tx,"Transaction not disputed"))
                        }
                        x.is_disputed = false;
                        match tx.amount {
                            Some(amount) => client.resolve(amount),
                            None => Err(TixError::Transaction(tx.tx,"Invalid transaction"))
                        }
                    },
                    None => return Err(TixError::Transaction(tx.tx,"Transaction not exist"))
                }
            },
            TxType::Chargeback => {
                match self.tx_history.get_tx(tx.tx) {
                    Some(x) => {
                        if !x.is_disputed {
                            return Err(TixError::Transaction(tx.tx,"Transaction not disputed"))
                        }
                        x.is_disputed = false;
                        match x.amount {
                            Some(amount) => client.chargeback(amount),
                            None => Err(TixError::Transaction(tx.tx,"Invalid transaction"))
                        }
                    },
                    None => return Err(TixError::Transaction(tx.tx,"Transaction not exist"))
                }
            }
        }?;

        Ok(())
    }

    /// Serialize all clients by transforming Clients to CSV
    pub fn write(&self) -> Result<(),Box<dyn Error>> {
        let mut writer = csv::Writer::from_writer(io::stdout());
        for client in &self.clients.content {
            writer.serialize(client)?;
        }
        writer.flush()?;
        Ok(())
    }
}