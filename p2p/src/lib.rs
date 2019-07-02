#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::util;

    #[test]
    fn test_encode() {
        let nonce: u32 = 10;
        let mut body: String = "hello".to_string();
        let msg = util::encode_msg(nonce, &mut body);
        let (_nonce, _body) = util::decode_msg(&msg);
        assert_eq!(nonce, _nonce);
        assert_eq!(body, String::from_utf8_lossy(_body));
    }
}

mod conn;
mod peer;
mod peers;
mod io;
mod error;
mod consts;
pub mod util;
pub mod serv;
