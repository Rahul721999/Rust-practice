use jsonwebtoken::{ Header, encode,EncodingKey,decode,  DecodingKey, errors::Error, Validation, Algorithm, get_current_timestamp};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims{
    pub email : String,
    pub exp : usize
}
impl TokenClaims{
    pub fn generate(&self) -> Result<String,Error>{
        let jwt_key =  "UnJtb25kYWwxMkAK";
        match encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(jwt_key.as_ref())
        ){
            Ok(token) => Ok(token),
            Err(err) => return Err(err)
        }
    }
}
fn main() {
    println!("{}",get_current_timestamp());
    let claims = TokenClaims{
        email : "anything@anywhere.any".to_string(),
        exp : get_current_timestamp() as usize,
    };
    let token = claims.generate().expect("Failed to encode");
    println!("{}",token.clone());
    let jwt_key =  "UnJtb25kYWwxMkAK";
    let res = decode_token(token.as_str(), jwt_key).expect("Failed");
    println!("email : {} ",res.email);
}
fn decode_token(token : &str, jwt_key : &str)-> Result<TokenClaims, Error>{
        match decode(
            &token, 
            &DecodingKey::from_secret(jwt_key.as_ref()),
            &Validation::new(Algorithm::HS256)
        ){
            Ok(data) =>return Ok(data.claims),
            Err(err) =>{ 
                println!("Decoding failed");
                return Err(err)},
        }
    }
