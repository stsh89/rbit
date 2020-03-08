pub struct Msg {
    pub prefix: [u8; 4],
    pub id: Option<u8>,
    pub payload: Option<Vec<u8>>,
}

pub struct SingleFileInfo {
    pub piece_length: u32,
    pub pieces: Vec<u8>,
    pub name: String,
    pub length: u32,
}

pub struct Piece {
    pub index: u32,
    pub begin: u32,
    pub block: Vec<u8>,
}

impl Msg {
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();

        buf.extend_from_slice(&self.prefix);

        if let Some(value) = self.id {
            buf.push(value);
        }

        if let Some(payload) = &self.payload {
            buf.extend_from_slice(&payload);
        }

        buf
    }

    pub fn request(index: u32, begin: u32, length: u32) -> Msg {
        println!("REQUEST MESSAGE");
        let mut data = Vec::<u8>::new();

        data.extend(&index.to_be_bytes());
        data.extend(&begin.to_be_bytes());
        data.extend(&length.to_be_bytes());

        Msg {
            prefix: [0, 0, 0, 13],
            id: Some(6),
            payload: Some(data),
        }
    }

    pub fn interested() -> Msg {
        println!("INTRESTED MESSAGE");

        Msg {
            prefix: [0, 0, 0, 1],
            id: Some(2),
            payload: None,
        }
    }
}

pub fn read_file_info(meta_info: &bencoder::DataType) -> SingleFileInfo {
    let piece_length = *meta_info
        .get_dict_value(b"piece length")
        .unwrap()
        .get_integer_value()
        .unwrap() as u32;

    let pieces = meta_info
        .get_dict_value(b"pieces")
        .unwrap()
        .get_string_value()
        .unwrap()
        .to_vec();

    let name = String::from_utf8_lossy(
        meta_info
            .get_dict_value(b"name")
            .unwrap()
            .get_string_value()
            .unwrap(),
    )
    .to_string();

    let length = *meta_info
        .get_dict_value(b"length")
        .unwrap()
        .get_integer_value()
        .unwrap() as u32;

    let info = SingleFileInfo {
        piece_length,
        length,
        pieces,
        name,
    };

    println!("Info pieces: {:?}", info.pieces);
    println!("Info piece length: {}", info.piece_length);
    println!("Info name: {}", info.name);
    println!("Info length: {}", info.length);

    info
}
