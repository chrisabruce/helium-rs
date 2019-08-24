use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{PublicKey, SecretKey, Seed};
use bs58::{decode};

type Signature = Vec<u8>;
type Message = Vec<u8>;
type Address = Vec<u8>;


pub fn sign(msg: &Message, sk: &SecretKey) -> Signature {
    sign::sign_detached(msg, sk)
}

pub fn verify(msg: &Message, sig: &Signature, pk: &PublicKey) -> bool {
    sign::verify_detached(msg, sig, pk)
}
