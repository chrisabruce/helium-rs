use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{PublicKey, SecretKey, Seed};
use bs58::{decode};

type Signature = Vec<u8>;
type Message = Vec<u8>;
type Address = Vec<u8>;
