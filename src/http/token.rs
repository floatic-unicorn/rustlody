use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};


const PANTOS_LOCAL_KEY: &str = 
"f9e9e44b8d96e1b3ba2817bc31233e8e3cc83aec84eff4c5fbdaedbfe2dba8cc65477841a65ded6d78ee88a0feabfc3571b8d19394d0ccf7b6fb55d41383856a";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    r#type: String,
    #[serde(rename = "accountId")]
    account_id: String,
    email: String,
    entity: String,
    entity_id: Option<String>,
    exp: u64,
}

pub fn make_token() -> String {
    let my_claims = Claims {
        r#type: "accessToken".to_string(),
        account_id: "dBK39Eak".to_string(),
        email: "local-test@floatic.io".to_string(),
        entity: "FLODY_CONSOLE".to_string(),
        entity_id: None,
        exp: 10000000000
    };

    let header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };

    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(PANTOS_LOCAL_KEY.as_bytes())) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    token
}
