use rand::Rng;
use reqwest::blocking::Client;
use sha2::{Digest, Sha512};

use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader, BatchList};
use sawtooth_sdk::signing::{create_context, CryptoFactory};

fn sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn create_payload(key: &str, value: u32) -> Vec<u8> {
    let payload = format!(
        r#"{{"action":"SET","key":"{}","value":{}}}"#,
        key, value
    );
    payload.into_bytes()
}

fn create_transaction(payload: Vec<u8>) -> (Transaction, String, String) {
    let context = create_context("secp256k1").unwrap();
    let private_key = context.new_random_private_key().unwrap();
    let factory = CryptoFactory::new(&context);
    let signer = factory.new_signer(private_key);

    let public_key = signer.get_public_key().unwrap().as_hex();

    let payload_hash = sha512(&payload);

    let mut header = TransactionHeader::new();
    header.set_family_name("intkey".into());
    header.set_family_version("1.0".into());
    header.set_signer_public_key(public_key.clone());
    header.set_batcher_public_key(public_key.clone());
    header.set_payload_sha512(payload_hash);

    let header_bytes = header.write_to_bytes().unwrap();
    let signature = signer.sign(&header_bytes).unwrap();

    let mut txn = Transaction::new();
    txn.set_header(header_bytes);
    txn.set_header_signature(signature.clone());
    txn.set_payload(payload);

    (txn, signature, public_key)
}

fn create_batch(txn: Transaction, signer_pub: String) -> Vec<u8> {
    let context = create_context("secp256k1").unwrap();
    let private_key = context.new_random_private_key().unwrap();
    let factory = CryptoFactory::new(&context);
    let signer = factory.new_signer(private_key);

    let mut batch_header = BatchHeader::new();
    batch_header.set_signer_public_key(signer_pub);
    batch_header.set_transaction_ids(vec![txn.get_header_signature().to_string()].into());

    let header_bytes = batch_header.write_to_bytes().unwrap();
    let signature = signer.sign(&header_bytes).unwrap();

    let mut batch = Batch::new();
    batch.set_header(header_bytes);
    batch.set_header_signature(signature);
    batch.set_transactions(vec![txn].into());

    let mut batch_list = BatchList::new();
    batch_list.set_batches(vec![batch].into());

    batch_list.write_to_bytes().unwrap()
}

fn send(batch_bytes: Vec<u8>) {
    let client = Client::new();

    let res = client
        .post("http://localhost:8008/batches")
        .header("Content-Type", "application/octet-stream")
        .body(batch_bytes)
        .send();

    match res {
        Ok(r) => println!("Submitted: {:?}", r.status()),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    for i in 1..=5 {
        let key = format!("key_{}", i);
        let value = rand::thread_rng().gen_range(1..1000);

        let payload = create_payload(&key, value);

        let (txn, _, pubkey) = create_transaction(payload);

        let batch_bytes = create_batch(txn, pubkey);

        send(batch_bytes);

        println!("Sent transaction {}", key);
    }
}
