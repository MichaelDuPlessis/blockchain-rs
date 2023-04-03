use elliptic_curve::rand_core::OsRng;
use k256::ecdsa::{SigningKey, VerifyingKey};

pub fn gen_key_pair() -> (SigningKey, VerifyingKey) {
    // Generate a random private key
    let mut rng = OsRng;
    let signing_key = SigningKey::random(&mut rng);
    let verifying_key = VerifyingKey::from(&signing_key);

    (signing_key, verifying_key)
}
