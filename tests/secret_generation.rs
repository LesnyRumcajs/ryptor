use ryptor::encryptor::Encryptor;

#[test]
fn should_generate_random_key() {
    let key_file = tempfile::NamedTempFile::new().unwrap();

    let encryptor = Encryptor::new();
    encryptor.save_secrets(key_file.path()).unwrap();

    assert!(key_file.path().exists());
    let data = std::fs::read_to_string(&key_file).unwrap();

    assert!(data.contains("key:"));
    assert!(data.contains("iv:"));
}

#[test]
fn should_generate_different_keys() {
    let key_file1 = tempfile::NamedTempFile::new().unwrap();
    let encryptor = Encryptor::new();

    assert!(encryptor.save_secrets(key_file1.path()).is_ok());

    let data1 = std::fs::read(key_file1.path()).unwrap();

    let key_file2 = tempfile::NamedTempFile::new().unwrap();
    let encryptor = Encryptor::new();
    assert!(encryptor.save_secrets(key_file2.path()).is_ok());

    let data2 = std::fs::read(key_file2.path()).unwrap();

    assert_ne!(data1, data2);
}
