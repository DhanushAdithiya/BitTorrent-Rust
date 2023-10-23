fn decode_bencode(encoded_value: &str) -> serde_json::Value {
    match encoded_value.chars().nth(0).unwrap() {
        'i' => decode_bencoded_int(encoded_value),
        'l' => decode_bencoded_list(encoded_value),
        'd' => decode_bencoded_dict(encoded_value),
        _ => decode_bencoded_strings(encoded_value),
    }
}

fn decode_bencoded_strings(encoded_strings: &str) -> serde_json::Value {
    let colon_index = encoded_strings.find(":").unwrap();
    let str_len = &encoded_strings[..colon_index].parse::<usize>().unwrap();
    let split = &encoded_strings[colon_index + 1..colon_index + 1 + str_len];
    return serde_json::Value::String(split.to_string());
}

fn decode_bencoded_int(encoded_strings: &str) -> serde_json::Value {
    let e_index = encoded_strings.find("e").unwrap();
    let int = encoded_strings[1..e_index]
        .parse::<isize>()
        .expect("Not a Number");
    return serde_json::Value::Number(int.into());
}

fn decode_bencoded_list(encoded_strings: &str) -> serde_json::Value {
    let mut remaining = &encoded_strings[1..encoded_strings.len() - 1];
    let mut list = Vec::new();

    while remaining.len() > 3 {
        let op = decode_bencode(remaining);
        match op.is_number() {
            true => {
                remaining = &remaining[2 + &op.to_string().len()..];
                list.push(op);
            }
            false => {
                let colon_index = remaining.find(":").unwrap();
                remaining = &remaining[colon_index + &op.as_str().unwrap().len() + 1..];
                list.push(op);
            }
        }
    }

    return serde_json::Value::Array(list);
}

fn decode_bencoded_dict(encoded_strings: &str) -> serde_json::Value {
    let mut dict = serde_json::Map::new();
    let mut remaining = &encoded_strings[1..encoded_strings.len() - 1];
    while remaining.len() > 3 {
        let key = decode_bencode(remaining);
        let colon_index = remaining.find(":").unwrap();
        remaining = &remaining[colon_index + &key.as_str().unwrap().len() + 1..];

        let value = decode_bencode(remaining);
        let _ = match value {
            serde_json::Value::Number(_) => {
                remaining = &remaining[2 + &value.to_string().len()..];
            }
            serde_json::Value::String(_) => {
                let colon_index = remaining.find(":").unwrap();
                remaining = &remaining[colon_index + &value.as_str().unwrap().len() + 1..];
            }
            _ => {
                panic!("TODO")
            }
        };

        dict.insert(String::from(key.as_str().unwrap()), value);
    }

    return dict.into();
}

fn main() {
    println!("{}", decode_bencode("d5:hello3:hele"))
}
