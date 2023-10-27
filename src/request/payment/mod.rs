use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod init_payment;

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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentGroup {
    /// Product
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentPhase {
    /// In standard provision transaction, the payment is set as this status value
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInitiationParameters {
    pub conversation_id: Option<String>,
    pub external_id: Option<String>,
    pub bank_order_id: Option<String>,
    pub price: Decimal,
    pub paid_price: Decimal,
    pub buyer_member_id: Option<u32>,
}
