use std::env;

use db::Db;
use jsonwebtoken::{
    decode, encode, errors::Result as JWTResult, get_current_timestamp, Algorithm, DecodingKey,
    EncodingKey, Validation,
};
use model_entity::dto::{LoginInfo, TokenOut};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    serde::{json::Json, Deserialize, Serialize},
};
use sea_orm_rocket::Connection;
use service::AccountService;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    // aud: String, // Optional. Audience
    exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // iat: usize, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    nbf: u64,    // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub struct User {
    pub id: i32,
}

struct Token;

// impl Claims {
//     pub fn new(sub: String, nbf: OffsetDateTime, exp: OffsetDateTime) -> Self {
//         let nbf = nbf
//             .date()
//             .with_hms_milli(nbf.hour(), nbf.minute(), nbf.second(), 0)
//             .unwrap()
//             .assume_utc();
//         let exp = exp
//             .date()
//             .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
//             .unwrap()
//             .assume_utc();
//         Self {
//             exp: exp,
//             nbf: nbf,
//             sub: sub,
//         }
//     }
// }

impl Token {
    fn create(account_id: i32) -> JWTResult<String> {
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            exp: expiration as u64,
            nbf: get_current_timestamp(),
            sub: account_id.to_string(),
        };

        encode(
            &jsonwebtoken::Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
    }

    fn validate(token: &str) -> Result<i32, core::num::ParseIntError> {
        let validation = Validation::new(Algorithm::HS512);
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let _token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &validation,
        )
        .unwrap();
        _token_data.claims.sub.parse::<i32>()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<User, Self::Error> {
        let token = match req.headers().get_one("Authorization") {
            Some(header) => header.trim_start_matches("Bearer "),
            None => return Outcome::Error((Status::BadRequest, ())),
        };
        match Token::validate(&token) {
            Ok(id) => Outcome::Success(User { id: id }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[post("/login", data = "<login>")]
async fn login(conn: Connection<'_, Db>, login: Json<LoginInfo>) -> Option<Json<TokenOut>> {
    let db = conn.into_inner();
    match AccountService::get(db, login.into_inner().id)
        .await
        .unwrap()
    {
        Some(account_object) => Some(Json(TokenOut {
            token: Token::create(account_object.id).unwrap(),
        })),
        None => None,
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}
