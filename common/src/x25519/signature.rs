use generic_array::GenericArray;
use hmac::{Hmac, Mac};
use sha2::Sha256;
// use typenum::U32;

#[derive(Debug, Clone)]
pub struct Signature {
    privkey: String,
    to_pubkey: String,
}

type HmacSha256 = Hmac<Sha256>;

impl Signature {
    pub fn new(privkey: String, to_pubkey: String) -> Self {
        Self { privkey, to_pubkey }
    }

    fn prikey_from_hex(&self, private_key_hex: &str) -> anyhow::Result<x25519_dalek::StaticSecret> {
        let private_key_bytes = hex::decode(private_key_hex)
            .map_err(|_| anyhow::anyhow!("Invalid hex in private key"))?;
        let private_key_bytes: [u8; 32] = private_key_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("Incorrect private key length"))?;
        Ok(x25519_dalek::StaticSecret::from(private_key_bytes))
    }

    fn pubkey_from_hex(&self, public_key_hex: &str) -> anyhow::Result<x25519_dalek::PublicKey> {
        let public_key_bytes = hex::decode(public_key_hex)
            .map_err(|_| anyhow::anyhow!("Invalid hex in public key"))?;
        let public_key_bytes: [u8; 32] = public_key_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("Incorrect public key length"))?;
        Ok(x25519_dalek::PublicKey::from(public_key_bytes))
    }

    fn get_pri_pubkey(
        &self,
    ) -> anyhow::Result<(x25519_dalek::StaticSecret, x25519_dalek::PublicKey)> {
        let prikey: x25519_dalek::StaticSecret = self.prikey_from_hex(&self.privkey)?;
        let to_pubkey: x25519_dalek::PublicKey = self.pubkey_from_hex(&self.to_pubkey)?;
        Ok((prikey, to_pubkey))
    }

    pub fn verify(&self, message: &str, signature: &str) -> anyhow::Result<bool> {
        let signature =
            hex::decode(signature).map_err(|e| anyhow::anyhow!("Invalid hex in message: {}", e))?;

        let (prikey, to_pubkey) = self.get_pri_pubkey()?;
        let shared_secret = prikey.diffie_hellman(&to_pubkey);

        let mut mac = HmacSha256::new_from_slice(shared_secret.as_bytes())
            .map_err(|e| anyhow::anyhow!("HMAC can take key of any size: {}", e))?; // Create HMAC-SHA256 instance

        mac.update(message.as_bytes());

        let signature_bytes = GenericArray::from_slice(&signature);

        mac.verify(signature_bytes)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }

    pub fn sign(&self, message: &str) -> anyhow::Result<String> {
        let (prikey, to_pubkey) = self.get_pri_pubkey()?;
        let shared_secret = prikey.diffie_hellman(&to_pubkey);
        let mut mac = HmacSha256::new_from_slice(shared_secret.as_bytes())
            .map_err(|e| anyhow::anyhow!("HMAC can take key of any size: {}", e))?;

        mac.update(message.as_bytes());

        let result = mac.finalize().into_bytes();
        Ok(hex::encode(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() -> Result<(), anyhow::Error> {
        let ser_sign = Signature::new(
            "58f13e43e096bf261e02d0db27e72f0920f13736586e204acba3aec377ee4020".to_string(),
            "edb4fe7a1d5aae37dfc37b9e575fcd36922a02a0e3ffcb79ba85268a5fb5b120".to_string(),
        );
        let cli_sign = Signature::new(
            "3447a0b8093193b7a0330b632c8d4e7566ebdefcfcaf44529a97d3be5d89615a".to_string(),
            "2df994be6eb07a574a1a859032e6d395afc1de14dbac5c235ac3d153c868001b".to_string(),
        );

        let message = "Hello, world!";
        let cli_signs = cli_sign.sign(message)?;
        assert!(ser_sign.verify(message, &cli_signs)?);

        let msg = "abcsssss";
        let ser_sign = ser_sign.sign(msg)?;
        // println!("ser_sign: {}", ser_sign);
        let cliss = cli_sign.verify(msg, &ser_sign)?;
        println!("cliss:{}", cliss);
        // assert!(cli_sign.verify(msg, &ser_sign)?);

        Ok(())
    }
}
