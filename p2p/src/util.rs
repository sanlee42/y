use crate::error::Error;
use crate::consts;

use base64;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::mem::transmute;

pub fn process_msg(msg: &String) -> Result<(), Error> {
    let data: Vec<_> = msg.split(",").collect();
    // TODO: split error
    let (nonce, body) = (data[0].parse::<i32>()?, data[1].to_string());
    let body = String::from_utf8(
        base64::decode_config(&body, base64::URL_SAFE)?).unwrap();

    let mut hasher = Sha256::new();
    hasher.input_str(&body);
    let hex = hasher.result_str();
    println!("nonce is {}, body is {:?}, hex:{}", nonce, body, hex);
    Ok(())
}

pub fn u32_to_vec(input: u32) -> Vec<u8> {
    let bytes: [u8; 4] = unsafe { transmute(input.to_be()) };
    bytes.to_vec()
}

pub fn encode_msg(nonce: u32, mut body: String) -> Vec<u8> {
    let mut msg = u32_to_vec(nonce);
    msg.append(unsafe { body.as_mut_vec() });
    msg
}
