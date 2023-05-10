use rsa::{pkcs8::{DecodePrivateKey}, Pkcs1v15Encrypt, RsaPrivateKey};
const PRIV_KEY: &str = include_str!("../private.pem");
fn main() {

    let private_key = RsaPrivateKey::from_pkcs8_pem(PRIV_KEY).unwrap();
    let pub_key = private_key.to_public_key();

    let data = "password@123".as_bytes();

    let mut rand = rand::thread_rng();
    let enc_data = pub_key.encrypt(
        &mut rand, 
        Pkcs1v15Encrypt,
        data).unwrap();
    let enc_data_str = String::from_utf8_lossy(&enc_data).into_owned();
    println!("{}",enc_data_str);
    let dec_code = private_key.decrypt(Pkcs1v15Encrypt, &enc_data).unwrap();
    println!("{}",String::from_utf8(dec_code).unwrap());
}