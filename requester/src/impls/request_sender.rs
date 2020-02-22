extern crate reqwest;
use std::str;

pub fn get(url: &str, hash_info: &str) -> reqwest::blocking::Response {
    let mut url_with_params = String::new();

    url_with_params.push_str(url);
    url_with_params.push_str("?info_hash=");
    url_with_params.push_str(hash_info);
    url_with_params.push_str("&uploaded=0");
    url_with_params.push_str("&downloaded=0");
    url_with_params.push_str("&left=0");
    url_with_params.push_str("&port=6881");
    url_with_params.push_str("&peer_id=Rbit-Sn5J5VGM5CkFccE");

    println!("Starting request {}", url_with_params);
    reqwest::blocking::get(&url_with_params).unwrap()
}
