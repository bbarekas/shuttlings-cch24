use axum::{
    Router,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

const SECRET: &[u8] = b"santarocks";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    data: serde_json::Value,
}

impl Claims {
    fn new(data: serde_json::Value, exp: usize) -> Self {
        Self {
            sub: "gift".to_string(),
            exp,
            data,
        }
    }
}

pub fn get_routes() -> Router {
    
    Router::new()
        .route("/16/wrap", post(handle_wrap))
        .route("/16/unwrap", get(handle_unwrap))
        //.route("/16/decode", post(handle_decode))

}

async fn handle_wrap(headers: HeaderMap, body: String) ->  impl IntoResponse {

    if let Some("application/json") = headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()) {

        let data: serde_json::Value = serde_json::from_str(&body).unwrap();
        println!("JSON Body:\n{}", data);
        let expire = (chrono::Utc::now() + chrono::Days::new(1)).timestamp() as usize;
        
        let claims = Claims::new(data, expire);
        let Ok(token) = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET))
        else {
            return (StatusCode::BAD_REQUEST, "Invalid JWT token".to_string()).into_response();
        };
        (StatusCode::OK, [(header::SET_COOKIE, format!("gift={token}"))], body.to_string()).into_response()
    } else {
        (StatusCode::BAD_REQUEST, "NO JSON given\n").into_response()
    }
}

async fn handle_unwrap(headers: HeaderMap) ->  impl IntoResponse {

    let Some(header_value) = headers.get(header::COOKIE) else {
        return (StatusCode::BAD_REQUEST, "Missing cookie".to_string()).into_response();
    };
    let Ok(cookie) = header_value.to_str() else {
        return (StatusCode::BAD_REQUEST, "Invalid cookie".to_string()).into_response();
    };
    let token: String = cookie.split("gift=").collect();
    // Decode the token.
    let Ok(data) = decode::<Claims>(&token, 
                                &DecodingKey::from_secret(SECRET), 
                                &Validation::default()) 
    else { 
        return (StatusCode::BAD_REQUEST, "Invalid JWT token".to_string()).into_response();
    };
    //println!("{:?}", data);
    (StatusCode::OK, data.claims.data.to_string()).into_response()
}
