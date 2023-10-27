pub mod create_member;
pub mod delete_member;

use serde::{Deserialize, Serialize};

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
