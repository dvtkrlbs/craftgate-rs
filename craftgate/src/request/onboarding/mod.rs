pub mod create_member;
pub mod search_members;
pub mod update_member;

use crate::request::Status;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MemberType {
    /// Personal member
    Personal,
    /// Private Company member
    PrivateCompany,
    /// Limited or joint stock company member
    LimitedOrJointStockCompany,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutDestination {
    /// IBAN
    Iban,
    /// Member Wallet
    Wallet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutMerchantType {
    /// Merchant
    Merchant,
    /// Sub merchant member
    SubMerchantMember,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutTransactionType {
    /// Payout for payments
    Payment,
    /// Payout for withdraws
    Withdraw,
    /// Payout for refund after settlement
    Ras,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutReturnStatus {
    /// The first value that occurs when money is sent. It means that the money transfer is sent on
    /// the first try.
    NotBounced,
    /// The status that occurs when the money transfer is marked as returned.
    Bounced,
    /// The status that occurs when the sub-merchant information is updated together with the Name
    /// / Surname / Iban information that is the subject of money transfer.
    Updated,
    /// The status that occurs when sending money again for a returned transaction.
    PayoutStarted,
    /// The status that occurs after the money transfer is made again for a returned transaction.
    PayoutCompleted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementEarningsDestination {
    Iban,
    Wallet,
    CrossBorder,
}

impl Default for SettlementEarningsDestination {
    fn default() -> Self {
        Self::Iban
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: u64,
    #[serde(with = "crate::request::craftgate")]
    pub created_date: PrimitiveDateTime,
    #[serde(with = "crate::request::craftgate::option")]
    #[serde(default)]
    pub updated_date: Option<PrimitiveDateTime>,
    pub status: Status,
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
    pub sub_merchant_maximum_allowed_negative_balance: f64,
}
