use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hmac::{Hmac, Mac};
use jwt::header::HeaderType;
use jwt::token::Signed;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use serde_json::{Number, Value};
use sha2::Sha384;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .hash
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str) -> bool {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    argon2.verify_password(password.as_bytes(), &hash).is_ok()
}

pub fn mint_token<'a>(
    access_secret: &'a str,
    sub: &'a str,
    tenant_id: &'a str,
) -> Result<Token<Header, BTreeMap<&'a str, Value>, Signed>, &'a str> {
    let key: Hmac<Sha384> = match Hmac::new_from_slice(access_secret.as_bytes()) {
        Ok(k) => k,
        Err(_) => return Err("Error signing JWT."),
    };

    let header = Header {
        algorithm: AlgorithmType::Hs384,
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    };

    let mut claims: BTreeMap<&str, Value> = BTreeMap::new();
    claims.insert("sub", Value::String(sub.to_string()));
    claims.insert("tid", Value::String(tenant_id.to_string()));
    claims.insert(
        "iat",
        Value::Number(Number::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )),
    );
    claims.insert(
        "exp",
        Value::Number(Number::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 300,
        )),
    );
    claims.insert("iss", Value::String(std::env::var("DOMAIN_NAME").unwrap()));

    let token = Token::new(header, claims);

    match token.sign_with_key(&key) {
        Ok(t) => Ok(t),
        Err(_) => Err("Error signing JWT."),
    }
}
