use serde_derive::*;

#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error)
}

impl From<std::io::Error> for TransactionError {
    fn from (err: std::io::Error) -> Self {
        TransactionError::LoadError(err)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from (err: serde_json::Error) -> Self {
        TransactionError::ParseError(err)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct  Transaction {
    from:  String,
    to: String,
    amount: u64,
}
fn main() {
    println!("Hello, world!");

    let trans = get_transactions("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }
    
    let trans = get_transactions_b("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }
    
    let trans = get_transactions_c("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }
}

fn get_transactions(file_name: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(file_name) {
        Ok(v) => v,
        Err(e) => return Err(format!("{:?}", e)),
    };

    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(format!("{:?}", e)),
    };
    Ok(t)
}

fn get_transactions_b(file_name: &str) -> Result<Vec<Transaction>, TransactionError> {
    std::fs::read_to_string(file_name)
        .map_err(|e| e.into())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.into()))
}

fn get_transactions_c(file_name: &str) -> Result<Vec<Transaction>, TransactionError> {
    Ok(serde_json::from_str(&std::fs::read_to_string(file_name)?)?)
}