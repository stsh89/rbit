use std::fmt;

pub enum DataType {
    ByteString(Vec<u8>),
    Integer(i64),
    List(Vec<DataType>),
    Dictionary(std::collections::HashMap<Vec<u8>, DataType>),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::ByteString(value) => write!(f, "{}", String::from_utf8_lossy(value)),
            DataType::Integer(value) => write!(f, "{}", value),
            DataType::List(list) => {
                write!(f, "[").ok();

                for x in list {
                    write!(f, "{}, ", x).ok();
                }

                write!(f, "]")
            }
            DataType::Dictionary(dict) => {
                write!(f, "{{").ok();

                for (k, v) in dict {
                    write!(f, "{}: {}", String::from_utf8(k.to_vec()).unwrap(), v).ok();
                }

                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataType::ByteString;
    use super::DataType::Dictionary;
    use super::DataType::Integer;
    use super::DataType::List;

    #[test]
    fn it_formats_integer() {
        assert_eq!(format!("{}", Integer(0)), "0");
        assert_eq!(format!("{}", Integer(1)), "1");
        assert_eq!(format!("{}", Integer(-1)), "-1");
    }

    #[test]
    fn it_formats_byte_string() {
        assert_eq!(format!("{}", ByteString("".as_bytes().to_vec())), "");
        assert_eq!(
            format!("{}", ByteString("Lorem ipsum".as_bytes().to_vec())),
            "Lorem ipsum"
        );
    }

    #[test]
    fn it_formats_list() {
        assert_eq!(format!("{}", List(vec![Integer(0)])), "[0, ]");
        assert_eq!(
            format!("{}", List(vec![Integer(0), Integer(0)])),
            "[0, 0, ]"
        );
        assert_eq!(
            format!(
                "{}",
                List(vec![ByteString("Lorem ipsum".as_bytes().to_vec())])
            ),
            "[Lorem ipsum, ]"
        );
        assert_eq!(
            format!(
                "{}",
                List(vec![ByteString("0".as_bytes().to_vec()), Integer(0)])
            ),
            "[0, 0, ]"
        );
    }

    #[test]
    fn it_formats_dictionary() {
        let mut dict = std::collections::HashMap::new();
        dict.insert("key1".as_bytes().to_vec(), Integer(18));
        assert_eq!(format!("{}", Dictionary(dict)), "{key1: 18}");
    }
}
