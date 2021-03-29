# TIXMAN
![](https://img.shields.io/badge/unsafe-forbidden-success.svg)

## Description
TixMan is a simple transaction application in Rust. 
The goal of TixMan is to take as input a CSV transaction file and to output on the stdout the account of all the clients.

🔒 Implemented using 100% safe rust and works on all platforms supported by rust

## Usage
```bash
cargo run -- sample/transactions.csv > accounts.csv
```

## IO
```
stdin : file of transactions
stdout : accounts
stderror : all errors 
```
### stdin
You can find a sample transaction file [here](sample/transactions.csv).

### stdout
The result is in the CSV form.
```sh
client,available,held,total,locked
1,3.0,0.0,3.0,false
3,1.4,0.0,1.4,false
2,0.0,0.0,0.0,false
```

### stderror
Several types of errors are handled by TixMan. If a transaction fails, TixMan continues to process other transactions.

## Performance
TixMan tries to make as soon as possible a zero copy. There is parallelization at the level of customer 
research and transactions. The objective is to distribute calculations within the same machine. One of the 
possible improvements is to set up a cluster system to distribute the transactions in a more efficient way. 
However the synchronization problems will be more complex to manage. 
 
