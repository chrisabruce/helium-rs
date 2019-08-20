use sodiumoxide::crypto::sign;
use bip39::{Mnemonic, MnemonicType, Language, Seed};

type Signature = Vec<u8>;
type Message = Vec<u8>;
type Address = Vec<u8>;

// Generates a list of Words used to create keypair.
pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).phrase().to_string()
}

#[cfg(test)]
mod should {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn successfully_generate_mnemonic() {
        let m = generate_mnemonic();
        println!("Mnemonic: {}", m);
        assert_ne!(m.len(), 0);
    }
}
