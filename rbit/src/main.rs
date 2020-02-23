mod file_reader;

extern crate crypto;
extern crate percent_encoding;

use bencoder;
use requester;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::io::prelude::*;
use std::net::SocketAddr;

fn main() {
    let res = bencoder::decode(&file_reader::read_file("./test_data/sample.torrent")[..]);

    let info = get_info(&res);
    let hash = hash_info(&info);
    println!("Percent encoded info key: {:}", hash);

    let announce_url = get_announce_url(&res);
    println!("Announce url: {}", announce_url);

    let mut response = requester::get(&announce_url, &hash);
    let mut response_content = Vec::new();
    response.read_to_end(&mut response_content).expect("HORROR");
    let bencoded_response = bencoder::decode(&response_content[..]);

    let peers = get_peers(&bencoded_response);
    let ip_addresses = collect_peers_info(&peers);

    println!("List of peers:");
    for addr in ip_addresses {
        println!("{}", addr);
    }
}

fn get_info(data: &bencoder::DataType) -> std::vec::Vec<u8> {
    bencoder::encode(data.get_dict_value(&b"info".to_vec()).unwrap())
}

fn hash_info(info: &[u8]) -> std::string::String {
    let mut hasher = Sha1::new();
    hasher.input(&info);
    let mut out = [0u8; 20];
    hasher.result(&mut out);
    let mut res = String::new();

    for byte in &out {
        res.push_str(percent_encoding::percent_encode_byte(*byte));
    }

    res
}

fn get_announce_url(data: &bencoder::DataType) -> std::string::String {
    let encoded_announce_url = data
        .get_dict_value(&b"announce".to_vec())
        .unwrap()
        .get_string_value()
        .unwrap();

    String::from_utf8(encoded_announce_url.to_vec()).unwrap()
}

fn get_peers(data: &bencoder::DataType) -> Vec<bencoder::DataType> {
    match data.get_dict_value(&b"peers".to_vec()).unwrap() {
        bencoder::DataType::List(value) => value.to_vec(),
        bencoder::DataType::ByteString(value) => convert_string_to_list(value),
        _ => vec![bencoder::DataType::Integer(0)], //TODO: handle it properly
    }
}

fn convert_string_to_list(value: &[u8]) -> Vec<bencoder::DataType> {
    let mut ips = Vec::<bencoder::DataType>::new();

    for chunk in value.chunks(6) {
        let ip = format!(
            "{}.{}.{}.{}",
            i32::from_be_bytes([0, 0, 0, chunk[0]]),
            i32::from_be_bytes([0, 0, 0, chunk[1]]),
            i32::from_be_bytes([0, 0, 0, chunk[2]]),
            i32::from_be_bytes([0, 0, 0, chunk[3]]),
        );

        let port = i32::from_be_bytes([0, 0, chunk[4], chunk[5]]);
        let mut peer_info = std::collections::HashMap::new();

        peer_info.insert(
            b"ip".to_vec(),
            bencoder::DataType::ByteString(ip.as_bytes().to_vec()),
        );
        peer_info.insert(
            b"port".to_vec(),
            bencoder::DataType::Integer(port as i64),
        );

        ips.push(bencoder::DataType::Dictionary(peer_info));
    }

    ips
}

fn collect_peers_info(peers: &[bencoder::DataType]) -> Vec<SocketAddr> {
    let mut addresses: Vec<SocketAddr> = Vec::new();

    for peer in peers {
        let ip = String::from_utf8(
            peer.get_dict_value(&b"ip".to_vec())
                .unwrap()
                .get_string_value()
                .unwrap()
                .to_vec(),
        )
        .unwrap();

        let port = peer
            .get_dict_value(&b"port".to_vec())
            .unwrap()
            .get_integer_value()
            .unwrap();

        addresses.push(format!("{}:{}", ip, port).parse().unwrap());
    }

    addresses
}
