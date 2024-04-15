//
//  Copyright 2024 Ram Flux, LLC.
//
use hkdf::Hkdf;
use rand_core::SeedableRng;
use sha2::Sha256;

#[derive(Debug, Clone)]
pub struct Device;

impl Device {
    pub fn generate_device_key(
        device_id: &[u8],
        account_public_key: &[u8],
        server_osrng: String,
    ) -> anyhow::Result<(x25519_dalek::StaticSecret, x25519_dalek::PublicKey)> {
        let salt_bytes = hex::decode(server_osrng)?;
        let info = [device_id, account_public_key].concat();

        let hkdf = Hkdf::<Sha256>::new(Some(&salt_bytes), &info);
        let mut okm = [0u8; 32];
        hkdf.expand(b"", &mut okm)
            .map_err(|e| anyhow::anyhow!("Failed to expand hkdf: {}", e))?;

        let rng = rand_chacha::ChaChaRng::from_seed(okm);
        let prikey = x25519_dalek::StaticSecret::random_from_rng(rng);
        let pubkey = x25519_dalek::PublicKey::from(&prikey);

        Ok((prikey, pubkey))
    }
}
