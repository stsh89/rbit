pub struct Msg {
    prefix: [u8; 4],
    id: Option<u8>,
    payload: Option<Payload>,
}

pub struct Payload {
    data: Vec<u8>,
}

impl Msg {
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();

        buf.extend_from_slice(&self.prefix);

        if let Some(value) = self.id {
            buf.push(value);
        }

        if let Some(payload) = &self.payload {
            buf.extend_from_slice(&payload.data);
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
            payload: Some(Payload { data }),
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
