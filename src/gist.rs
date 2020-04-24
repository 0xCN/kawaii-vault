pub use struct_::Config;
use rand::{Rng, thread_rng, distributions::Alphanumeric};
use aes_gcm::Aes256Gcm;
use aead::{Aead, NewAead, generic_array::GenericArray};


pub fn unique_gen() -> String {
    (0..12).map(|_| thread_rng().sample(Alphanumeric)).collect()
}

pub fn encrypt(password: &String, nonce_str: &String, message: &String) -> String {
    let key = GenericArray::clone_from_slice(&password.as_bytes());
    let aead = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&nonce_str.as_bytes());
    let dd3 = aead.encrypt(&nonce, message.as_ref()).expect("encryption failure!");
    let encrypted = base64::encode(&dd3);
    return encrypted;
}

pub fn decrypt(password: &String, nonce_str: &String, encrypted_msg: &String) -> String {
    let key = GenericArray::clone_from_slice(&password.as_bytes());
    let aead = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&nonce_str.as_bytes());
    let ciphertext = base64::decode(&encrypted_msg).unwrap();
    let plaintext = aead.decrypt(&nonce, ciphertext.as_ref()).expect("decryption failure!");
    let decrypted: String = String::from_utf8_lossy(&plaintext).into();
    return decrypted;
}

pub fn get_gist(conf: &Config) -> Vec<String> {
    let response = ureq::get(&format!("https://api.github.com/gists/{}", &conf.id))
        .set("Authorization", &format!("token {}", &conf.access_token))
        .call();
    let resp = response.into_json().unwrap();
    let uname: String = resp["owner"]["login"].to_string().replace("\"", "");
    let userid: String = resp["owner"]["id"].to_string().replace("\"", "");
    let unique: String = resp["files"][&conf.files.unique]["content"].to_string().replace("\"", "");
    let vaultd: String = resp["files"][&conf.files.vaultd]["content"].to_string().replace("\"", "");

    return vec![uname, userid, unique, vaultd];
}

pub fn update_gist(conf: &Config, unique: &String, content: &String) {
    ureq::patch(&format!("https://api.github.com/gists/{}", &conf.id))
        .set("Authorization", &format!("token {}", &conf.access_token))
        .send_json(ureq::json!({
            "files":{
                &conf.files.unique:{
                    "content": &unique
                },
                &conf.files.vaultd:{
                    "content": &content
                }

            }
        }));
}
