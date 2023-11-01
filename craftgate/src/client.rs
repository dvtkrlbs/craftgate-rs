
use reqwest::{
    Response, Url,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

use serde::{Deserialize};

use crate::{
    api_error::{ErrorCode, ErrorGroup, ErrorResponse},
    response::{ApiResponse, SuccessResponse},
};
use crate::middleware::CraftgateSignatureMiddleware;
use crate::request::onboarding::create_member::CreateMemberRequest;
use crate::request::onboarding::Member;
use crate::request::onboarding::search_members::SearchMembersRequest;
use crate::request::onboarding::update_member::UpdateMemberRequest;
use crate::request::payment::checkout_payment::{CheckoutPaymentInitiationRequest, CheckoutPaymentInitiationResponse};
use crate::request::payment::Payment;
use crate::response::{ApiResponseVariant, PaginatedResponse};



#[derive(Clone)]
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
    ReqwestMiddlewareError(reqwest_middleware::Error)
}

impl From<reqwest_middleware::Error> for CraftgateError {
    fn from(value: reqwest_middleware::Error) -> Self { Self::ReqwestMiddlewareError(value) }
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
        member: CreateMemberRequest,
    ) -> Result<Member, CraftgateError> {
        let resp = self
            .client
            .post(
                self.base_url
                    .join("/onboarding/v1/members")
                    .expect("valid url"),
            )
            .json(&member)
            .send()
            .await?;

        let member: Member = extract_single_response(resp).await?;

        Ok(member)
    }

    pub async fn update_member(
        &self,
        member_id: u64,
        member: UpdateMemberRequest,
    ) -> Result<Member, CraftgateError> {
        let resp = self
            .client
            .put(
                self.base_url
                    .join(&format!("/onboarding/v1/members/{}", member_id))
                    .expect("valid_url"))
            .json(&member)
            .send()
            .await?;

        let member: Member = extract_single_response(resp).await?;

        Ok(member)
    }

    pub async fn retrieve_member(
        &self,
        member_id: u64
    ) -> Result<Option<Member>, CraftgateError> {
        let resp = self
            .client
            .get(
                self.base_url
                    .join(&format!("/onboarding/v1/members/{}", member_id))
                    .expect("valid_url")
            )
            .send()
            .await?;

        let member: Option<Member> = extract_single_response(resp).await?;

        Ok(member)
    }

    pub async fn search_members(
        &self,
        params: SearchMembersRequest
    ) -> Result<PaginatedResponse<Member>, CraftgateError> {
        let resp = self
            .client
            .get(
                self.base_url
                    .join("/onboarding/v1/members")
                    .expect("valid url")
            )
            .query(&params)
            .send()
            .await?;

        let members: PaginatedResponse<Member> = extract_paginated_response(resp).await?;

        Ok(members)
    }

    pub async fn initiate_checkout_payment(
        &self,
        params: CheckoutPaymentInitiationRequest
    ) -> Result<CheckoutPaymentInitiationResponse, CraftgateError> {
        let resp = self
            .client
            .post(
                self.base_url
                    .join("/payment/v1/checkout-payments/init")
                    .expect("valid url")
            )
            .json(&params)
            .send()
            .await?;

        let resp: CheckoutPaymentInitiationResponse = extract_single_response(resp).await?;

        Ok(resp)
    }

    pub async fn checkout_payment_inquiry(
        &self,
        token: String,
    ) -> Result<Payment, CraftgateError> {
        let resp = self
            .client
            .get(
                self.base_url
                    .join(&format!("/payment/v1/checkout-payments/{}", token))
                    .expect("valid url")
            )
            .send()
            .await?;

        // let payment: Payment = extract_single_response(resp).await?;

        #[derive(Deserialize)]
        pub struct Data {
            data: Payment
        }

        let resp_text = resp.text().await.unwrap();
        println!("{}", &resp_text);
        // let payment: Data = resp.json().await.unwrap();
        let payment: Data = serde_json::from_str(&resp_text).unwrap();
        Ok(payment.data)
    }

    pub async fn expire_common_page_token(
        &self,
        token: String,
    ) -> Result<(), CraftgateError> {
        let _resp = self
            .client
            .delete(
                self.base_url
                    .join(&format!("/payment/v1/checkout-payments/{}", token))
                    .expect("valid url")
            )
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[allow(unused)]
async fn extract_single_response<T: for<'a> Deserialize<'a>>(resp: Response) -> Result<T, CraftgateError> {
    // let text = resp.text().await?;
    // dbg!(&text);
    let resp: ApiResponse<T> = resp.json().await?;
    // let resp: ApiResponse<T> = serde_json::from_str(&text).unwrap();
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
async fn extract_paginated_response<T: for<'a> Deserialize<'a>>(resp: Response) -> Result<PaginatedResponse<T>, CraftgateError> {
    let resp: ApiResponse<T> = resp.json().await?;

    match resp.response {
        ApiResponseVariant::Error(e) => Err(CraftgateError::from(e)),
        ApiResponseVariant::Success(succ) => match succ {
            SuccessResponse::Single(_s) => Err(CraftgateError::UnexpectedFormat {
                expected: ResponseFormat::Paginated,
            }),
            SuccessResponse::Paginated(paginated) => Ok(paginated),
        },
    }
}


