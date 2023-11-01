use derive_builder::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::request::Currency;
use crate::request::payment::{PaymentGroup, PaymentItem, PaymentMethod, PaymentPhase};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct CheckoutPaymentInitiationRequest {
    pub conversation_id: Option<String>,
    pub external_id: Option<String>,
    pub bank_order_id: Option<String>,
    pub price: Decimal,
    pub paid_price: Decimal,
    pub buyer_member_id: Option<String>,
    pub currency: Currency,
    pub payment_group: PaymentGroup,
    pub payment_phase: PaymentPhase,
    pub payment_channel: Option<String>,
    pub callback_url: String,
    pub card_user_key: Option<String>,
    pub enabled_installments: Option<Vec<u64>>,
    pub allow_only_credit_card: Option<bool>,
    pub allow_only_stored_cards: Option<bool>,
    pub allow_store_card_after_payment: Option<bool>,
    pub allow_installment_only_commercial_cards: Option<bool>,
    pub force_auth_for_non_credit_cards: Option<bool>,
    pub force_three_d_s: Option<bool>,
    pub ttl: Option<u64>,
    pub items: Vec<PaymentItem>,
    pub masterpass_gsm_number: Option<String>,
    pub masterpass_user_id: Option<String>,
    pub apm_user_identity: Option<String>,
    pub enabled_payment_methods: Option<Vec<PaymentMethod>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutPaymentInitiationResponse {
    pub token: String,
    pub page_url: String,
    // #[serde(with = "time::serde::rfc2822")]
    pub token_expire_date: String,
}