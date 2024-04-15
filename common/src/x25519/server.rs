//
//  Copyright 2024 Ram Flux, LLC.
//



use anyhow::Ok;
use hkdf::Hkdf;
use rand_core::{RngCore, SeedableRng};
use sha2::Sha256;

pub fn server_generate(
    device_proof: &[u8],
    account_public_key: &[u8],
) -> anyhow::Result<(String, String)> {
    //to server in osRng
    let mut server_osrng = rand_core::OsRng;
    let salt_bytes = server_osrng.next_u64().to_be_bytes();

    let info = [device_proof, account_public_key].concat();

    let hkdf = Hkdf::<Sha256>::new(Some(&salt_bytes), &info);
    let mut okm = [0u8; 32];
    hkdf.expand(b"", &mut okm)
        .map_err(|e| anyhow::anyhow!("Failed to expand hkdf: {}", e))?;

    let rng = rand_chacha::ChaChaRng::from_seed(okm);
    let prikey = x25519_dalek::StaticSecret::random_from_rng(rng);
    let pubkey = x25519_dalek::PublicKey::from(&prikey);
    let prikey_hax = hex::encode(prikey.to_bytes());
    let pubkey_hex = hex::encode(pubkey.as_bytes());

    Ok((prikey_hax, pubkey_hex))
}

pub fn server_osrng() -> anyhow::Result<String> {
    let mut server_osrng = rand_core::OsRng;
    let random_number = server_osrng.next_u64().to_le_bytes();
    let osrng = hex::encode(random_number.to_vec());
    Ok(osrng)
}
