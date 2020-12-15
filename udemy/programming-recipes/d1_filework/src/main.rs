extern crate d1_filework;

use d1_filework::{get_first_transaction, get_transactions};
use failure::Error;

fn main() -> Result<(), Error> {
    let trx = get_transactions("test_data/transactions.json")?;
    for t in trx {
        println!("{:?}", t);
    }
    match get_first_transaction("test_data/transactions.json", "Mate") {
        Ok(v) => println!("Found transaction: {:?}", v),
        Err(e) => println!("Error: {}, Backtrace: {}", e, e.backtrace()),
    }
    Ok(())
}
