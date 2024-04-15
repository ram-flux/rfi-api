//
//  Copyright 2024 Ram Flux, LLC.
//

use secp256k1::ecdsa::Signature;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

pub fn sign_message(message: &[u8], secret_key: &SecretKey) -> anyhow::Result<Signature> {
    let digest = sha256::Hash::hash(message);
    let message = Message::from_digest(digest.to_byte_array());
    let secp = Secp256k1::new();
    let sig = secp.sign_ecdsa(&message, &secret_key);
    Ok(sig)
}

pub fn verify_signature(
    message: &[u8],
    signature: &Signature,
    public_key: &PublicKey,
) -> anyhow::Result<bool> {
    let digest = sha256::Hash::hash(message);
    let message = Message::from_digest(digest.to_byte_array());
    let secp = Secp256k1::new();
    let verify = secp.verify_ecdsa(&message, &signature, &public_key).is_ok();
    Ok(verify)
}

pub fn from_pubkey_hex(public_key_hex: &str) -> anyhow::Result<PublicKey> {
    let public_key_vec =
        hex::decode(public_key_hex).map_err(|e| anyhow::anyhow!("Failed to decode: {}", e))?;

    let public_key = PublicKey::from_slice(&public_key_vec)
        .map_err(|e| anyhow::anyhow!("PublicKey from_slice to failed: {}", e))?;
    Ok(public_key)
}

pub fn from_sig_hex(signature_hex: &str) -> anyhow::Result<Signature> {
    let signature_vec =
        hex::decode(signature_hex).map_err(|e| anyhow::anyhow!("Failed to decode: {}", e))?;
        
    let signature = Signature::from_der(&signature_vec)
        .map_err(|e| anyhow::anyhow!("Signature from_der to failed: {}", e))?;
    Ok(signature)
}
