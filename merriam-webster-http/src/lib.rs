#![doc = include_str!("../README.md")]
use connector::Connector;
use error::MerriamWebsterError;
use hyper::{
    client::Client as HyperClient,
    header::{CONTENT_TYPE, USER_AGENT},
    Body, Method, Request,
};
use merriam_webster_model::{entry::Entry, top_words::APIGetTopWordsJSONResponse};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

mod connector;
pub mod error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionResponse {
    Entries(Vec<Entry>),
    PotentialWords(Vec<String>),
}
/// A client for the Merriam Webster API.
pub struct MerriamWebsterClient {
    key: Secret<String>,
    http: HyperClient<Connector>,
}

const MERRIAM_WEBSTER_USER_AGENT: &str = concat!(
    "MerriumWebsterHttpRust (",
    env!("CARGO_PKG_HOMEPAGE"),
    ", ",
    env!("CARGO_PKG_VERSION"),
    ")",
);

impl MerriamWebsterClient {
    /// Create a new Merriam Webster client.
    pub fn new(key: Secret<String>) -> Self {
        let connector = connector::create();
        let http = hyper::Client::builder().build(connector);

        Self { http, key }
    }

    /// Fetch a definition from the Merriam Webster API.
    pub async fn collegiate_definition(
        &self,
        word: String,
    ) -> Result<DefinitionResponse, MerriamWebsterError> {
        let url_encoded_word = utf8_percent_encode(&word, NON_ALPHANUMERIC).to_string();
        let request = Request::builder()
            .method(Method::GET)
            .uri(format!(
                "https://www.dictionaryapi.com/api/v3/references/learners/json/{url_encoded_word}?key={}",
                self.key.expose_secret()
            ))
            .header(USER_AGENT, MERRIAM_WEBSTER_USER_AGENT)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(vec![0; 0]))
            .expect("request builder");

        let response = self.http.request(request).await?;
        let body_bytes = hyper::body::to_bytes(response).await?;
        // println!("{:?}", &body_bytes);
        let body = serde_json::from_slice::<DefinitionResponse>(&body_bytes)?;

        Ok(body)
    }

    /// Fetch the currently most-searched words from the Merriam Webster API.
    pub async fn top_words(&self) -> Result<APIGetTopWordsJSONResponse, MerriamWebsterError> {
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://www.merriam-webster.com/lapi/v1/mwol-mp/get-lookups-data-homepage")
            .header(USER_AGENT, MERRIAM_WEBSTER_USER_AGENT)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(vec![0; 0]))
            .expect("request builder");

        let response = self.http.request(request).await?;

        let body_bytes = hyper::body::to_bytes(response).await?;
        let body = serde_json::from_slice::<APIGetTopWordsJSONResponse>(&body_bytes)?;

        Ok(body)
    }
}
