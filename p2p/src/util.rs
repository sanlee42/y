use crate::error::Error;
use crate::consts;

use base64;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::mem::transmute;
use std::convert::TryInto;


pub fn process_msg(msg: &[u8]) -> Result<(), Error> {
    let (nonce, body) = decode_msg(msg);
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

pub fn encode_msg(nonce: u32, body: &mut String) -> Vec<u8> {
    let mut msg = u32_to_vec(nonce);
    msg.append(body.as_bytes().to_vec().as_mut());
    //msg.append(unsafe { body.as_mut_vec() });
    msg
}

pub fn decode_msg(msg: &[u8]) -> (u32, &[u8]) {
    let (mut nonce, body) = msg.split_at(4);

    let nonce = u32::from_be_bytes(nonce.try_into().unwrap());
    return (nonce, body);
}