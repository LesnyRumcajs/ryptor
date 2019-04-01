use crate::crypt;
use base64::STANDARD;
use std::fs;
use std::path::Path;
base64_serde_type!(Base64Standard, STANDARD);

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    #[serde(with = "Base64Standard")]
    pub key: Vec<u8>,
    #[serde(with = "Base64Standard")]
    pub iv: Vec<u8>,
}

impl Secrets {
    /// Generates new secrets instance with random key and initialisation vector
    pub fn generate() -> Self {
        Secrets {
            key: Secrets::generate_random_block(),
            iv: Secrets::generate_random_block(),
        }
    }

    /// Loads the secrets from a file (deserializes them)
    pub fn load_from(filename: &Path) -> Result<Self, std::io::Error> {
        let data = fs::read(filename)?;
        Ok(serde_yaml::from_slice(&data).unwrap())
    }

    /// Save the secrets to a file (serializes them)
    pub fn save_to(&self, filename: &Path) -> Result<(), std::io::Error> {
        let secret = serde_yaml::to_string(&self).unwrap();

        fs::write(filename, secret)?;
        Ok(())
    }

    /// Generates block of random bytes
    fn generate_random_block() -> Vec<u8> {
        (0..crypt::AES_BLOCK_SIZE)
            .map(|_| rand::random::<u8>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypt::AES_BLOCK_SIZE;
    use std::io::Write;

    #[test]
    fn generate_random_different_blocks() {
        let block1 = Secrets::generate_random_block();
        let block2 = Secrets::generate_random_block();

        assert_ne!(block1, block2);
        assert_eq!(block1.len(), AES_BLOCK_SIZE);
        assert_eq!(block2.len(), AES_BLOCK_SIZE);
    }

    #[test]
    fn generate_random_secrets() {
        let secrets = (Secrets::generate(), Secrets::generate());

        assert_eq!(secrets.0.key.len(), AES_BLOCK_SIZE);
        assert_eq!(secrets.0.iv.len(), AES_BLOCK_SIZE);
        assert_ne!(secrets.0.key, secrets.1.key);
        assert_ne!(secrets.0.iv, secrets.1.iv);
    }

    #[test]
    fn from_file() {
        let mut secrets_file = tempfile::NamedTempFile::new().unwrap();
        write!(
            secrets_file,
            "---
key: hxltPZWlLGDfaJjGbC0E6w==
iv: 2cKt1xcHkbpXLSq0ShzZkg=="
        )
        .unwrap();

        let secrets = Secrets::load_from(secrets_file.path()).unwrap();

        let expected_key = base64::decode("hxltPZWlLGDfaJjGbC0E6w==").unwrap();
        let expected_iv = base64::decode("2cKt1xcHkbpXLSq0ShzZkg").unwrap();

        assert_eq!(secrets.key, expected_key);
        assert_eq!(secrets.iv, expected_iv);
    }

    #[test]
    fn to_file() {
        let key = base64::decode("hxltPZWlLGDfaJjGbC0E6w==").unwrap();
        let iv = base64::decode("2cKt1xcHkbpXLSq0ShzZkg").unwrap();

        let secrets = Secrets { key, iv };

        let temp_file = tempfile::NamedTempFile::new().unwrap();
        secrets.save_to(temp_file.path()).unwrap();

        let result = fs::read_to_string(temp_file.path()).unwrap();
        let expected = "---
key: hxltPZWlLGDfaJjGbC0E6w==
iv: 2cKt1xcHkbpXLSq0ShzZkg==";

        assert_eq!(result, expected);
    }
}
