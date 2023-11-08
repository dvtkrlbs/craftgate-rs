use crate::request::onboarding::{MemberType, SettlementEarningsDestination};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Debug, Clone, Default, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default))]
pub struct UpdateMemberRequest {
    pub is_buyer: Option<bool>,
    pub is_sub_merchant: Option<bool>,
    pub member_type: Option<MemberType>,
    #[builder(!default)]
    pub name: String,
    #[builder(!default)]
    pub email: String,
    #[builder(!default)]
    pub address: String,
    pub phone_number: Option<String>,
    #[builder(!default)]
    pub contact_name: String,
    #[builder(!default)]
    pub contact_surname: String,
    pub identity_number: Option<String>,
    pub legal_company_title: Option<String>,
    pub tax_office: Option<String>,
    pub tax_number: Option<String>,
    pub iban: Option<String>,
    pub settlement_earnings_destination: Option<SettlementEarningsDestination>,
    pub sub_merchant_maximum_allowed_negative_balance: Option<u64>,
}
