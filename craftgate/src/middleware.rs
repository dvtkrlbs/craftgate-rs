use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use data_encoding::BASE64;
use hmac_sha256::Hash;
use rand::distributions::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use reqwest::header::{HeaderName, InvalidHeaderValue};
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use secrecy::{ExposeSecret, SecretString};
use std::str::FromStr;
use task_local_extensions::Extensions;
use thiserror::Error;

const X_API_KEY: HeaderName = HeaderName::from_static("x-api-key");
const X_RND_KEY: HeaderName = HeaderName::from_static("x-rnd-key");
const X_AUTH_VERSION: HeaderName = HeaderName::from_static("x-auth-version");
const X_SIGNATURE: HeaderName = HeaderName::from_static("x-signature");

pub struct CraftgateSignatureMiddleware {
    secret_key: SecretString,
    access_key: SecretString,
}

impl CraftgateSignatureMiddleware {
    pub fn new_with_keys(secret_key: &str, access_key: &str) -> Self {
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
    let headers = req.headers_mut();
    headers.insert(X_API_KEY, access_key.parse()?);
    headers.insert(X_RND_KEY, random_string.parse()?);
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

        let res = next.run(req, extensions).await?;

        Ok(res)
    }
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
