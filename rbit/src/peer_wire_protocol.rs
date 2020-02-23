pub struct Msg {
    prefix: [u8; 4],
    id: Option<u8>,
    payload: Option<Payload>
}

pub struct Payload {
    data: Vec<u8>
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

    pub fn intrested() -> Msg {
        Msg{
            prefix: [0, 0, 0, 0],
            id: Some(2),
            payload: None
        }
    }
}
