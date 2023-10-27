use std::str::FromStr;

use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use data_encoding::BASE64;
use hmac_sha256::Hash;
use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use reqwest::{
    header::{HeaderName, InvalidHeaderValue},
    Request, Response, Url,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension, Middleware, Next};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Error;
use task_local_extensions::Extensions;
use thiserror::Error;

use crate::{
    api_error::{ErrorCode, ErrorGroup, ErrorResponse},
    request::onboarding::create_member::{CreateMemberResponse, Member},
    response::{ApiResponse, SuccessResponse},
};
use crate::response::ApiResponseVariant;

const X_API_KEY: HeaderName = HeaderName::from_static("x-api-key");
const X_RND_KEY: HeaderName = HeaderName::from_static("x-rnd-key");
const X_AUTH_VERSION: HeaderName = HeaderName::from_static("x-auth-version");
const X_SIGNATURE: HeaderName = HeaderName::from_static("x-signature");

pub struct CraftgateSignatureMiddleware {
    secret_key: SecretString,
    access_key: SecretString,
}

impl CraftgateSignatureMiddleware {
    fn new_with_keys(secret_key: &str, access_key: &str) -> Self {
        Self {
            secret_key: SecretString::from_str(secret_key).expect("infallible"),
            access_key: SecretString::from_str(access_key).expect("infallible"),
        }
    }
}

impl From<SignatureCalculationError> for reqwest_middleware::Error {
    fn from(value: SignatureCalculationError) -> Self {
        reqwest_middleware::Error::Middleware(anyhow::anyhow!(value))
    }
}

#[derive(Error, Debug)]
pub enum SignatureCalculationError {
    #[error("Body is incompatible for safe consumption")]
    IncompatibleBody,
    #[error("Invalid header value {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}

fn calculate_signature(
    mut req: Request,
    access_key: &str,
    secret_key: &str,
    random_string: &str,
) -> Result<Request, SignatureCalculationError> {
    let mut payload = BytesMut::new();

    payload.put_slice(req.url().as_str().as_bytes());
    payload.put_slice(access_key.as_bytes());
    payload.put_slice(secret_key.as_bytes());

    payload.put_slice(random_string.as_bytes());

    if let Some(body) = req.body() {
        let Some(body_bytes) = body.as_bytes() else {
            return Err(SignatureCalculationError::IncompatibleBody);
        };

        payload.put_slice(body_bytes);
    }

    let hash = Hash::hash(payload.as_ref());

    let base64_hash = BASE64.encode(&hash);
    let mut headers = req.headers_mut();
    headers.insert(X_API_KEY, access_key.parse()?);
    headers.insert(X_RND_KEY, random_string.clone().parse()?);
    headers.insert(X_AUTH_VERSION, "1".parse()?);
    headers.insert(X_SIGNATURE, base64_hash.parse()?);

    Ok(req)
}

#[async_trait]
impl Middleware for CraftgateSignatureMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let rng = ChaCha20Rng::from_entropy();
        let random_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let req = calculate_signature(
            req,
            self.access_key.expose_secret(),
            self.secret_key.expose_secret(),
            &random_string,
        )?;

        dbg!(&req);

        let res = next.run(req, extensions).await?;

        Ok(res)
    }
}

pub struct CraftgateClient {
    client: ClientWithMiddleware,
    base_url: Url,
}

#[derive(Debug)]
pub enum ResponseFormat {
    Single,
    Paginated,
}

#[derive(Debug)]
pub enum CraftgateError {
    ValidationError {
        code: ErrorCode,
        description: String,
    },
    PaymentError {
        code: ErrorCode,
        description: String,
        group: ErrorGroup,
    },
    UnexpectedFormat {
        expected: ResponseFormat,
    },
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
}

impl From<serde_json::Error> for CraftgateError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<reqwest::Error> for CraftgateError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl From<ErrorResponse> for CraftgateError {
    fn from(value: ErrorResponse) -> Self {
        if value.code.0 < 10000 {
            Self::ValidationError {
                code: value.code,
                description: value.description,
            }
        } else {
            Self::PaymentError {
                code: value.code,
                description: value.description,
                group: value.group.expect("Payment error expected have a group"),
            }
        }
    }
}

impl CraftgateClient {
    pub(crate) fn new(sandbox: bool, api_key: &str, secret_key: &str) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(TracingMiddleware::default())
            // Retry failed requests.
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .with(CraftgateSignatureMiddleware::new_with_keys(
                secret_key, api_key,
            ))
            .build();

        Self {
            client,
            base_url: Self::url(sandbox),
        }
    }

    fn url(sandbox: bool) -> Url {
        if sandbox {
            Url::parse("https://sandbox-api.craftgate.io").unwrap()
        } else {
            Url::parse("https://api.craftgate.io").unwrap()
        }
    }

    fn from_client(client: ClientWithMiddleware, sandbox: bool) -> Self {
        Self {
            client,
            base_url: Self::url(sandbox),
        }
    }

    pub(crate) async fn create_member(
        &self,
        member: Member,
    ) -> Result<CreateMemberResponse, CraftgateError> {
        let resp = self
            .client
            .post(
                self.base_url
                    .join("/onboarding/v1/members")
                    .expect("valid url"),
            )
            .json(&member)
            .send()
            .await.expect("valid body");

        // dbg!(&resp.text().await);
        let member: CreateMemberResponse = extract_single_response(resp).await?;

        // Err(CraftgateError::UnexpectedFormat { expected: ResponseFormat::Single})
        Ok(member)
    }
}

async fn extract_single_response<T: for<'a> Deserialize<'a>>(resp: Response) -> Result<T, CraftgateError> {
    dbg!(&resp);
    let resp: ApiResponse<T> = resp.json().await?;

    return match resp.response {
        ApiResponseVariant::Error(e) => Err(CraftgateError::from(e)),
        ApiResponseVariant::Success(succ) => match succ {
            SuccessResponse::Single(s) => Ok(s),
            SuccessResponse::Paginated(_) => Err(CraftgateError::UnexpectedFormat {
                expected: ResponseFormat::Single,
            }),
        },
    };
}

async fn extract_paginated_response<T: for<'a> Deserialize<'a>>(resp: Response) -> Result<Vec<T>, CraftgateError> {
    let resp: ApiResponse<T> = resp.json().await?;

    return match resp.response {
        ApiResponseVariant::Error(e) => Err(CraftgateError::from(e)),
        ApiResponseVariant::Success(succ) => match succ {
            SuccessResponse::Single(s) => Err(CraftgateError::UnexpectedFormat {
                expected: ResponseFormat::Paginated,
            }),
            SuccessResponse::Paginated(paginated) => Ok(paginated.items),
        },
    };
}

#[cfg(test)]
mod tests {
    use reqwest::{Body, Method, Request};

    use super::{calculate_signature, X_API_KEY, X_AUTH_VERSION, X_RND_KEY, X_SIGNATURE};

    #[test]
    fn test_without_body() {
        let req = Request::new(
            Method::GET,
            "https://api.craftgate.io/onboarding/v1/members/1"
                .parse()
                .unwrap(),
        );

        let req = calculate_signature(req, "key-1", "FooBar123!", "Xa15Fp11T").unwrap();

        assert_eq!(req.headers().get(X_API_KEY).unwrap(), "key-1");
        assert_eq!(req.headers().get(X_RND_KEY).unwrap(), "Xa15Fp11T");
        assert_eq!(req.headers().get(X_AUTH_VERSION).unwrap(), "1");
        assert_eq!(
            req.headers().get(X_SIGNATURE).unwrap(),
            "y1TtnjNCJEvlkP5ufCkK3H0i2guMB/bKL4Ayw3VlKWA="
        );
    }

    #[test]
    fn test_with_body() {
        let mut req = Request::new(
            Method::POST,
            "https://api.craftgate.io/onboarding/v1/members"
                .parse()
                .unwrap(),
        );

        *req.body_mut() = Some(Body::from(
            r#"{"email": "haluk.demir@example.com","name": "Haluk Demir","phoneNumber": "905551111111","address": "Beylerbeyi Cad. Lale Sok. No: 38 Daire: 3 Üsküdar","identityNumber": "11111111110","contactName": "Haluk","contactSurname": "Demir","memberExternalId": "0ac49f08-f2a9-4326-a4d8-f6c1b01596fb"}"#,
        ));

        let req = calculate_signature(req, "key-1", "FooBar123!", "Xa15Fp11T").unwrap();

        assert_eq!(req.headers().get(X_API_KEY).unwrap(), "key-1");
        assert_eq!(req.headers().get(X_RND_KEY).unwrap(), "Xa15Fp11T");
        assert_eq!(req.headers().get(X_AUTH_VERSION).unwrap(), "1");
        assert_eq!(
            req.headers().get(X_SIGNATURE).unwrap(),
            "nv8y2bSnFjYNRzVRqzkHTK5RXKuN04hoK6fLE2+nzTw="
        );
    }
}
