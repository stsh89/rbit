use std::collections::BTreeMap;

use super::data_type::DataType;
use super::data_type::DataType::ByteString;
use super::data_type::DataType::Dictionary;
use super::data_type::DataType::Integer;
use super::data_type::DataType::List;

use super::event::{DATA_END, DICT_BEGIN, INTEGER_BEGIN, LIST_BEGIN, STRING_DELIMITER};

pub fn encode(t: &DataType) -> Vec<u8> {
    match t {
        ByteString(value) => encode_string(value),
        Integer(value) => encode_integer(*value),
        List(value) => encode_list(value),
        Dictionary(value) => encode_dictionary(value),
    }
}

fn encode_string(s: &[u8]) -> Vec<u8> {
    let mut res = vec![];

    for x in s.len().to_string().as_bytes() {
        res.push(*x);
    }

    res.push(STRING_DELIMITER);

    for x in s {
        res.push(*x);
    }

    res
}

fn encode_integer(i: i64) -> Vec<u8> {
    let mut res = vec![INTEGER_BEGIN];

    for x in i.to_string().as_bytes() {
        res.push(*x);
    }

    res.push(DATA_END);
    res
}

fn encode_list(list: &[DataType]) -> Vec<u8> {
    let mut res = vec![LIST_BEGIN];

    for element in list {
        for x in &encode(element) {
            res.push(*x);
        }
    }

    res.push(DATA_END);
    res
}

fn encode_dictionary(dict: &std::collections::HashMap<Vec<u8>, DataType>) -> Vec<u8> {
    let mut res = vec![DICT_BEGIN];

    let ordered: BTreeMap<_, _> = dict.iter().collect();

    for (key, value) in ordered {
        for x in &encode_string(key) {
            res.push(*x);
        }

        for x in &encode(value) {
            res.push(*x);
        }
    }

    res.push(DATA_END);
    res
}

#[cfg(test)]
mod tests {
    use super::encode;
    use super::DataType::ByteString;
    use super::DataType::Dictionary;
    use super::DataType::Integer;
    use super::DataType::List;

    #[test]
    fn it_encodes_integer() {
        assert_eq!(encode(&Integer(0)), "i0e".as_bytes().to_vec());
        assert_eq!(encode(&Integer(-1)), "i-1e".as_bytes().to_vec());
        assert_eq!(encode(&Integer(1)), "i1e".as_bytes().to_vec());
    }

    #[test]
    fn it_encodes_string() {
        assert_eq!(
            encode(&ByteString("Lorem ipsum".as_bytes().to_vec())),
            "11:Lorem ipsum".as_bytes().to_vec()
        );
        assert_eq!(
            encode(&ByteString("".as_bytes().to_vec())),
            "0:".as_bytes().to_vec()
        );
    }

    #[test]
    fn it_encodes_list() {
        assert_eq!(encode(&List(vec![])), "le".as_bytes().to_vec());
        assert_eq!(encode(&List(vec![Integer(0)])), "li0ee".as_bytes().to_vec());
        assert_eq!(
            encode(&List(vec![
                Integer(0),
                ByteString("Lorem ipsum".as_bytes().to_vec())
            ])),
            "li0e11:Lorem ipsume".as_bytes().to_vec()
        );
    }

    #[test]
    fn it_encodes_dictionary() {
        let dict = std::collections::HashMap::new();
        assert_eq!(encode(&Dictionary(dict)), "de".as_bytes().to_vec());

        let mut dict2 = std::collections::HashMap::new();
        dict2.insert("key1".as_bytes().to_vec(), Integer(0));
        dict2.insert(
            "key2".as_bytes().to_vec(),
            ByteString("Lorem ipsum".as_bytes().to_vec()),
        );
        assert_eq!(
            encode(&Dictionary(dict2)),
            "d4:key1i0e4:key211:Lorem ipsume".as_bytes().to_vec()
        );
    }
}
