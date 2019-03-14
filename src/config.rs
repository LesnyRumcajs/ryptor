use base64::STANDARD;
base64_serde_type!(Base64Standard, STANDARD);

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    #[serde(with = "Base64Standard")]
    pub key: Vec<u8>,
    #[serde(with = "Base64Standard")]
    pub iv: Vec<u8>,
}
