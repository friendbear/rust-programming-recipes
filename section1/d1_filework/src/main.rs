pub struct  Transaction {
    from:  String,
    to: String,
    amount: u64,
}
fn main() {
    println!("Hello, world!");

    let trans = get_transaction("test_data/transactions.jsonr");
}

fn get_transaction(file_name: &str) -> Result<Vec<Transaction>, String> {
    Ok(Vec::new())
}
