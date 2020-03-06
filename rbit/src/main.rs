mod file_reader;
mod peer_wire_protocol;

extern crate crypto;
extern crate percent_encoding;

use bencoder;
use requester;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

fn main() {
    let res = bencoder::decode(&file_reader::read_file("./test_data/sample.torrent")[..]);

    let info = get_info(&res);
    let hash = hash_info(&info);
    let percent_encoded_hash = percent_encode(&hash[..]);
    println!("Percent encoded info key: {:}", percent_encoded_hash);

    let announce_url = get_announce_url(&res);
    println!("Announce url: {}", announce_url);

    let mut response = requester::get(&announce_url, &percent_encoded_hash);
    let mut response_content = Vec::new();
    response.read_to_end(&mut response_content).expect("HORROR");
    let bencoded_response = bencoder::decode(&response_content[..]);

    let peers = get_peers(&bencoded_response);
    let ip_addresses = collect_peers_info(&peers);

    println!("List of peers:");
    for addr in &ip_addresses {
        println!("{}", addr);
    }

    download_file(&ip_addresses, &hash, &res.get_dict_value(b"info").unwrap());
}

fn get_info(data: &bencoder::DataType) -> std::vec::Vec<u8> {
    bencoder::encode(data.get_dict_value(&b"info".to_vec()).unwrap())
}

fn hash_info(info: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.input(&info);
    let mut out = [0u8; 20];
    hasher.result(&mut out);
    out
}

fn percent_encode(data: &[u8]) -> String {
    let mut res = String::new();

    for byte in data {
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
        peer_info.insert(b"port".to_vec(), bencoder::DataType::Integer(port as i64));

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

fn download_file(ip_addresses: &[SocketAddr], hash: &[u8], meta_info: &bencoder::DataType) {
    println!("Start file download");
    let socket = ip_addresses.get(0).unwrap();
    println!("Start connection to {:?}", socket);
    let mut stream = TcpStream::connect(socket).unwrap();

    println!("Start hand shake");
    hand_shake(&mut stream, &hash);
    println!("End hand shake");

    send_interested(&mut stream);
    let mut i = 0;

    loop {
        println!("Start read reply");

        match read_reply(&mut stream) {
            Some(1) => {
                println!("Start block request");

                println!("Read piece length");
                let piece_length = meta_info
                    .get_dict_value(b"piece length")
                    .unwrap()
                    .get_integer_value()
                    .unwrap();

                println!("Read pieces");
                let pieces = meta_info
                    .get_dict_value(b"pieces")
                    .unwrap()
                    .get_string_value()
                    .unwrap();

                println!("Read name");
                let name = meta_info.get_dict_value(b"name").unwrap();

                println!("Read length");
                let length = meta_info
                    .get_dict_value(b"length")
                    .unwrap()
                    .get_integer_value()
                    .unwrap();

                println!("Info piece length: {}", piece_length);
                println!("Info pieces: {:?}", pieces);
                println!("Info name: {}", name);
                println!("Info length: {}", length);

                send_request(&mut stream, 0, 0, 16384)
            }
            Some(5) => println!("Thanks for bitfield"),
            Some(7) => println!("Thanks for piece"),
            Some(value) => println!("Received unprocessible id: {}", value),
            None => println!("Thanks for alive"),
        }

        i += 1;

        if i == 10 {
            break;
        } else {
            println!("Iteration {}", i);
        }
    }
}

fn hand_shake(stream: &mut TcpStream, hash: &[u8]) {
    let mut bytes: Vec<u8> = Vec::new();

    bytes.push(0b0001_0011);
    bytes.extend(b"BitTorrent protocol");
    bytes.extend(&[0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend(hash);
    bytes.extend(b"Rbit-Sn5J5VGM5CkFccE");

    stream.write_all(&bytes).unwrap();

    let length_prefix = read_len_prefix(stream);
    println!("Length prefix: {}", length_prefix);

    let pstr = String::from_utf8(read_bytes(stream, length_prefix)).unwrap();
    println!("pstr: {}", pstr);

    let reserved = read_bytes(stream, 8);
    println!("reserved: {:?}", reserved);

    let info_hash = read_bytes(stream, 20);
    println!("info hash: {}", info_hash == hash);

    let buf = read_bytes(stream, 20);
    let peer_id = String::from_utf8_lossy(&buf[..]);
    println!("peer id: {}", peer_id);
}

fn read_len_prefix(stream: &mut TcpStream) -> usize {
    *read_bytes(stream, 1).get(0).unwrap() as usize
}

fn read_bytes(stream: &mut TcpStream, number_of_bytes: usize) -> Vec<u8> {
    let mut buf = vec![0u8; number_of_bytes];
    stream.read_exact(&mut buf).unwrap();
    buf
}

fn send_interested(stream: &mut TcpStream) {
    println!("Send intrested");
    send_bytes(stream, &peer_wire_protocol::Msg::interested().pack());
}

fn send_request(stream: &mut TcpStream, index: u32, begin: u32, length: u32) {
    println!("Send request");
    send_bytes(
        stream,
        &peer_wire_protocol::Msg::request(index, begin, length).pack(),
    );
}

fn send_bytes(stream: &mut TcpStream, data: &[u8]) {
    stream.write_all(data).unwrap();
}

fn read_reply(stream: &mut TcpStream) -> Option<u8> {
    let reply = read_bytes(stream, 4);
    let prefix = i32::from_be_bytes([reply[0], reply[1], reply[2], reply[3]]);
    println!("prefix: {:?}", prefix);

    if prefix != 0 {
        let id = *read_bytes(stream, 1).get(0).unwrap() as u8;
        println!("Got id: {}", id);

        if id == 5 {
            let bitfield_len = (prefix - 1) as usize;
            let bitfield = read_bytes(stream, bitfield_len);
            println!("bitfield: {:?}", bitfield);
        }

        if id == 1 {
            println!("Unchoked!!1");
        }

        if id == 7 {
            let block_len = (prefix - 9) as usize;
            let payload = read_bytes(stream, block_len + 8);
            println!("Piece: {:?}", payload);
        }

        Some(id)
    } else {
        println!("Keep alive?");
        None
    }
}
