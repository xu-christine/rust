use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Password([u8; 32]);

impl From<[u8; 32]> for Password {
    fn from(digest: [u8; 32]) -> Self {
        Password(digest)
    }
}

impl From<Vec<u8>> for Password {
    fn from(vec: Vec<u8>) -> Self {
        Password(vec.try_into().unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 32, v.len())
        }))
    }
}

impl From<Password> for Vec<u8> {
    fn from(password: Password) -> Self {
        password.0.to_vec()
    }
}

impl Password {
    pub fn new_with_sha256(raw_password: String) -> Password {
        Sha256::digest(raw_password.as_bytes())
            .as_slice()
            .to_vec()
            .into()
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Password(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

fn main() {
    let plaintext = "123456@q".to_string();
    let password = Password::new_with_sha256(plaintext);
    //let pd1= String::from_utf8(pd.to_vec().clone()).expect("Found invalid UTF-8");
    //let pd2=String::from_utf8_lossy(&pd).to_string();
    println!("{:?}", password);
}
