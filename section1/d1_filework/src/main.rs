use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct  Transaction {
    from:  String,
    to: String,
    amount: u64,
}
fn main() {
    println!("Hello, world!");

    let trans = get_transaction("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }
}

fn get_transaction(file_name: &str) -> Result<Vec<Transaction>, String> {
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