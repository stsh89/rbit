use bencoder;
use crypto::digest::Digest;
mod file_reader;
extern crate percent_encoding;
use crypto::sha1::Sha1;

fn main() {
    let res = bencoder::decode(&file_reader::read_file("./test_data/sample.torrent")[..]);
    match res {
        bencoder::DataType::Dictionary(dict) => {
            let mut hasher = Sha1::new();

            let info = bencoder::encode(dict.get(&b"info".to_vec()).unwrap());
            hasher.input(&info);

            let mut encoded_announce_url = &Vec::new();

            if let bencoder::DataType::ByteString(value) = dict.get(&b"announce".to_vec()).unwrap()
            {
                encoded_announce_url = value;
            }

            let announce_url = String::from_utf8(encoded_announce_url.to_vec()).unwrap();
            let mut out = [0u8; 20];
            hasher.result(&mut out);
            println!("{:?}", out);
            println!("Announce url: {}", announce_url);
            let hash_info =
                percent_encoding::percent_encode(&out, percent_encoding::DEFAULT_ENCODE_SET);
            println!("Percent encoded info key: {:}", hash_info);
        }
        _ => println!("Bencoding error"),
    }
}
