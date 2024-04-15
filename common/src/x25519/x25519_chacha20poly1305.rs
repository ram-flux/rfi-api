//
//  Copyright 2024 Ram Flux, LLC.
//


#[warn(dead_code)]
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};

#[derive(Debug, Clone)]
pub struct Encrypt {
    pub privkey: String,
    pub to_pubkey: String,
    pub data: String,
    pub nonce: String,
}

impl Encrypt {
    pub fn new(privkey: String, to_pubkey: String, nonce: String, data: String) -> Self {
        Self {
            privkey,
            to_pubkey,
            data,
            nonce,
        }
    }

    fn pubkey_from_hex(&self, public_key_hex: &str) -> Option<x25519_dalek::PublicKey> {
        let public_key_bytes = match hex::decode(public_key_hex) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };
        let public_key_bytes: [u8; 32] = match public_key_bytes.try_into() {
            Ok(bytes) => bytes,
            Err(_) => return None,
        };
        Some(x25519_dalek::PublicKey::from(public_key_bytes))
    }

    fn prikey_from_hex(&self, private_key_hex: &str) -> Option<x25519_dalek::StaticSecret> {
        let private_key_bytes: [u8; 32] = hex::decode(private_key_hex)
            .expect("Invalid hex in private key")
            .try_into()
            .expect("Incorrect private key length");
        Some(x25519_dalek::StaticSecret::from(private_key_bytes))
    }

    pub fn encrypt(&self) -> anyhow::Result<String> {
        let prikey = match self.prikey_from_hex(&self.privkey) {
            Some(key) => key,
            None => anyhow::bail!("Invalid private key"),
        };
        let to_pubkey = match self.pubkey_from_hex(&self.to_pubkey) {
            Some(key) => key,
            None => anyhow::bail!("Invalid public key"),
        };

        let shared_secret = prikey.diffie_hellman(&to_pubkey);
        let key = Key::from_slice(shared_secret.as_bytes());
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(self.nonce.as_bytes());

        let plaintext = self.data.as_bytes();
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| anyhow::anyhow!("encrypt:{}", e))?;
        Ok(hex::encode(ciphertext))
    }

    pub fn decrypt(&self) -> anyhow::Result<String> {
        let prikey = match self.prikey_from_hex(&self.privkey) {
            Some(key) => key,
            None => anyhow::bail!("Invalid private key"),
        };
        let to_pubkey = match self.pubkey_from_hex(&self.to_pubkey) {
            Some(key) => key,
            None => anyhow::bail!("Invalid public key"),
        };

        let shared_secret = prikey.diffie_hellman(&to_pubkey);
        let key = Key::from_slice(shared_secret.as_bytes());
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(self.nonce.as_bytes());

        let ciphertext = hex::decode(&self.data)?;
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!("decrypt:{}", e))?;
        Ok(String::from_utf8(plaintext)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let server_pri_key = x25519_dalek::StaticSecret::random_from_rng(rand_core::OsRng);
        let server_public_key = x25519_dalek::PublicKey::from(&server_pri_key);
        let serde_public_key_hex = hex::encode(server_public_key.as_bytes());
        let serde_private_key_hex = hex::encode(server_pri_key.to_bytes());

        let client_private_key = x25519_dalek::StaticSecret::random_from_rng(rand_core::OsRng);
        let client_public_key = x25519_dalek::PublicKey::from(&client_private_key);
        let client_public_key_hex = hex::encode(client_public_key.as_bytes());
        let client_private_key_hex = hex::encode(client_private_key.to_bytes());
        let data = "hello world".to_string();
        let nonce = "unique nonce".to_string();

        let cli = Encrypt::new(
            client_private_key_hex,
            serde_public_key_hex,
            nonce.clone(),
            data.clone(),
        );
        let cli_encrypt = cli.encrypt().unwrap();

        let ser = Encrypt::new(
            serde_private_key_hex,
            client_public_key_hex,
            nonce.clone(),
            cli_encrypt,
        );
        let ser_decrypt = ser.decrypt().unwrap();
        println!("ser_decrypt:{}", ser_decrypt);
    }

    //check in flow
    // use chacha20poly1305::{
    //     aead::{Aead, KeyInit},
    //     ChaCha20Poly1305, Key, Nonce,
    // };

    // #[test]
    // fn test_imitate() {
    //     //server
    //     let server_pri_key = x25519_dalek::StaticSecret::random_from_rng(rand_core::OsRng);
    //     let server_public_key = x25519_dalek::PublicKey::from(&server_pri_key);
    //     //server_public_key
    //     let client_private_key = x25519_dalek::StaticSecret::random_from_rng(rand_core::OsRng);
    //     let client_public_key = x25519_dalek::PublicKey::from(&client_private_key);

    //     let ser_shared_secret = server_pri_key.diffie_hellman(&client_public_key);
    //     let cli_shared_secret = client_private_key.diffie_hellman(&server_public_key);

    //     //client encrypt
    //     let key = Key::from_slice(cli_shared_secret.as_bytes());
    //     let cipher = ChaCha20Poly1305::new(key);
    //     let nonce = Nonce::from_slice(b"unique nonce");

    //     let plaintext = b"plaintext messagesdfsdfsdfsdfsdf sdfsdfsdf";
    //     let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();

    //     //server decrypt
    //     let key = Key::from_slice(ser_shared_secret.as_bytes());
    //     let cipher = ChaCha20Poly1305::new(key);
    //     // let nonce = Nonce::from_slice(b"unique nonce");

    //     // decrypted
    //     let decrypted_plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    //     assert_eq!(&decrypted_plaintext[..], plaintext);
    // }
}
