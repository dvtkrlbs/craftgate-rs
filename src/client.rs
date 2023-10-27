



use reqwest::{
    Response, Url,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

use serde::{Deserialize};

use crate::{
    api_error::{ErrorCode, ErrorGroup, ErrorResponse},
    request::onboarding::create_member::{CreateMemberResponse, Member},
    response::{ApiResponse, SuccessResponse},
};
use crate::middleware::CraftgateSignatureMiddleware;
use crate::response::ApiResponseVariant;



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
    pub fn new(sandbox: bool, api_key: &str, secret_key: &str) -> Self {
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

    pub async fn create_member(
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

        let member: CreateMemberResponse = extract_single_response(resp).await?;

        Ok(member)
    }
}

#[allow(unused)]
async fn extract_single_response<T: for<'a> Deserialize<'a>>(resp: Response) -> Result<T, CraftgateError> {
    let resp: ApiResponse<T> = resp.json().await?;

    match resp.response {
        ApiResponseVariant::Error(e) => Err(CraftgateError::from(e)),
        ApiResponseVariant::Success(succ) => match succ {
            SuccessResponse::Single(s) => Ok(s),
            SuccessResponse::Paginated(_) => Err(CraftgateError::UnexpectedFormat {
                expected: ResponseFormat::Single,
            }),
        },
    }
}

#[allow(unused)]
async fn extract_paginated_response<T: for<'a> Deserialize<'a>>(resp: Response) -> Result<Vec<T>, CraftgateError> {
    let resp: ApiResponse<T> = resp.json().await?;

    match resp.response {
        ApiResponseVariant::Error(e) => Err(CraftgateError::from(e)),
        ApiResponseVariant::Success(succ) => match succ {
            SuccessResponse::Single(_s) => Err(CraftgateError::UnexpectedFormat {
                expected: ResponseFormat::Paginated,
            }),
            SuccessResponse::Paginated(paginated) => Ok(paginated.items),
        },
    }
}


