mod models;
mod engine;
mod tools;
mod error;

use quicli::prelude::*;
use structopt::StructOpt;

use crate::models::client::{Clients};
use crate::models::transaction::TxHistory;
use crate::engine::{Engine};

#[derive(Debug, StructOpt)]
struct Cli {
    tx_file: String,
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    let mut r_file = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&args.tx_file)?;
    let mut headers = r_file.byte_headers()?.clone();
    headers.trim();
    let mut raw_record = csv::ByteRecord::new();

    // Clients initialisation
    let mut clients: Clients = Clients {
        content: Vec::new()
    };

    // Transactions initialisation
    let mut tx_history: TxHistory = TxHistory {
        content: Vec::new()
    };

    // The transaction manager initialisation
    let mut engine: Engine = Engine {
        clients: &mut clients,
        tx_history: &mut tx_history
    };

    // Read every line of CSV and apply the transaction
    while r_file.read_byte_record(&mut raw_record)? {
        raw_record.trim();
        match  raw_record.deserialize(Some(&headers)){
            Ok(mut tx) => {
                match engine.add(&mut tx) {
                    Err(e) => eprintln!("{}",e),
                    _ => {}
                }
            },
            Err(_e) => eprintln!("The transaction is invalid, Row : {:?}", raw_record)
        }


    }

    // Write the result of transactions on the stdout
    // Errors will be written on the std error (2)
    if let Err(e) = engine.write() {
        eprintln!("{}",e);
    }

    Ok(())
}