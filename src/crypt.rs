use crate::config::Secrets;
use crate::crypto_utils;

use std::fs;

const AES_BLOCK_SIZE: usize = 16;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use aes_soft::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

pub struct Encryptor {
    secrets: Secrets,
}

impl Encryptor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Encryptor {
        Encryptor {
            secrets: Secrets {
                key: Encryptor::generate_key(),
                iv: Encryptor::generate_key(),
            },
        }
    }

    fn generate_key() -> Vec<u8> {
        (0..AES_BLOCK_SIZE).map(|_| rand::random::<u8>()).collect()
    }

    pub fn encrypt(&self, path: &str) -> Result<(), std::io::Error> {
        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let ciphertext = cipher.encrypt_vec(&data);

        fs::write(path, ciphertext)?;

        Ok(())
    }

    pub fn save_key(&self) -> Result<String, std::io::Error> {
        let hash = crypto_utils::sha256_hex(&self.secrets.key)[0..=6].to_owned();

        let secret = serde_yaml::to_string(&self.secrets).unwrap();

        fs::write(&hash, secret)?;

        Ok(hash)
    }
}

pub struct Decryptor {
    secrets: Secrets,
}

impl Decryptor {
    pub fn from_file(path_to_key: &str) -> Result<Decryptor, std::io::Error> {
        let data = fs::read(path_to_key)?;
        Ok(Decryptor {
            secrets: serde_yaml::from_slice(&data).unwrap(),
        })
    }

    pub fn decrypt(&self, path: &str) -> Result<(), std::io::Error> {
        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let plaintext = cipher.decrypt_vec(&data).unwrap();

        fs::write(path, plaintext)?;

        Ok(())
    }
}
