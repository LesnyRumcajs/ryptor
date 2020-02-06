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
    /// Creates new Encryptor instance with random secrets
    #[allow(clippy::new_without_default)]
    pub fn new() -> Encryptor {
        trace!("Creating encryptor with random key");
        Encryptor {
            secrets: Secrets::generate(),
        }
    }

    /// Creates new Encryptor from secrets file
    pub fn from_file(path_to_secret: &Path) -> Result<Encryptor, std::io::Error> {
        trace!(
            "Creating encryptor with key from: {}",
            path_to_secret.to_str().unwrap()
        );

        Ok(Encryptor {
            secrets: Secrets::load_from(path_to_secret)?,
        })
    }

    /// Encrypts the target file under secrets
    pub fn encrypt(&self, path: &Path) -> Result<(), std::io::Error> {
        info!("Encrypting: {}", path.to_str().unwrap());
        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let ciphertext = cipher.encrypt_vec(&data);

        fs::write(path, ciphertext)?;

        Ok(())
    }

    /// Saves the currents secrets to a file
    pub fn save_secrets(&self, filename: &Path) -> Result<(), std::io::Error> {
        info!("Saving secrets to `{}`", filename.to_str().unwrap());
        self.secrets.save_to(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_secrets_file() -> NamedTempFile {
        let mut secrets_file = tempfile::NamedTempFile::new().unwrap();
        write!(
            secrets_file,
            "---
key: hxltPZWlLGDfaJjGbC0E6w==
iv: 2cKt1xcHkbpXLSq0ShzZkg=="
        )
        .unwrap();

        secrets_file
    }

    #[test]
    fn create_new_encryptor() {
        let encryptor = Encryptor::new();
        assert!(!encryptor.secrets.key.is_empty());
        assert!(!encryptor.secrets.iv.is_empty());
    }

    #[test]
    fn create_encryptor_from_file() {
        let secrets_file = create_secrets_file();
        let encryptor = Encryptor::from_file(secrets_file.path()).unwrap();
        assert!(!encryptor.secrets.key.is_empty());
        assert!(!encryptor.secrets.iv.is_empty());
    }

    #[test]
    fn encrypt() {
        let secrets_file = create_secrets_file();
        let encryptor = Encryptor::from_file(secrets_file.path()).unwrap();

        let target_file = tempfile::NamedTempFile::new().unwrap();
        assert!(write!(&target_file, "test").is_ok());

        assert!(encryptor.encrypt(target_file.path()).is_ok());

        let expected_ciphertext = hex::decode("526bf0d5fc310ae0bff151650b7a3ae8").unwrap();
        assert_eq!(fs::read(target_file).unwrap(), expected_ciphertext);
    }
}
