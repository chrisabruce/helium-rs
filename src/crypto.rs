//! A module for using Helium's Crypto constructs.
//!
//! This is a 1 for 1 port from Helium's mobile app `crypto.js`.
//! 
//! *NOTE:* it does _not_ appear that Helium's mnemonic implementation is
//! [bip39]:(https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) 
//! compatible.  bip39 uses the last word as a checksum and this implementation
//! does not seem to support the same checksum calc.
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{Signature, PublicKey, SecretKey, Seed};
use sodiumoxide::randombytes::randombytes;
use bs58::{decode};
use sha2::{Sha256, Digest};

type Message = Vec<u8>;
type Address = Vec<u8>;


pub fn sign(msg: &Message, sk: &SecretKey) -> Signature {
    sign::sign_detached(msg, sk)
}

pub fn verify(sig: &Signature, msg: &Message, pk: &PublicKey) -> bool {
    sign::verify_detached(sig, msg, pk)
}

pub fn generate_mnemonic() -> String {
    let rand16 = randombytes(16);
    entropy_to_mnemonic(&rand16)

}

pub fn generate_keypair(mnemonic: &str) -> (PublicKey, SecretKey) {
    unimplemented!()
}

fn entropy_to_mnemonic(entropy: &Vec<u8>) -> String {
    assert!(!(entropy.len() < 16), "invalid entropy, less than 16");
    assert!(!(entropy.len() > 32), "invalid entropy, greater than 32");
    assert!(entropy.len() % 4 == 0, "invalid entropy, less than 16");

    let entropy_bits = bytes_to_binary(entropy);
    let checksum_bits = derive_checksum_bits(entropy);
    let bits = format!("{}{}", entropy_bits, checksum_bits);

    println!("{:?}", entropy_bits);

    "".to_string()
}

/// Calculates checksum bits for entropy and returns
/// a single binary number string.
fn derive_checksum_bits(entropy: &Vec<u8>) -> String {
    let ent = entropy.len() * 8;
    let cs = ent / 32;

    let mut hasher = Sha256::new();
    hasher.input(entropy);
    let hash = hasher.result();

    bytes_to_binary(&hash.as_slice().to_vec())[0..cs].to_string()
}

/// Converts a vec of bytes into a single binary number string.
fn bytes_to_binary(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|b| {
        format!("{:08b}", b)
    }).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod should {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generate_mnemonic_without_panic() {
        assert_eq!(generate_mnemonic(), "".to_string());
    }

    #[test]
    fn correctly_convert_bytes_to_binary() {
        let entropy = vec!(0x49, 0xd9, 0x5b, 0xc5, 0xcd, 0x16, 0x0b, 0xb7, 0x7f, 0xa8, 0x04, 0x04, 0x2c, 0xbb, 0x75, 0x95);
        let test_val = "01001001110110010101101111000101110011010001011000001011101101110111111110101000000001000000010000101100101110110111010110010101";
        assert_eq!(bytes_to_binary(&entropy), test_val);
    }

    #[test]
    fn create_correct_checksum_bits() {
        let entropy = vec!(0x49, 0xd9, 0x5b, 0xc5, 0xcd, 0x16, 0x0b, 0xb7, 0x7f, 0xa8, 0x04, 0x04, 0x2c, 0xbb, 0x75, 0x95);
        let test_val = "1011";
        let checksum_bits = derive_checksum_bits(&entropy);
        assert_eq!(checksum_bits, test_val);
        
    }
}
