use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use quicli::prelude::{IntoParallelRefMutIterator,ParallelIterator};

use crate::tools::cast_float;
use crate::error::TixError;

#[derive(Debug, Deserialize)]
pub struct Client {
    #[serde(rename = "client")]
    pub id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Client {
    /// Increase the available and total funds of the client account.
    ///
    /// * `amount` - the amount
    pub fn deposit(&mut self, amount: f64) -> Result<(), TixError> {
        if self.locked {
            return Err(TixError::ClientLocked(self.id))
        }
        self.available += amount;
        self.total += amount;
        Ok(())
    }

    /// Decrease the available and total funds of the client account.
    ///
    /// * `amount` - the amount
    pub fn withdrawal(&mut self, amount: f64) -> Result<(), TixError>{
        if self.locked {
            return Err(TixError::ClientLocked(self.id))
        }
        if self.available < amount  ||  self.total < amount {
            return Err(TixError::Client(self.id, "Not enough amount available"))
        }
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    /// The client available funds decrease by the amount disputed,
    /// the held funds increase by the amount
    ///
    /// * `amount` - the amount
    pub fn dispute(&mut self, amount: f64) -> Result<(), TixError> {
        if self.locked {
            return Err(TixError::ClientLocked(self.id))
        }
        if self.available < amount {
            return Err(TixError::Client(self.id, "Not enough amount available"))
        }
        self.available -= amount;
        self.held += amount;
        Ok(())
    }

    /// The client held funds decrease by the amount no longer disputed,
    /// the available funds shoul dincrease by the amount
    ///
    /// * `amount` - the amount
    pub fn resolve(&mut self, amount: f64) -> Result<(), TixError> {
        if self.locked {
            return Err(TixError::ClientLocked(self.id))
        }
        if self.held < amount {
            return Err(TixError::Client(self.id, "Not held amount available"))
        }
        self.held -= amount;
        self.available += amount;
        Ok(())
    }

    /// The client held funds and total funds decrease
    /// the client account is locked
    ///
    /// * `amount` - the amount
    pub fn chargeback(&mut self, amount: f64) -> Result<(), TixError> {
        if self.locked {
            return Err(TixError::ClientLocked(self.id))
        }
        if self.held < amount {
            return Err(TixError::Client(self.id, "Not enough amount available"))
        }
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        Ok(())
    }

}

impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Client", 4)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &cast_float(self.available, 4))?;
        state.serialize_field("held", &cast_float(self.held, 4))?;
        state.serialize_field("total", &cast_float(self.total, 4))?;
        state.serialize_field("locked", &self.locked)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Clients {
    pub content: Vec<Client>
}

impl Clients {

    /// Get the client by id
    ///
    /// * `id` - the client id
    pub fn get(&mut self, id: u16) -> Option<&mut Client> {
        self.content.par_iter_mut().find_first(|x|
               x.id == id
        )
    }

    /// Create a new client and add it to clients
    ///
    /// * `id` - the client id
    pub fn new(&mut self, id: u16) -> &mut Client{
        self.content.push(Client{
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false
        });
        // We juste added the element that's why we can unwrap
        self.get(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actions() -> Result<(), TixError>{
        let mut client: Client = Client {
            id: 0,
            available: 10.0,
            held: 0.0,
            total: 10.0,
            locked: false
        };
        client.deposit(5.0)?;
        assert_eq!(client.available, 15.0);
        client.withdrawal(2.0)?;
        assert_eq!(client.total, 13.0);

        assert_eq!(client.total, client.available + client.held);

        Ok(())
    }
}