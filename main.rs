fn create_payload(key: &str, value: u32) -> Vec<u8> {
    let payload = format!(
        r#"{{"action":"SET","key":"{}","value":{}}}"#,
        key, value
    );

    payload.into_bytes()
}

fn main() {
    let key = "key_1";
    let value = 100;

    let payload = create_payload(key, value);

    println!("Payload as string: {}", String::from_utf8(payload.clone()).unwrap());
    println!("Payload as bytes: {:?}", payload);
}