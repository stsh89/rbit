mod file_reader;

extern crate crypto;
extern crate percent_encoding;

use bencoder;
use requester;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::io::prelude::*;
use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4, TcpStream};

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
    // println!("Bencoded response {}", bencoded_response);

    let peers = get_peers(&bencoded_response);
    let ip_addresses = collect_peers_info(&peers);

    println!("List of peers:");
    for addr in ip_addresses {
        println!("{}", addr);
    }

    // match res {
    //     bencoder::DataType::Dictionary(dict) => {
    //         let mut hasher = Sha1::new();
    //
    //         let info = bencoder::encode(dict.get(&b"info".to_vec()).unwrap());
    //         hasher.input(&info);
    //
    //         let mut encoded_announce_url = &Vec::new();
    //
    //         if let bencoder::DataType::ByteString(value) = dict.get(&b"announce".to_vec()).unwrap()
    //         {
    //             encoded_announce_url = value;
    //         }
    //
    //         let announce_url = String::from_utf8(encoded_announce_url.to_vec()).unwrap();
    //         let mut out = [0u8; 20];
    //         hasher.result(&mut out);
    //         println!("{:?}", out);
    //         println!("Announce url: {}", announce_url);
    //         let hash_info =
    //             percent_encoding::percent_encode(&out, percent_encoding::DEFAULT_ENCODE_SET);
    //         println!("Percent encoded info key: {:}", hash_info);
    //
    //         let mut response = requester::get(&announce_url, &hash_info.to_string());
    //         // std::io::copy(&mut response, &mut std::io::stdout()).unwrap();
    //         // println!("\n\nDone.");
    //
    //         println!("***********************************");
    //         let mut response_content = Vec::new();
    //         response.read_to_end(&mut response_content).expect("HORROR");
    //         let bencoded_response = bencoder::decode(&response_content[..]);
    //         // println!("Bencoded response {}", bencoded_response);
    //
    //         // match bencoded_response {
    //         //     bencoder::DataType::Dictionary(dict) => {
    //         //         match dict.get(&b"peers".to_vec()).unwrap() {
    //         //             bencoder::DataType::ByteString(value) => {},
    //         //             bencoder::DataType::List(value) => {
    //                     // let mut ip = String::new();
    //
    //                     // for chunk in value[0].chunks(6) {
    //                     //     // println!("{}", i32::from_be(chunk[0] as i32));
    //                     //     // println!("{}", chunk[0]);
    //                     //     // println!("{}", chunk[0] as i32);
    //                     //     println!(
    //                     //         "{}.{}.{}.{}:{}",
    //                     //         i32::from_be_bytes([0, 0, 0, chunk[0]]),
    //                     //         i32::from_be_bytes([0, 0, 0, chunk[1]]),
    //                     //         i32::from_be_bytes([0, 0, 0, chunk[2]]),
    //                     //         i32::from_be_bytes([0, 0, 0, chunk[3]]),
    //                     //         i32::from_be_bytes([0, 0, chunk[4], chunk[5]])
    //                     //     )
    //                     // }
    //
    //                     // let socket = SocketAddrV4::new(Ipv4Addr::new(77, 122, 68, 143), 34961);
    //                     // // let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8888);
    //                     // println!("{:?}", socket);
    //                     // let mut stream = TcpStream::connect(socket).unwrap();
    //                     // let mut bytes: Vec<u8> = Vec::new();
    //                     //
    //                     // bytes.push(0b0001_0011);
    //                     // bytes.extend(b"BitTorrent protocol");
    //                     // bytes.extend(&[0, 0, 0, 0, 0, 0, 0, 0]);
    //                     // bytes.extend(&out);
    //                     // bytes.extend(b"-BOWxxx-yyyyyyyyyyyy");
    //                     //
    //                     // stream.write_all(&bytes).unwrap();
    //                     //
    //                     // let mut buf = [0; 4];
    //                     // stream.read_exact(&mut buf).unwrap();
    //                     // let length_prefix = i32::from_be_bytes(buf);
    //                     // println!("Length prefix: {}", length_prefix);
    //
    //                     // let mut response = Vec::new();
    //                     // for x in stream.bytes() {
    //                     //     println!("{}", x.unwrap());
    //                     // };
    //                     // println!("------------------------------------");
    //                     // println!("{:?}", response);
    //
    //                     // println!("{}", ip);
    //             //     }
    //             //     _ => {
    //             //         println!("{}", dict.get(&b"peers".to_vec()).unwrap());
    //             //         println!("Missing peers information");
    //             //     }
    //             // }
    //             // _ => {
    //             //     println!("Error");
    //             // }
    //         // }
    //     }
    //     _ => println!("Bencoding error"),
    // }
}

fn get_info(data: &bencoder::DataType) -> std::vec::Vec<u8> {
    bencoder::encode(data.get_dict_value(&b"info".to_vec()).unwrap())
}

fn hash_info(info: &[u8]) -> std::string::String {
    let mut hasher = Sha1::new();
    hasher.input(&info);
    let mut out = [0u8; 20];
    hasher.result(&mut out);
    percent_encoding::percent_encode(&out, percent_encoding::DEFAULT_ENCODE_SET).to_string()
}

fn get_announce_url(data: &bencoder::DataType) -> std::string::String {
    let encoded_announce_url =
        data
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
        _ => vec!(bencoder::DataType::Integer(0)) //TODO: handle it properly
    }
}

fn convert_string_to_list(value: &Vec<u8>) -> Vec<bencoder::DataType> {
    let mut ips = Vec::<bencoder::DataType>::new();

    for chunk in value.chunks(6) {
        let ip =
            format!(
                "{}.{}.{}.{}",
                i32::from_be_bytes([0, 0, 0, chunk[0]]),
                i32::from_be_bytes([0, 0, 0, chunk[1]]),
                i32::from_be_bytes([0, 0, 0, chunk[2]]),
                i32::from_be_bytes([0, 0, 0, chunk[3]]),
            );

        let port = i32::from_be_bytes([0, 0, chunk[4], chunk[5]]);
        let mut peer_info = std::collections::HashMap::new();

        peer_info.insert("ip".as_bytes().to_vec(), bencoder::DataType::ByteString(ip.as_bytes().to_vec()));
        peer_info.insert("port".as_bytes().to_vec(), bencoder::DataType::Integer(port as i64));

        ips.push(bencoder::DataType::Dictionary(peer_info));
    }

    ips
}

fn collect_peers_info(peers: &Vec<bencoder::DataType>) -> Vec<SocketAddr>{
    let mut addresses: Vec<SocketAddr> = Vec::new();

    for peer in peers {
        let ip = String::from_utf8(
            peer
            .get_dict_value(&b"ip".to_vec())
            .unwrap()
            .get_string_value()
            .unwrap()
            .to_vec()
        ).unwrap();

        let port =
            peer
            .get_dict_value(&b"port".to_vec())
            .unwrap()
            .get_integer_value()
            .unwrap();

        addresses.push(format!("{}:{}", ip, port).parse().unwrap());
    }

    addresses
}
