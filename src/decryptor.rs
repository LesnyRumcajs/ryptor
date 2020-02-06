use crate::secrets::Secrets;

use aes_soft::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use std::fs;

use log::info;
use std::path::Path;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub struct Decryptor {
    /// Cryptographic secrets that were used to encrypt the data
    secrets: Secrets,
}

impl Decryptor {
    /// Creates decryptor object from a file containing serialized secrets
    pub fn from_file(path_to_secret: &Path) -> Result<Decryptor, std::io::Error> {
        let data = fs::read(path_to_secret)?;
        Ok(Decryptor {
            secrets: serde_yaml::from_slice(&data).unwrap(),
        })
    }

    /// Decrypts file in a given location
    pub fn decrypt(&self, path: &Path) -> Result<(), std::io::Error> {
        info!("Decrypting: {}", path.to_str().unwrap());

        let data = fs::read(path)?;

        let cipher = Aes128Cbc::new_var(&self.secrets.key, &self.secrets.iv).unwrap();
        let plaintext = cipher.decrypt_vec(&data).unwrap();

        fs::write(path, plaintext)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;

    fn create_decryptor() -> Decryptor {
        let mut secrets_file = tempfile::NamedTempFile::new().unwrap();
        write!(
            secrets_file,
            "---
key: hxltPZWlLGDfaJjGbC0E6w==
iv: 2cKt1xcHkbpXLSq0ShzZkg=="
        )
        .unwrap();

        Decryptor::from_file(secrets_file.path()).unwrap()
    }

    #[test]
    fn create_from_file() {
        let decryptor = create_decryptor();

        let expected_key = base64::decode("hxltPZWlLGDfaJjGbC0E6w==").unwrap();
        let expected_iv = base64::decode("2cKt1xcHkbpXLSq0ShzZkg").unwrap();

        assert_eq!(decryptor.secrets.key, expected_key);
        assert_eq!(decryptor.secrets.iv, expected_iv);
    }

    #[test]
    fn decrypt_file() {
        let mut ciphertext_file = tempfile::NamedTempFile::new().unwrap();
        ciphertext_file
            .write_all(&hex::decode("526bf0d5fc310ae0bff151650b7a3ae8").unwrap())
            .unwrap();

        let decryptor = create_decryptor();
        let result = decryptor.decrypt(ciphertext_file.path());

        assert!(result.is_ok());
        assert_eq!("test", fs::read_to_string(ciphertext_file.path()).unwrap());
    }

    #[test]
    #[should_panic]
    fn decrypt_fail() {
        let mut ciphertext_file = tempfile::NamedTempFile::new().unwrap();
        ciphertext_file
            .write_all(&hex::decode("526bf0d5fc312ae0bff151650b7a3ae8").unwrap())
            .unwrap();

        let decryptor = create_decryptor();
        decryptor.decrypt(ciphertext_file.path()).unwrap();
    }
}
