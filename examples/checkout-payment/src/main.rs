use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Extension, Form, Json, Router, ServiceExt};
use craftgate::client::CraftgateClient;
use craftgate::request::payment::checkout_payment::CheckoutPaymentInitiationRequestBuilder;
use craftgate::request::payment::{
    Payment, PaymentGroup, PaymentItem, PaymentItemBuilder, PaymentPhase,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY");
    let client = CraftgateClient::new(true, &api_key, &secret_key);

    let app = Router::new()
        .route("/callback", post(callback_handler))
        .route("/payment", get(payment_handler))
        .layer(Extension(client));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
pub struct CallbackParams {
    pub token: String,
}

async fn callback_handler(
    Extension(client): Extension<CraftgateClient>,
    Form(params): Form<CallbackParams>,
) -> Json<Payment> {
    dbg!(&params);

    let payment = client.checkout_payment_inquiry(params.token).await.unwrap();

    Json(payment)
}

async fn payment_handler(Extension(client): Extension<CraftgateClient>) -> Response {
    let payment_initiation_request = CheckoutPaymentInitiationRequestBuilder::default()
        .price(10.0.try_into().unwrap())
        .paid_price(10.0.try_into().unwrap())
        .payment_group(PaymentGroup::Product)
        .payment_phase(PaymentPhase::Auth)
        .external_id("test123".to_owned())
        .callback_url("http://127.0.0.1:3000/callback".to_owned())
        .items(vec![PaymentItemBuilder::default()
            .price(10.0.try_into().unwrap())
            .build()
            .unwrap()])
        .build()
        .unwrap();

    let resp = client
        .initiate_checkout_payment(payment_initiation_request)
        .await
        .unwrap();

    dbg!(&resp);

    Redirect::to(&resp.page_url).into_response()
}
