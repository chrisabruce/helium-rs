use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{PublicKey, SecretKey, Seed};
use bip39::{Mnemonic, MnemonicType, Language};
use bs58::{decode};

type Signature = Vec<u8>;
type Message = Vec<u8>;
type Address = Vec<u8>;

/// Generates 12-word Mnemonic for Helium Wallet Construction.
pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).phrase().to_string()
}

pub fn generate_keypair(phrase: &str) -> (PublicKey, SecretKey) {
    let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    println!("Mnemonic: {:?}", mnemonic);
    let seed_bip = bip39::Seed::new(&mnemonic, "");
    

    let seed_bytes = seed_bip.as_bytes();
    println!("seed size: {:?}", seed_bytes.len());

    let seed = Seed::from_slice(&seed_bytes);

    match Seed::from_slice(&seed_bytes) {
        Some(s) => {
            return sign::keypair_from_seed(&s);
        },
        None => {
            panic!("seed from slice returned None");
        }

    }


    //sign::keypair_from_seed(&Seed::from_slice(&seed_bip.as_bytes()).unwrap())
}

#[cfg(test)]
mod should {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn be_a_valid_mnemonic() {

    //     assert!(Mnemonic::validate(test_mnemonic, Language::English).is_ok());
    // }

    #[test]
    fn successfully_generate_mnemonic() {
        let m = generate_mnemonic();
        println!("Mnemonic: {}", m);
        assert_ne!(m.len(), 0);
    }

    #[test]
    fn successfully_generate_keypair() {
        let m = generate_mnemonic();
        let (pk, sk) = generate_keypair(&m);
        assert_ne!(hex::encode(pk), hex::encode(sk));
    }

    // #[test]
    // fn generate_correct_key_from_phrase() {

    //     let test_key = decode("14a5sytsEVKBVhGsVz2rJDkxf1cs1L89zvtFjmPtmPzrQHXPVjF").into_vec().unwrap();
    //     println!("TEST_K: {:?}", test_key);

    //     let (pk, _sk) = generate_keypair(&phrase);
    //     println!("PUB_K: {:?}", pk);

    //     let known_pk = PublicKey::from_slice(&test_key[..]).unwrap();

    //     assert_eq!(pk, known_pk);
    // }
}
