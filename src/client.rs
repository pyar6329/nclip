mod clipboard;
use crate::config::Port;
use anyhow::{Error, Result};
pub use clipboard::*;
use reqwest::{Client as ReqwestClient, Error as ReqwestError, Method as HTTPMethod, StatusCode};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error as ReqwestMiddlewareError};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use strum::EnumIs;
use thiserror::Error as ThisError;
use url::Url;
use ClientError::*;

#[derive(ThisError, Debug, Copy, Clone, Eq, PartialEq, EnumIs, Default)]
pub enum ClientError {
    #[error("Cannot create client")]
    InitializeError,
    #[error("The request is already created.")]
    Conflict, // 409
    #[error("The request isn't found.")]
    NotFound, // 404
    #[error("The requesting API key is not correct.")]
    AuthenticationError, // 401
    #[error("The request was malformed or missing some required parameters")]
    InvalidRequest, // 400
    #[error("Rate limit reached for requests API")]
    RateLimit, // 429
    #[error("Request timed out")]
    Timeout, // 408, 504
    #[error("The API had an error while processing our request")]
    ServiceUnavailableError, // 500, 503
    #[error("JSON Response Parsing was failed")]
    ResponseParseError,
    #[error("Reqwest Middleware error was occurred")]
    ReqwestMiddlewareError,
    #[error("Unknown error was occurred")]
    #[default]
    UnknownError,
}

pub struct Client {
    client: ClientWithMiddleware,
    base_url: Url,
    port: Port,
}

impl Client {
    pub fn new(base_url: Url, port: &Port, timeout: &u8, retry: &u8) -> Result<Self, ClientError> {
        let reqwest_client = ReqwestClient::builder()
            .timeout(Duration::from_secs(*timeout as u64))
            .build()
            .map_err(|_| InitializeError)?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(*retry as u32);

        let reqwest_middleware = ClientBuilder::new(reqwest_client)
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        let client = Self {
            client: reqwest_middleware,
            base_url,
            port: *port,
        };

        Ok(client)
    }

    pub async fn execute<T, U, V>(
        &self,
        method: HTTPMethod,
        endpoint: V,
        body: &T,
    ) -> Result<U, ClientError>
    where
        T: Serialize,
        U: DeserializeOwned,
        V: AsRef<Path>,
    {
        let endpoint_strs: Vec<&str> = endpoint
            .as_ref()
            .to_str()
            .unwrap_or_default()
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let mut url = self.base_url.to_owned();

        let _ = url.set_port(Some(self.port));

        if let Ok(mut url_segment) = url.path_segments_mut() {
            url_segment.pop_if_empty().extend(endpoint_strs);
        }

        let response = self
            .client
            .request(method, url)
            .json(&body)
            .send()
            .await
            .map_err(ClientError::from)?;

        let status = response.status();

        if status.is_client_error() || status.is_server_error() {
            return Err(ClientError::from(status));
        }

        response.json::<U>().await.map_err(ClientError::from)
    }
}

impl From<StatusCode> for ClientError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::NOT_FOUND => NotFound,               // 404
            StatusCode::CONFLICT => Conflict,                // 409
            StatusCode::REQUEST_TIMEOUT => Timeout,          // 408
            StatusCode::TOO_MANY_REQUESTS => RateLimit,      // 429
            StatusCode::GATEWAY_TIMEOUT => Timeout,          // 504
            StatusCode::UNAUTHORIZED => AuthenticationError, // 401
            s if s.is_client_error() => InvalidRequest,      // 400
            _ => ServiceUnavailableError,                    // 500
        }
    }
}

impl From<ReqwestError> for ClientError {
    fn from(err: ReqwestError) -> Self {
        if err.is_timeout() {
            return Timeout;
        }
        if err.is_connect() {
            return ServiceUnavailableError;
        }
        if err.is_body() {
            return InvalidRequest;
        }
        if err.is_decode() {
            return ResponseParseError;
        }
        UnknownError
    }
}

impl From<ReqwestMiddlewareError> for ClientError {
    fn from(err: ReqwestMiddlewareError) -> Self {
        if let ReqwestMiddlewareError::Reqwest(e) = err {
            return Self::from(e);
        }
        if let ReqwestMiddlewareError::Middleware(_) = err {
            return ReqwestMiddlewareError;
        }
        UnknownError
    }
}
