extern crate d1_filework;
use failure::Error;
use d1_filework::{get_transactions,get_first_trunsaction_for, get_transactions_b, get_transactions_c};

fn main() -> Result<(), Error> {
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

    let tran = get_first_trunsaction_for("test_data/transactions.json", "A");
    match tran {
        Ok(v)  => println!("Found transaction: {:?}", v),
        Err(e) => println!("Error: {}, Backtrace: {}", e, e.backtrace())
    }
    Ok(())
}

