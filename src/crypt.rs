use crate::config::Secrets;
use std::fs;

const AES_BLOCK_SIZE: usize = 16;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use aes_soft::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use log::{info, trace};

pub struct Encryptor {
    secrets: Secrets,
}

impl Encryptor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Encryptor {
        trace!("Creating encryptor with random key");
        Encryptor {
            secrets: Secrets {
                key: Encryptor::generate_key(),
                iv: Encryptor::generate_key(),
            },
        }
    }

    pub fn from_secret(path_to_secret: &str) -> Result<Encryptor, std::io::Error> {
        trace!("Creating encryptor with key from: {}", path_to_secret);
        let data = fs::read(path_to_secret)?;
        Ok(Encryptor {
            secrets: serde_yaml::from_slice(&data).unwrap(),
        })
    }

    fn generate_key() -> Vec<u8> {
        (0..AES_BLOCK_SIZE).map(|_| rand::random::<u8>()).collect()
    }

    pub fn encrypt(&self, path: &str) -> Result<(), std::io::Error> {
        info!("Encrypting: {}", path);
        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let ciphertext = cipher.encrypt_vec(&data);

        fs::write(path, ciphertext)?;

        Ok(())
    }

    pub fn save_key(&self, filename: &str) -> Result<(), std::io::Error> {
        info!("Saving key to `{}`", filename);
        let secret = serde_yaml::to_string(&self.secrets).unwrap();

        fs::write(filename, secret)?;
        Ok(())
    }
}

pub struct Decryptor {
    secrets: Secrets,
}

impl Decryptor {
    pub fn from_file(path_to_secret: &str) -> Result<Decryptor, std::io::Error> {
        let data = fs::read(path_to_secret)?;
        Ok(Decryptor {
            secrets: serde_yaml::from_slice(&data).unwrap(),
        })
    }

    pub fn decrypt(&self, path: &str) -> Result<(), std::io::Error> {
        info!("Decrypting: {}", path);
        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let plaintext = cipher.decrypt_vec(&data).unwrap();

        fs::write(path, plaintext)?;

        Ok(())
    }
}
