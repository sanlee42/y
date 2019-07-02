use crate::error::Error;
use base64;
use crypto::sha2::Sha256;
use crypto::digest::Digest;


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