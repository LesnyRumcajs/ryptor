type Aes128Cbc = Cbc<Aes128, Pkcs7>;

use crate::secrets::Secrets;

use aes_soft::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use std::fs;

use log::{info, trace};
use std::path::Path;

pub struct Encryptor {
    secrets: Secrets,
}

impl Encryptor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Encryptor {
        trace!("Creating encryptor with random key");
        Encryptor {
            secrets: Secrets::generate(),
        }
    }

    pub fn from_secret(path_to_secret: &Path) -> Result<Encryptor, std::io::Error> {
        trace!(
            "Creating encryptor with key from: {}",
            path_to_secret.to_str().unwrap()
        );

        Ok(Encryptor {
            secrets: Secrets::load_from(path_to_secret)?,
        })
    }

    pub fn encrypt(&self, path: &Path) -> Result<(), std::io::Error> {
        info!("Encrypting: {}", path.to_str().unwrap());
        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let ciphertext = cipher.encrypt_vec(&data);

        fs::write(path, ciphertext)?;

        Ok(())
    }

    pub fn save_key(&self, filename: &Path) -> Result<(), std::io::Error> {
        info!("Saving key to `{}`", filename.to_str().unwrap());
        self.secrets.save_to(filename)
    }
}
