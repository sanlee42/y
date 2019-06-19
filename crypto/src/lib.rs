#[cfg(test)]
extern crate rand;
extern crate secp256k1;
extern crate ed25519_dalek;

mod tests {
    use secp256k1::{Secp256k1, Message};
    use rand::rngs::OsRng;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_secp256k1_sign() {
        let secp = Secp256k1::new();
        let msg = Message::from_slice(&[0xab; 32]).unwrap();
        match OsRng::new() {
            Ok(ref mut rng) => {
                let (sec_key, pub_key) = secp.generate_keypair(rng);
                let sig = secp.sign(&msg, &sec_key);
                assert!(secp.verify(&msg, &sig, &pub_key).is_ok());
            }
            Err(e) => panic!(e)
        }
    }

    use ed25519_dalek::Keypair;
    use ed25519_dalek::Signature;
    use sha2::Sha512;

    #[test]
    fn test_ed25519_sign() {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
        let message: &[u8] = b"This is the answer to the universe";
        let signature: Signature = keypair.sign::<Sha512>(message);
        assert!(keypair.verify::<Sha512>(message, &signature).is_ok());
    }
}
