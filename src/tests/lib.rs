use std::collections::TreeMap;

#[test]
fn test_encode_simple_string() {
    let object = "simple string";

    let encoded = yaml::encode(&object);

    let decoded: = yaml::decode(encoded.as_slice()).unwrap();
}
