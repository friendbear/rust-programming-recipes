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
    let s = std::fs::read_to_string(file_name).unwrap();
    let t: Vec<Transaction> = serde_json::from_str(&s).unwrap();
    Ok(t)
}
