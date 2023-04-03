use elliptic_curve::rand_core::OsRng;
use k256::{
    ecdsa::{Signature, SigningKey, VerifyingKey},
    schnorr::signature::{DigestVerifier, Signer, Verifier},
    Secp256k1,
};
use sha2::Digest;

// pub fn gen_key_pair() -> (Vec<u8>, Vec<u8>) {
//     let mut rng = OsRng;
//     let secret_key: SecretKey<k256::Secp256k1> = SecretKey::random(&mut rng);
//     let public_key = secret_key.public_key();

//     let secret_key: Vec<u8> = secret_key.to_bytes().as_slice().into();
//     let public_key: Vec<u8> = public_key.to_public_key_der().unwrap().as_bytes().into();

//     (secret_key, public_key)
// }

// pub fn gen_key_pair() -> SecretKey<k256::Secp256k1> {
//     let mut rng = OsRng;
//     SecretKey::random(&mut rng)
// }

pub fn gen_key_pair() -> (SigningKey, VerifyingKey) {
    // Generate a random private key
    let mut rng = OsRng;
    let signing_key = SigningKey::random(&mut rng);
    let verifying_key = VerifyingKey::from(&signing_key);

    (signing_key, verifying_key)

    // Create a message to sign
    // let message = b"Hello, world!";

    // let signature: Signature = signing_key.sign(message);

    // verifying_key.verify(message, &signature).is_ok();

    // let signature_valid = public_key.verify_digest(hash.into(), &sig).is_ok();

    // // Hash the message
    // let hash = Sha256::digest(message);

    // // Sign the hash using the private key
    // let signature: K256Signature = signing_key.sign_digest(&hash);

    // // Convert the signature to a byte array
    // let signature_bytes = signature.serialize();

    // // Verify the signature using the public key
    // let public_key = signing_key.verifying_key();

    // let signature: Signature = DigestSignature::new(&Sha256::default(), signature_bytes).into();

    // let signature_valid = public_key.verify_digest(&hash, &signature).is_ok();
    // assert!(signature_valid);
}
