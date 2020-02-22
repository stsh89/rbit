use super::data_type::DataType;

use super::event::{
    DATA_END, DICT_BEGIN, INTEGER_BEGIN, LIST_BEGIN, NINE_BYTE, STRING_DELIMITER, ZERO_BYTE,
};

pub struct Parser<'input> {
    pub position: usize,
    pub input: &'input [u8],
}

impl<'input> Parser<'input> {
    pub fn parse(&mut self) -> DataType {
        match self.input[self.position] {
            INTEGER_BEGIN => {
                self.position += 1;
                self.parse_integer()
            }
            ZERO_BYTE..=NINE_BYTE => self.parse_string(),
            DICT_BEGIN => {
                self.position += 1;
                self.parse_dict()
            }
            LIST_BEGIN => {
                self.position += 1;
                self.parse_list()
            }
            _ => DataType::Integer(0),
        }
    }

    fn parse_integer(&mut self) -> DataType {
        let mut res = vec![];

        loop {
            match self.input[self.position] {
                DATA_END => {
                    self.position += 1;
                    return DataType::Integer(chars_to_integer(res));
                }
                byte => {
                    self.position += 1;
                    res.push(byte as char);
                }
            }
        }
    }

    fn parse_string(&mut self) -> DataType {
        let mut res = vec![];
        let mut length = vec![];

        loop {
            match self.input[self.position] {
                ZERO_BYTE..=NINE_BYTE => {
                    let digit = self.input[self.position] as char;
                    length.push(digit);
                    self.position += 1;
                }
                STRING_DELIMITER => {
                    self.position += 1;
                    let len = chars_to_integer(length);

                    for _i in 0..len {
                        res.push(self.input[self.position]);
                        self.position += 1;
                    }

                    return DataType::ByteString(res);
                }
                _ => {
                    return DataType::ByteString(b"".to_vec());
                }
            }
        }
    }

    fn parse_dict(&mut self) -> DataType {
        let mut map = std::collections::HashMap::new();

        loop {
            match self.input[self.position] {
                DATA_END => {
                    self.position += 1;
                    return DataType::Dictionary(map);
                }
                _ => match self.parse_string() {
                    DataType::ByteString(str) => {
                        map.insert(str, self.parse());
                    }
                    _ => {
                        return DataType::Dictionary(map);
                    }
                },
            }
        }
    }

    fn parse_list(&mut self) -> DataType {
        let mut v = vec![];

        loop {
            match self.input[self.position] {
                DATA_END => {
                    self.position += 1;
                    return DataType::List(v);
                }
                _ => {
                    v.push(self.parse());
                }
            }
        }
    }
}

fn chars_to_integer(chars: Vec<char>) -> i64 {
    chars
        .iter()
        .cloned()
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
