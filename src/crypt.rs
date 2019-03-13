use rand::Rng;

const AES_BLOCK_SIZE: usize = 16;

pub struct Encryptor {
    key: [u8; AES_BLOCK_SIZE]
}

impl Encryptor {
    pub fn new() -> Encryptor {
        Encryptor { key: Encryptor::generate_key()}
    }

    fn generate_key() -> [u8; AES_BLOCK_SIZE] {
        rand::thread_rng().gen::<[u8; AES_BLOCK_SIZE]>()
    }

    fn encrypt(&self, path: &str) -> Result<(), std::io::Error> {
        unimplemented!()
    }

    /// Saves the current key to file
    pub fn save_key() -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
