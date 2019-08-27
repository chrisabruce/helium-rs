//! A module for using Helium's Crypto constructs.
//!
//! This is a 1 for 1 port from Helium's mobile app `crypto.js`.
//!
//! *NOTE:* it does _not_ appear that Helium's mnemonic implementation is
//! [bip39]:(https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
//! compatible.  bip39 uses the last word as a checksum and this implementation
//! does not seem to support the same checksum calc.
use base64;
use bs58;
use regex::Regex;
use sha2::{Digest, Sha256};
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{PublicKey, SecretKey, Seed, Signature};
use sodiumoxide::randombytes::randombytes;

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
    println!("Random: {:?}", rand16);
    entropy_to_mnemonic(&rand16)
}

pub fn generate_keypair(mnemonic: &str) -> (PublicKey, SecretKey) {
    let mut entropy = mnemonic_to_entropy(mnemonic);

    let mut seed: Vec<u8> = Vec::new();
    seed.append(&mut entropy.clone());
    seed.append(&mut entropy);

    match Seed::from_slice(&seed) {
        Some(s) => sign::keypair_from_seed(&s),
        _ => panic!("keypair from seed is None"),
    }
}

/// Turns an entropy into a word list ala bip39
fn entropy_to_mnemonic(entropy: &Vec<u8>) -> String {
    assert!(!(entropy.len() < 16), "invalid entropy, less than 16");
    assert!(!(entropy.len() > 32), "invalid entropy, greater than 32");
    assert!(entropy.len() % 4 == 0, "invalid entropy, less than 16");

    let entropy_bits = bytes_to_binary(entropy);
    let checksum_bits = derive_checksum_bits(entropy);
    let bits = format!("{}{}", entropy_bits, checksum_bits);

    lazy_static! {
        static ref RE_BITS: Regex = Regex::new("(.{1,11})").unwrap();
    }

    let wordlist_en = get_wordlist_en();

    // This can be more efficiently handled with a single iter,
    // but want to stay consistent with mobile app.
    let chunks: Vec<String> = RE_BITS
        .find_iter(&bits)
        .map(|m| m.as_str().to_string())
        .collect();

    let words: Vec<String> = chunks
        .iter()
        .map(|binary| wordlist_en[binary_to_bytes(binary)].clone())
        .collect();

    words.join(" ")
}

/// Converts a mnemonic to entropy.
fn mnemonic_to_entropy(mnemonic: &str) -> Vec<u8> {
    let words: Vec<&str> = mnemonic.split(" ").collect();

    // TODO: Static This
    let wordlist_en = get_wordlist_en();

    let bits: String = words
        .iter()
        .map(|w| match wordlist_en.iter().position(|s| s == w) {
            Some(idx) => format!("{:011b}", idx),
            _ => panic!("mnemonic word not found in wordlist."),
        })
        .collect::<Vec<String>>()
        .join("");

    let divider_index: usize = ((bits.len() as f64 / 33.0) * 32.0).floor() as usize;
    let (entropy_bits, checksum_bits) = bits.split_at(divider_index);

    lazy_static! {
        static ref RE_BYTES: Regex = Regex::new("(.{1,8})").unwrap();
    }

    let entropy_bytes: Vec<u8> = RE_BYTES
        .find_iter(&entropy_bits)
        .map(|m| binary_to_bytes(m.as_str()) as u8)
        .collect();

    assert!(!(entropy_bytes.len() < 16), "invalid checksum");
    assert!(!(entropy_bytes.len() > 32), "invalid checksum");
    assert!(entropy_bytes.len() % 4 == 0, "invalid checksum");

    let new_checksum = derive_checksum_bits(&entropy_bytes);

    assert!(checksum_bits == new_checksum, "invalid checksum");

    entropy_bytes
}

/// Takes a PublicKey and converts it to an address.
pub fn pubkey_to_address(pk: PublicKey) -> String {
    let mut address = vec![1];
    //address.append(pk.pub);

    unimplemented!()
}

/// Converts a vec of bytes into a single binary number string.
fn bytes_to_binary(bytes: &Vec<u8>) -> String {
    bytes
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join("")
}

/// Converts a binary string into an integer
fn binary_to_bytes(bin: &str) -> usize {
    let res: usize = usize::from_str_radix(bin, 2).unwrap();
    res
}

/// Calculates checksum bits for entropy and returns
/// a single binary number string.
fn derive_checksum_bits(entropy: &Vec<u8>) -> String {
    let ent = entropy.len() * 8;
    let cs = ent / 32;

    let mut hasher = Sha256::new();
    hasher.input(entropy);
    let hash = hasher.result();

    bytes_to_binary(&hash.as_slice().to_vec())[..cs].to_string()
}

/// returns an english wordlist for mnemonic
/// currently, isn't static (which is slightly better for mem)
fn get_wordlist_en() -> Vec<String> {
    // TODO: Static This
    let wl: Vec<String> = include_str!("wordlists/english.txt")
        .split_whitespace()
        .map(|w| w.to_string())
        .collect();
    debug_assert!(wl.len() == 2048, "Invalid wordlist length");

    wl
}

#[cfg(test)]
mod should {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn generate_mnemonic_twelve_words() {
        let words = generate_mnemonic();
        println!("{:?}", words);
        let list: Vec<&str> = words.split(" ").collect();
        assert_eq!(list.len(), 12);
    }

    #[test]
    fn convert_mnemonic_to_words() {
        let valid_entropy = vec![
            206, 74, 172, 150, 62, 114, 97, 231, 56, 126, 145, 227, 72, 121, 66, 212,
        ];
        let mnemonic =
            "soft fever cereal language champion vicious tiger split today duck expose prepare"
                .to_string();
        let entropy = mnemonic_to_entropy(&mnemonic);

        assert_eq!(entropy, valid_entropy);
    }

    #[test]
    fn correctly_generate_keypair() {
        let mnemonic =
            "soft fever cereal language champion vicious tiger split today duck expose prepare";
        let (pk, sk) = generate_keypair(&mnemonic);

        println!("{:?}", pk);
    }

    #[test]
    fn correctly_convert_bytes_to_binary() {
        let entropy = vec![
            0x49, 0xd9, 0x5b, 0xc5, 0xcd, 0x16, 0x0b, 0xb7, 0x7f, 0xa8, 0x04, 0x04, 0x2c, 0xbb,
            0x75, 0x95,
        ];
        let test_val = "01001001110110010101101111000101110011010001011000001011101101110111111110101000000001000000010000101100101110110111010110010101";
        assert_eq!(bytes_to_binary(&entropy), test_val);
    }

    #[test]
    fn correctly_convert_binary_to_bytes() {
        let bins = vec![
            "01001001110",
            "11001010110",
            "11110001011",
            "10011010001",
            "01100000101",
            "11011011101",
            "11111110101",
            "00000000100",
            "00000100001",
            "01100101110",
            "11011101011",
            "00101010000",
        ];

        let test_vals = vec![
            590, 1622, 1931, 1233, 773, 1757, 2037, 4, 33, 814, 1771, 336,
        ];

        for (idx, bin) in bins.iter().enumerate() {
            assert_eq!(binary_to_bytes(bin), test_vals[idx]);
        }
    }

    #[test]
    fn create_correct_checksum_bits() {
        let entropy = vec![
            0x49, 0xd9, 0x5b, 0xc5, 0xcd, 0x16, 0x0b, 0xb7, 0x7f, 0xa8, 0x04, 0x04, 0x2c, 0xbb,
            0x75, 0x95,
        ];
        let test_val = "1011";
        let checksum_bits = derive_checksum_bits(&entropy);
        assert_eq!(checksum_bits, test_val);
    }
}
