use core::fmt;
use std::error;

#[derive(Debug)]
pub enum TixError {
    Transaction(u32, &'static str),
    ClientLocked(u16),
    Client(u16, &'static str),
}

impl error::Error for TixError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TixError::Transaction(_, _) => None,
            TixError::ClientLocked(_) => None,
            TixError::Client(_, _) => None,
        }
    }
}

impl fmt::Display for TixError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TixError::Transaction(tx,message) => write!(fmt, "Transaction: {}, Message : {}", tx, message),
            TixError::ClientLocked(id) => write!(fmt, "Client {} is locked", id),
            TixError::Client(id,message) => write!(fmt, "Client: {}, Message : {}", id, message),
        }
    }
}