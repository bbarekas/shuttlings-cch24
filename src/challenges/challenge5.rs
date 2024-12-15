// Challenge 5 : https://console.shuttle.dev/shuttlings/cch24/challenge/5

use std::fmt;
use axum::{
    response::{ IntoResponse, Response},
    http::{StatusCode, HeaderMap, header::CONTENT_TYPE},
    routing::post,
    Router
};
use cargo_manifest::Manifest;
use serde::Deserialize;
use serde_yaml::Value as YamlValue;
use serde_json::Value as JsonValue;
use serde_with::serde_as;


#[derive(Deserialize)]
struct Content {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    metadata: Metadata,
    keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(default)]
    orders: Vec<Order>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct Order {
    pub item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    pub quantity: Option<u32>,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quantity {
            Some(num) => write!(f, "{}: {:?}", self.item, num),
            None => write!(f, ""),
        }
    }
}

const MAGIC_KEYWORD: &str = "Christmas 2024";

//
pub enum ManifestError {
    NoContent,
    TomlParserError(toml::de::Error),
    CargoManifestError(cargo_manifest::Error),
    MagicKeywordNotFound,
    ContentTypeNotSupported,
}

impl IntoResponse for ManifestError {
    fn into_response(self) -> Response {
        const INVALID_MANIFEST_DETAIL: &str = "Invalid manifest";

        match self {
            ManifestError::NoContent => {
                println!("ERR: NoContent");
                (StatusCode::NO_CONTENT, "")
            },

            ManifestError::TomlParserError(rejection) => {
                println!("{}", rejection);
                (StatusCode::NO_CONTENT, "")
            },

            ManifestError::CargoManifestError(rejection) => {
                println!("{}", rejection);
                (StatusCode::BAD_REQUEST, INVALID_MANIFEST_DETAIL)
            },

            ManifestError::MagicKeywordNotFound => {
                println!("ERR: MagicKeywordNotFound");
                (StatusCode::BAD_REQUEST, "Magic keyword not provided")
            },

            ManifestError::ContentTypeNotSupported => {
                println!("ERR: ContentTypeNotSupported");
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, "")
            },

        }.into_response()
    }
}

impl From<toml::de::Error> for ManifestError {
    fn from(rejection: toml::de::Error) -> Self {
        Self::TomlParserError(rejection)
    }
}

impl From<cargo_manifest::Error> for ManifestError {
    fn from(rejection: cargo_manifest::Error) -> Self {
        Self::CargoManifestError(rejection)
    }
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/5/manifest", post(handle_manifest))

}

pub fn convert_json_to_toml(json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value: JsonValue = serde_json::from_str(json)?;
    let toml = toml::to_string(&value)?;
    Ok(toml)
}

pub fn convert_yaml_to_toml(yaml: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value: YamlValue = serde_yaml::from_str(yaml)?;
    let toml = toml::to_string(&value)?;
    Ok(toml)
}

async fn handle_manifest(headers: HeaderMap, body: String) ->  Result<String, ManifestError> {
    let content_type = headers.get(CONTENT_TYPE).unwrap().to_str().unwrap();
    //println!("Content Type: {:?}", content_type);
    let content: Content;

    let payload  = match content_type {
        // TOML
        "application/toml" => {
            body
        }
        // YAML
        "application/yaml"  => match convert_yaml_to_toml(&body) {
            Err(_) => return Err(ManifestError::ContentTypeNotSupported),
            Ok(toml) => toml,
        },
        // JSON
        "application/json" => match convert_json_to_toml(&body) {
            Err(_) => return Err(ManifestError::ContentTypeNotSupported),
            Ok(toml) => toml,
        },
        _ => return Err(ManifestError::ContentTypeNotSupported)
    };

    //println!("Payload:\n{:?}", payload);
    let manifest = Manifest::from_slice(&payload.as_bytes())?; //toml::de::Error
    if let Some(keywords) = manifest.package.unwrap().keywords {
        let keywords = keywords.as_local().unwrap();
        if !keywords.contains(&String::from(MAGIC_KEYWORD)) {
            return Err(ManifestError::MagicKeywordNotFound);
        }
    } else {
        return Err(ManifestError::MagicKeywordNotFound);
    }

    content = toml::from_str::<Content>(&payload)?;
    //println!("Content:\n{:?}", content);

    let order_items = content.package.metadata.orders
        .into_iter()
        .filter(|order| order.quantity.is_some())
        .map(|order| order.to_string())
        .collect::<Vec<String>>();

    //println!("Order items:\n{:?}", order_items);
    if order_items.is_empty() {
        return Err(ManifestError::NoContent);
    }

    Ok(order_items.join("\n"))
}
