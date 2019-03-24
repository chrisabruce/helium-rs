use sodiumoxide::crypto::sign;

type Signature = [u8];
type Message = [u8];
type Address = [u8];

/// Returns address.
pub fn get_address() -> Address {
    // TODO: replace with file read
    "x".as_bytes()
}

/// Returns a Signature.
///
/// # Arguments
///
/// * `msg` - The message to sign.
pub fn sign(msg: &Message) -> Signature {
    // TODO: replace with secure key retrieval
    let (pk, sk) = sign::gen_keypair();
    sign::sign_detached(msg, &sk);
}

/// Verifies the Signature of signed message.
///
/// # Arguments
///
/// * `sig` - The Signature used.
/// * `msg` - The Message signed.
pub fn verify(sig: &Signature, msg: &Message) -> bool {
    // TODO: replace with secure key retrieval
    let (pk, sk) = sign::gen_keypair();
    sign::verify_detached(sig, msg, pk)
}
