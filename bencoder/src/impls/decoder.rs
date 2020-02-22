use super::data_type::DataType;
use super::parser::Parser;

pub fn decode(s: &[u8]) -> DataType {
    Parser {
        position: 0,
        input: s,
    }
    .parse()
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::DataType::ByteString;
    use super::DataType::Dictionary;
    use super::DataType::Integer;
    use super::DataType::List;

    #[test]
    fn it_decodes_integer() {
        match decode("i0e".as_bytes()) {
            Integer(value) => assert_eq!(value, 0),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_decodes_string() {
        match decode("11:Lorem ipsum".as_bytes()) {
            ByteString(value) => assert_eq!(value, "Lorem ipsum".as_bytes().to_vec()),
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_decodes_list() {
        match decode("li0ee".as_bytes()) {
            List(value) => match value[0] {
                Integer(value) => assert_eq!(value, 0),
                _ => assert_eq!(true, false),
            },
            _ => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_decodes_dictionary() {
        match decode("d3:key11:Lorem ipsume".as_bytes()) {
            Dictionary(value) => match value.get(&"key".as_bytes().to_vec()).unwrap() {
                ByteString(value) => assert_eq!(*value, "Lorem ipsum".as_bytes().to_vec()),
                _ => assert_eq!(true, false),
            },
            _ => assert_eq!(true, false),
        }
    }
}
