mod error;

pub use error::TransactionError;

use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_transactions_old(fname: &str) -> Result<Vec<Transaction>, String> {
    // Variant 1:
    // ________________________________________________________________________
    // let s = match std::fs::read_to_string(fname) {
    //     Ok(v) => v,
    //     Err(e) => return Err(e.to_string()),
    // };
    // let t: Vec<Transaction> = match serde_json::from_str(&s) {
    //     Ok(v) => v,
    //     Err(e) => return Err(e.to_string()),
    // };
    // Ok(t)
    // ________________________________________________________________________
    //
    // Variant 2:
    // ________________________________________________________________________
    // let s = std::fs::read_to_string(fname).map_err(|e| e.to_string())?;
    // let t = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    // Ok(t)
    // ________________________________________________________________________
    //
    // Variant 3:
    // ________________________________________________________________________
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
    // ________________________________________________________________________
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // Variant 4:
    // ________________________________________________________________________
    // std::fs::read_to_string(fname)
    //     .map_err(|e| e.into())
    //     .and_then(|s| serde_json::from_str(&s).map_err(|e| e.into()))
    // ________________________________________________________________________
    //
    // Variant 5:
    // here .into() is called automatically (see the ? operators)
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

pub fn get_first_transaction(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    let transactions = get_transactions(fname)?; // Result -> Option
    for t in transactions {
        if t.from == uname {
            return Ok(t);
        }
    }
    Err(TransactionError::Msg("Could not find transaction").into())
}
