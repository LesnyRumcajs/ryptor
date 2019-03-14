use crate::crypto_utils;
use rand::Rng;
use std::fs;

const AES_BLOCK_SIZE: usize = 16;

pub struct Encryptor {
    key: [u8; AES_BLOCK_SIZE],
}

impl Encryptor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Encryptor {
        Encryptor {
            key: Encryptor::generate_key(),
        }
    }

    fn generate_key() -> [u8; AES_BLOCK_SIZE] {
        rand::thread_rng().gen::<[u8; AES_BLOCK_SIZE]>()
    }

    fn encrypt(&self, path: &str) -> Result<(), std::io::Error> {
        unimplemented!()
    }

    /// Saves the current key to file
    pub fn save_key(&self) -> Result<String, std::io::Error> {
        let hash = crypto_utils::sha256_hex(&self.key)[0..=6].to_owned();

        let encoded_key = base64::encode(&self.key);
        fs::write(&hash, encoded_key)?;

        Ok(hash)
    }
}

pub struct Decryptor {
    key: [u8; AES_BLOCK_SIZE],
}

impl Decryptor {
    fn from_file(path_to_key: &str) -> Result<Decryptor, std::io::Error> {
        let key = fs::read(&path_to_key)?;
        let key = base64::decode(&key).unwrap();
        
        if key.len() != AES_BLOCK_SIZE {
            return Err("Failed");
        }

        Ok(Decryptor {key})
    }

    fn decrypt(&self, path: &str) -> Result<(), std::io::Error> {
        unimplemented!();
    }
}
