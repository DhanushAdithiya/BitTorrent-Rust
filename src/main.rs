// use serde::Deserialize;

use core::panic;

fn decode_bencode(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().nth(0).unwrap() {
        'i' => decode_bencoded_int(encoded_value),
        'l' => decode_bencoded_list(encoded_value),
        'd' => decode_bencoded_dict(encoded_value),
        _ => decode_bencoded_strings(encoded_value),
    }
}

fn decode_bencoded_strings(encoded_strings: &str) -> (serde_json::Value, &str) {
    let (len, string_slice) = encoded_strings.split_once(":").unwrap();
    let split_string = &string_slice[..len.parse::<usize>().unwrap()];
    return (
        split_string.to_string().into(),
        &string_slice[len.parse::<usize>().unwrap()..],
    );
}

fn decode_bencoded_int(encoded_strings: &str) -> (serde_json::Value, &str) {
    let e_index = encoded_strings.find("e").unwrap();
    let int = encoded_strings[1..e_index]
        .parse::<isize>()
        .expect("Not a Number");
    return (int.into(), &encoded_strings[e_index + 1..]);
}

fn decode_bencoded_list(encoded_strings: &str) -> (serde_json::Value, &str) {
    let mut remaining = &encoded_strings[1..encoded_strings.len() - 1];
    let mut list = Vec::new();

    while !remaining.is_empty() && remaining.chars().nth(0).unwrap() != 'e' {
        let (value, rem) = decode_bencode(remaining);
        list.push(value);
        remaining = rem;
    }

    return (list.into(), remaining);
}

fn decode_bencoded_dict(encoded_strings: &str) -> (serde_json::Value, &str) {
    let mut dict = serde_json::Map::new();
    let mut remaining = &encoded_strings[1..encoded_strings.len() - 1];
    while !remaining.is_empty() && remaining.chars().nth(0).unwrap() != 'e' {
        println!("{remaining}");
        let (key, rem) = decode_bencode(remaining);
        let key = match key {
            serde_json::Value::String(key) => key,
            key => {
                panic!("Key has to be a String {key:?}")
            }
        };
        let (value, rem) = decode_bencode(rem);
        dict.insert(String::from(key), value);
        remaining = rem;
    }

    return (dict.into(), remaining);
}

fn main() {
    let encoded_value = "l5:hellol4:spami32eee";
    let decoded = decode_bencode(encoded_value);
    println!("{}", decoded.0.to_string())
}
