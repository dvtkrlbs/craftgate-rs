use crate::request::{CardType, Currency};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::PrimitiveDateTime;
use typed_builder::TypedBuilder;

pub mod checkout_payment;

/// Values expressing the way a payment is collected
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    /// Payment fully deducted from card
    CardPayment,
    /// Top-up payment from card
    DepositPayment,
    /// Payment fully deducted from wallet
    WalletPayment,
    /// Payment, some of which is debited from a card and some from a wallet
    CardAndWalletPayment,
    /// Bank transfer
    BankTransfer,
}

/// Group names expressing the product or service to which the payment is done
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentGroup {
    /// Product
    #[default]
    Product,
    /// Ad, listing, service or subscription
    ListingOrSubscription,
}

/// Values expressing the status of the payment
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    /// Payment is failed
    Failure,
    /// Payment is successful
    Success,
    /// 3D Secure payment is initiated
    InitThreeds,
    /// After entering the sms code on the bank's 3D Secure SMS page, the transaction is waiting
    /// for buyer return to the callback address of merchant and complete the payment
    CallbackThreeds,
}

/// Values expressing approval status for a payment item
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentItemConfirmationStatus {
    /// Awaiting approval
    WaitingForApproval,
    /// Approved
    Approved,
}

/// Transaction type/phase of the payment at the bank
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentPhase {
    /// In standard provision transaction, the payment is set as this status value
    #[default]
    Auth,
    /// If a Pre-authorization is made, the payment phase takes this value. In the pre-authorization
    /// process, the payment is not withdrawn directly, the relevant amount is blocked on the user's
    /// card by the bank during the provisioning period
    PreAuth,
    /// In case of capture for a pre-authorized transaction, the payment phase takes this value
    PostAuth,
}

/// Users can pay below methods from Common Payment Page/Form
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethod {
    /// User pays with card
    Card,
    /// User pays with own Masterpass account
    MasterPass,
    /// User pays with own Papara account
    Papara,
    /// User pays withown Payoneer account
    Payoneer,
    /// User pays with own Sodexo card
    Sodexo,
    /// User pays with own Edenred card
    Edenred,
    /// User pays with own Edenred gift card
    EdenredGift,
    /// User pays with own PayPal account
    Paypal,
    /// User pays with own Afterpay account
    AfterPay,
    /// User pays with own Klarna account
    Klarna,
    /// User pays with own Stripe account
    Stripe,
}

/// Values expressing the Payout status
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutStatus {
    /// Money transfer will not be done
    NoPayout,
    /// Waiting for money transfer
    WaitingForPayout,
    /// Money transfer started
    PayoutStarted,
    /// Money transfer completed
    PayoutCompleted,
}

/// Values expressing Payout Types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutType {
    /// Represents money transfers due to Payment/Cancellation/Return transactions
    Settlement,
    /// Represents a money transfer to resend transactions that have been refunded for some reason
    /// after the money has been sent
    BouncedSettlement,
    /// Represents money transfers based on sub-merchants' wallet balance withdrawal requests
    Withdraw,
}

/// Values expressing the Payout Source
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutSource {
    /// Represents money transfers due to Payment/Cancellation/Return transactions
    Collection,
    /// Represents a money transfer to resend transactions that have been refunded for some reason
    /// after the money has been sent
    Bounced,
    /// Represents money transfers based on your sub-merchants' wallet balance withdrawal requests
    Withdraw,
}

/// These are the numerical values transmitted to Craftgate by the bank after verification for
/// payments made with 3D Secure. Although banks and payment institutions sometimes return with
/// their unique MD Status figures, the values in the table below can be accepted as standard.
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum MdStatus {
    /// 3D Secure verification or signature invalid
    VerificationOrSignatureInvalid = 0,
    /// Full Verification, processing can be continued
    FullVerification = 1,
    /// Card holder or bank not registered in the system
    CardholderOrBankNotRegistered = 2,
    /// The bank of the card is not registered in the system
    BankNotRegistered = 3,
    /// Verification attempt, cardholder has chosen to register later in the system
    VerificationAttempt = 4,
    /// Unable to verify
    UnableToVerify = 5,
    /// 3D Secure error
    Error = 6,
    /// System error
    SystemError = 7,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardAssociation {
    Visa,
    MasterCard,
    Amex,
    Troy,
    Jcb,
    UnionPay,
    Maestro,
    Discover,
    DinersClub,
}

/// Indicated which type of integration a payment is coming from
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentSource {
    /// via API
    Api,
    /// via Masterpass
    Masterpass,
    /// via Common Payment Form
    CheckoutForm,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct PaymentItem {
    pub name: Option<String>,
    #[builder(!default)]
    pub price: Decimal,
    pub external_id: Option<String>,
    pub sub_merchant_member_id: Option<String>,
    pub sub_merchant_member_price: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerchantPos {
    /// ID of the POS
    pub id: u64,
    /// Name of the POS
    pub name: String,
    /// Alias of the POS
    pub alias: String,
    /// Bank of the POS
    pub bank_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyType {
    RewardMoney,
    AdditionalInstallment,
    PostponingInstallment,
    ExtraPoints,
    GainingMinutes,
    PostponingStatement,
}

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub card_reward_money: Decimal,
    pub firm_reward_money: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder)]
pub struct Loyalty {
    #[serde(rename = "type")]
    pub loyalty_type: LoyaltyType,
    #[builder(default)]
    pub reward: Option<Reward>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FraudAction {
    Block,
    Review,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    WaitingForApproval,
    Approved,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    /// ID of the payment.
    pub id: u64,
    /// The date that payment is created
    #[serde(with = "crate::request::craftgate")]
    pub created_date: PrimitiveDateTime,
    /// Basket price of the payment
    pub price: Decimal,
    /// Paid price of the payment
    pub paid_price: Decimal,
    /// Wallet price of the payment
    pub wallet_price: Decimal,
    pub currency: Currency,
    /// Buyer member ID of the payment
    pub buyer_member_id: Option<u64>,
    /// Installment number
    pub installment: u64,
    /// Value of the `conversation_id` parameter sent in the payment request
    pub conversation_id: String,
    /// `external_id` value that sent in payment request by merchant
    pub external_id: Option<String>,
    pub payment_type: PaymentType,
    pub payment_group: PaymentGroup,
    pub payment_source: PaymentSource,
    pub payment_status: PaymentStatus,
    pub payment_phase: PaymentPhase,
    /// `payment_channel` value sent by the merchant in the payment request
    pub payment_channel: Option<String>,
    /// Indicates whether the payment is 3D Secure payment
    pub is_three_d_s: bool,
    /// Interest rate to calculate paid price that buyer will pay
    pub merchant_commission_rate: Decimal,
    /// Interest amount. Equal to the difference between `paid_price` and `price` values
    pub merchant_commission_rate_amount: Decimal,
    /// Bank commission rate
    pub bank_commission_rate: Decimal,
    /// Bank commission rate amount
    pub bank_commission_rate_amount: Decimal,
    /// Card user key that represents the card holder.
    pub card_user_key: Option<String>,
    /// Card token that represents the card.
    pub card_token: Option<String>,
    /// Indicates whether the payment was made with a stored card
    pub paid_with_stored_card: bool,
    /// Exist for the payments fully or partially collected from the card. First 8 digits of the card
    pub bin_number: String,
    /// Exist for the payments fully or partially collected from the card. Last 4 digits of the card
    pub last_four_digits: String,
    /// Card holder name surname
    pub card_holder_name: String,
    /// Card holder name surname retrieved from bank. The return value might be null for PF poses
    pub bank_card_holder_name: String,
    /// authCode value retrieved from bank related to the payment. The return value might be null for PF poses
    pub auth_code: Option<String>,
    /// hostReference value retrieved from bank related to the payment
    pub host_reference: String,
    /// transId value retrieved from bank related to the payment. The return value might be null for PF poses
    pub trans_id: Option<String>,
    /// orderId value retrieved from bank related to the payment
    pub order_id: String,
    pub card_type: CardType,
    pub card_association: CardAssociation,
    /// Brand of card. Exist for the payments fully or partially collected from the card
    pub card_brand: Option<String>,
    /// Value of the `pos_alias` parameter sent in the payment request
    pub requested_pos_alias: Option<String>,
    /// POS info that payment is received from
    pub pos: MerchantPos,
    /// Loyalty info that used in payment
    pub loyalty: Option<Loyalty>,
    /// Fraud Check if fraud rule matches
    pub fraud_id: Option<u64>,
    /// Fraud Check Action if fraud rule matches
    pub fraud_action: Option<FraudAction>,
    /// It includes the transaction information sent when making the payment request, and the
    /// pricing and money transfer information of the payment based on these transactions
    pub payment_transactions: Vec<PaymentTransaction>,
    /// Additional data related to the payment
    pub additional_data: Option<AdditionalData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransaction {
    /// ID of the payment transaction
    pub id: u64,
    /// External ID of the payment item that is sent in the request
    pub external_id: Option<String>,
    /// Name of the payment item that is sent in the request
    pub name: Option<String>,
    /// Basket price of the payment item
    pub price: Decimal,
    /// Paid price of the payment item
    pub paid_price: Decimal,
    /// Wallet price of the payment item
    pub wallet_price: Decimal,
    /// Merchant's interest rate of the payment transaction
    pub merchant_commission_rate: Decimal,
    /// Merchant's interest rate amount of the payment transaction
    pub merchant_commission_rate_amount: Decimal,
    /// Merchant's payout amount of the payment transaction
    pub merchant_payout_amount: Decimal,
    /// ID of the sub-merchant
    pub sub_merchant_member_id: Option<u64>,
    /// Sub-merchant requested payout amount for the payment item.
    pub sub_merchant_member_price: Decimal,
    /// Sub-merchant payout rate for the payment item.
    pub sub_merchant_member_payout_rate: Decimal,
    /// Sub-merchant payout amount for the payment item.
    pub sub_merchant_member_payout_amount: Decimal,
    /// Status of payment transaction confirmation
    pub transaction_status: TransactionStatus,
    /// Date that the blockage will be resolved
    #[serde(with = "crate::request::craftgate")]
    pub blockage_resolved_date: PrimitiveDateTime,
    /// It includes the distribution of the money transfer to be made in relation to the payment,
    /// in a transaction specific, between the merchant and the sub-merchant. The currency of the
    /// money distribution is always `Currency::Try`
    pub payout: Payout,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payout {
    /// Total paid price of the payment transaction
    pub paid_price: Decimal,
    /// Foreign currency rate, for TRY payment it would be 1.
    pub parity: Decimal,
    pub currency: Currency,
    /// Payout amount of the merchant
    pub merchant_payout_amount: Decimal,
    /// Payout amount of the sub-merchant
    pub sub_merchant_member_payout_amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalData {
    /// This parameter is valid for merchants that have an additional installment agreement with
    /// Garanti bank. It contains the URL to be shown to the buyer as a result of installment
    /// transactions to make additional installments
    pub campaign_url: Option<String>,
    #[serde(flatten)]
    pub rest: serde_json::Value,
}
