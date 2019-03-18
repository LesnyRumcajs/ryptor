use ryptor::crypt;
use std::path::Path;

#[test]
fn should_generate_random_key() {
    let key_file = "should_generate_random_key";

    let encryptor = crypt::Encryptor::new();
    encryptor.save_key(key_file).unwrap();

    assert!(Path::new(&key_file).exists());
    let data = std::fs::read_to_string(&key_file).unwrap();

    std::fs::remove_file(key_file).unwrap();

    assert!(data.contains("key:"));
    assert!(data.contains("iv:"));
}

#[test]
fn should_generate_different_keys() {
    let key_file1 = "should_generate_different_keys1";
    let encryptor = crypt::Encryptor::new();
    encryptor.save_key(key_file1).unwrap();

    let data1 = std::fs::read("should_generate_different_keys1").unwrap();
    std::fs::remove_file(key_file1).unwrap();

    let key_file2 = "should_generate_different_keys2";
    let encryptor = crypt::Encryptor::new();
    encryptor.save_key(key_file2).unwrap();

    let data2 = std::fs::read(key_file2).unwrap();
    std::fs::remove_file(key_file2).unwrap();

    assert_ne!(data1, data2);
}
