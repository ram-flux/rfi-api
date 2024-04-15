//
//  Copyright 2024 Ram Flux, LLC.
//



pub mod server;
pub mod x25519_chacha20poly1305;
pub mod device;



pub fn verify_keys(private_key_hex: &str, public_key_hex: &str) -> bool {
    let private_key_bytes: [u8; 32] = hex::decode(private_key_hex)
        .expect("Invalid hex in private key")
        .try_into()
        .expect("Incorrect private key length");
    let private_key = x25519_dalek::StaticSecret::from(private_key_bytes);

    let public_key = x25519_dalek::PublicKey::from(&private_key);

    let regenerated_public_key_hex = hex::encode(public_key.as_bytes());
    public_key_hex == &regenerated_public_key_hex
}

pub fn pubkey_from_hex(public_key_hex: &str) -> Option<x25519_dalek::PublicKey> {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_verification() {
        let private_key_hex = "1023d991f1d32ceb56eaee081b1f2e47f45cad1bc8ed832cb0388c1feb417d67";
        let public_key_hex = "0f662cace821b3766c92c266a9f35c34ae68af3e3743f0fcc1497a9f128b9306";
        println!("public_key_hex: {:?}", pubkey_from_hex(&public_key_hex));
        assert!(verify_keys(private_key_hex, public_key_hex));
    }
}
