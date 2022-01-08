mod error;
pub use error::TransactionError;

use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct  Transaction {
    from:  String,
    to: String,
    amount: u64,
}


pub fn get_first_trunsaction_for(file_name: &str, uname: &str) -> Option<Transaction> {

    let  trans = get_transactions_c(file_name).ok()?;
    for t  in trans {
        if t.from == uname {
            return Some(t)
        }
    }
    None
}
pub fn get_transactions(file_name: &str) -> Result<Vec<Transaction>, String> {
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

pub fn get_transactions_b(file_name: &str) -> Result<Vec<Transaction>, TransactionError> {
    std::fs::read_to_string(file_name)
        .map_err(|e| e.into())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.into()))
}

pub fn get_transactions_c(file_name: &str) -> Result<Vec<Transaction>, TransactionError> {
    Ok(serde_json::from_str(&std::fs::read_to_string(file_name)?)?)
}
