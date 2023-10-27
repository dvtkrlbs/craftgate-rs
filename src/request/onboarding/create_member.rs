use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::request::{onboarding::MemberType, Status};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    /// Set this parameter true if member is a buyer
    pub is_buyer: Option<bool>,
    /// Set this parameter true if member is a seller
    pub is_sub_merchant: Option<bool>,
    /// Member type. Required if member is a seller.
    pub member_type: Option<MemberType>,
    /// External id of the member
    pub member_external_id: String,
    /// Required if member is a seller and has limited/joint stock
    pub name: Option<String>,
    /// Address of the member
    pub address: String,
    /// E-mail address of the member
    pub email: String,
    /// Member IBAN. Required if member is a seller and `settlement_earnings_destination` is `IBAN`.
    /// Only TR IBAN should be sent
    pub iban: Option<String>,
    /// Phone number of the member
    pub phone_number: String,
    /// Legal company title of the member's company. Required if member is a seller and has
    /// limited/joint stock company
    pub legal_company_title: Option<String>,
    /// Tax office of the member's company. Required if member is a seller and has limited/joint
    /// stock company
    pub tax_office: Option<String>,
    /// Tax number of the member's company. Required if member is a seller and has limited/joint
    /// stock company
    pub tax_number: Option<String>,
    /// Tax number of the member's company. Required if member is a seller and has limited/joint stock company
    pub contact_name: Option<String>,
    /// Contact surname of the member. Required if member is a buyer or is a seller and has personal or private company
    pub contact_surname: Option<String>,
    /// Identity number of the member
    pub identity_number: Option<String>,
    /// Maximum allowed negative balance limit for sub merchant. It will be use if sub merchant balance is not enough for make refund.
    pub sub_merchant_maximum_allowed_negative_balance: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateMemberResponse {
    pub id: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_date: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_date: OffsetDateTime,
    pub status: Status,
    #[serde(flatten)]
    pub data: Member,
}
