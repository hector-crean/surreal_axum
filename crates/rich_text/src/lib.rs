use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Doc, GetString, ReadTxn, StateVector, Text, Transact, Update};

fn doc() -> () {
    let doc = Doc::new();
    let text = doc.get_or_insert_text("article");

    let mut txn = doc.transact_mut();
    text.insert(&mut txn, 0, "hello");
    text.insert(&mut txn, 5, " world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_doc() {
        doc();
    }
}
