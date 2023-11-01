use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::request::onboarding::{MemberType, SettlementEarningsDestination};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct UpdateMemberRequest {
    pub is_buyer: Option<bool>,
    pub is_sub_merchant: Option<bool>,
    pub member_type: Option<MemberType>,
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone_number: Option<String>,
    pub contact_name: String,
    pub contact_surname: String,
    pub identity_number: Option<String>,
    pub legal_company_title: Option<String>,
    pub tax_office: Option<String>,
    pub tax_number: Option<String>,
    pub iban: Option<String>,
    pub settlement_earnings_destination: Option<SettlementEarningsDestination>,
    pub sub_merchant_maximum_allowed_negative_balance: Option<u64>,
}