//! A module for using Helium's Crypto constructs.
//!
//! This is a 1 for 1 port from Helium's mobile app `crypto.js`.
//! 
//! *NOTE:* it does _not_ appear that Helium's mnemonic implementation is
//! [bip39]:(https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) 
//! compatible.  bip39 uses the last word as a checksum and this implementation
//! does not seem to support that.
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{Signature, PublicKey, SecretKey, Seed};
use sodiumoxide::randombytes::randombytes;
use base64;
use bs58::{decode};

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
    let entropyb64 = base64::encode(entropy);
    assert!(!(entropyb64.len() < 16), "invalid entropy, less than 16");
    assert!(!(entropyb64.len() > 32), "invalid entropy, greater than 32");
    assert!(entropyb64.len() % 4 == 0, "invalid entropy, less than 16");

    "".to_string()
}

#[cfg(test)]
mod should {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generate_mnemonic_without_panic() {
        assert_eq!(generate_mnemonic(), "".to_string());
    }
}
