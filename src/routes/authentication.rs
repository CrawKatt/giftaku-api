use dotenvy::var;
use jsonwebtoken::{
    self, decode, encode, Algorithm::HS256, DecodingKey, EncodingKey, Header, Validation,
};
use rocket::{
    get,
    http::{Status, StatusClass},
    request::{FromRequest, Outcome},
    serde::json::Json,
    Request,
};
use serde::{Deserialize, Serialize};
use std::{
    convert::Infallible,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize, Serialize)]
struct Claims {
    exp: usize,
}

#[derive(Deserialize, Serialize)]
pub struct TokenAuthenticate {
    token: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for TokenAuthenticate {
    type Error = StatusClass;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token) = request.headers().get_one("token") {
            let secret_key = var("SIGNATURE_KEY").expect("SIGNATURE_KEY is not set");
            let validation_token = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret_key.as_ref()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            if validation_token.is_err() {
                Outcome::Error((Status::Unauthorized, StatusClass::ClientError))
            } else {
                Outcome::Success(TokenAuthenticate {
                    token: token.to_string(),
                })
            }
        } else {
            Outcome::Error((Status::Unauthorized, StatusClass::ClientError))
        }
    }
}

#[get("/authenticate")]
pub fn authenticate() -> Json<TokenAuthenticate> {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    let claims = Claims {
        exp: current_timestamp as usize,
    };

    let signature = var("SIGNATURE_KEY").expect("SIGNATURE_KEY is not set");

    let token = encode(
        &Header::new(HS256),
        &claims,
        &EncodingKey::from_secret(signature.as_ref()),
    )
    .unwrap();

    Json(TokenAuthenticate { token })
}
