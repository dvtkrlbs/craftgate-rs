use std::{fmt};

use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    #[serde(rename = "errorCode")]
    pub code: ErrorCode,
    #[serde(rename = "errorDescription")]
    pub description: String,
    #[serde(rename = "errorGroup")]
    pub group: Option<ErrorGroup>,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ErrorCode(pub(crate) u32);

impl AsRef<u32> for ErrorCode {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize_enum_str, Serialize_enum_str, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorGroup {
    /// American Express card error. An error occurred while making payment transaction with an
    /// American Express card.
    AmexCanUseOnlyMr,
    /// APM Payment declined
    ApmError,
    /// Pre-approved transaction. The order number (orderId) was previously used during another
    /// successful checkout. lSuccessful payment transaction should be checked, if you think that
    /// the payment has not been made, the payment should be tried again with a new order number.
    ApprovedCompleted,
    /// BIN not found. The first 8 digits of the card number identify the BIN number. A valid BIN
    /// number could not be found from the entered card number. Buyer should check the card number
    /// and try to pay again.
    BinNotFound,
    /// Card is blocked. Card is blocked. Buyer should contact to the bank.
    BlockedCard,
    /// The card does not allow the transaction. No transaction can be made with this card. Payment
    /// can be made by trying a different card information.
    CardNotPermitted,
    /// Communication or system error. Bank could not respond within the expected time, there may
    /// have been a temporary interruption.
    CommunicationOrSystemError,
    /// CVC2 incorrect entry attempts exceeded. The security code has been entered incorrectly
    /// repeatedly and the payment was rejected because the number of incorrect payment attempts was
    /// exceeded.
    Cvc2MaxAttempt,
    /// CVC is required. Payment cannot be made through the POS without the CVC security code. The
    /// payment should be tried again, making sure that the security code information is entered.
    CvcRequired,
    /// Installments cannot be made with debit cards. Installment cannot be made with debit cards.
    /// In order for the payment to be successful, payment should be done with single installment.
    DebitCardsInstallmentNotAllowed,
    /// Debit cards can only be used in 3D Secure transaction. With debit cards, 3D Secure payment
    /// should be done. If buyer has entered a debit card as the card number, it should be directed
    /// to 3D Secure payment process.
    DebitCardsRequires3ds,
    /// Payment declined. Payment transaction is declined. The cardholder should contact to the bank.
    Declined,
    /// The transaction has not been approved. Please try with another card. Bank of card rejected
    /// the payment transaction. Daily transaction limit/count may be exceeded or an error may have
    /// been received because many attempts were made. Buyer should contact to the bank.
    DoNotHonour,
    /// Allowed number of PIN entries exceeded. Max PIN tries exceeded.
    ExceedsAllowablePinTries,
    /// Withdrawal limit exceeded.The number of daily transactions or daily limit may have been
    /// exceeded.
    ExceedsWithdrawalAmountLimit,
    /// Incorrect expiration date. The expiry date of the card is incorrect. Buyer can try to pay
    /// again by correcting the expiry date.
    ExpiredCard,
    /// Payment blocked due to fraud check rules. Payment blocked due to fraud check rules.
    FraudCheckBlock,
    /// The payment fails to pass the security check. Bank of card rejected the payment
    /// transaction for security reasons. There is a suspicion of fraud or an error may have been
    /// received because many attempts were made. The payment transaction detail should be checked
    /// and payment may be requested to be tried again by contacting the buyer.
    FraudSuspect,
    /// Invalid amount. Invalid amount, check the payment amount. The number of daily transactions
    /// or daily limit may have been exceeded. Please contact to the bank for detailed information.
    InvalidAmount,
    /// Invalid card number. Card number not accepted by the bank. The card number should be checked
    /// and the payment should be tried again.
    InvalidCardNumber,
    /// Invalid card type. Invalid card type, please check card number. Please contact to the bank
    /// for detailed information.
    InvalidCardType,
    /// Invalid CAVV information. Incorrect CAVV information. Buyer should contact to the bank.
    InvalidCavv,
    /// Email is not in valid format. Email is invalid. Email information should be checked.
    InvalidCharsInEmail,
    /// CVC length is invalid. The security code (CVC2) of the card is incorrect. Buyer can try to
    /// pay again by correcting the security code.
    InvalidCvc2Length,
    /// Cvc2 information is invalid. The length of the security code (CVC) entered with the card is
    /// invalid. Buyer can try to pay again by correcting the security code.
    InvalidCvc2,
    /// Invalid ECI information. Incorrect ECI. Buyer should contact to the bank.
    InvalidEci,
    /// Invalid expiration date. The expiry date of the card entered is invalid. Buyer should check
    /// the card information entered.
    InvalidExpireYearMonth,
    /// Invalid IP. If IP definition is mandatory in Virtual POS, please make defined Craftgate IP
    /// addresses by contacting to the bank.
    InvalidIp,
    /// Merchant category code is incorrect. Merchant category is incorrect. It is necessary to make
    /// a correction to merchant category by contacting the bank.
    InvalidMerchantOrSp,
    ///Invalid PIN. Invalid PIN. Buyer should contact to the bank.
    InvalidPin,
    /// Invalid transaction. Please try with another card. Invalid transaction. Please check the
    /// payment transaction detail, buyer should contact to the bank for detailed information.
    InvalidTransaction,
    /// Bank or terminal fail to process. The bank or POS cannot perform any payment transaction, a
    /// temporary interruption may have occurred.
    IssuerOrSwitchInoperative,
    /// Lost card, pickup the card. Lost card. Transactions cannot be made with this card. Payment
    /// attempt may be suspected of fraud.
    LostCard,
    /// The amount to be refunded must be less than the total paid amount. This payment may have
    /// been previously refunded. The amount to be refunded must be less than the sales amount.
    /// This payment may have been previously refunded.
    MayHaveAlreadyRefunded,
    /// The cardholder cannot do this transaction. Please try with another card. No transaction can
    /// be made with this card. Payment can be made by trying a different card information.
    NotPermittedToCardholder,
    /// Terminal is closed to international cards. POS has not authorization for international card
    /// transactions. POS authorization should be checked.
    NotPermittedToForeignCard,
    /// Terminal does not allow installment. POS has not authorization for installments.
    /// POS authorization should be checked.
    NotPermittedToInstallment,
    /// The terminal is not authorized to perform this transaction. POS has not authorization for
    /// this operation. POS authorization should be checked.
    NotPermittedToTerminal,
    ///     Insufficient award points. The reward point to be used is higher than the point amount
    /// belong to the card. Since there is not enough reward point, the amount of point should be
    /// updated and payment should be tried again.
    NotSufficientAward,
    ///     Insufficient card limit, insufficient balance. Buyer’s card does not have sufficient balance
    /// for payment. Buyer should contact to the bank.
    NotSufficientFunds,
    ///     A general error occurred during the payment process. Bank could not respond within the
    /// expected time, there may have been a temporary interruption.
    NoResponse,
    ///     Bank not found. Bank of card not found. The card number should be checked and the payment
    /// should be tried again.
    NoSuchIssuer,
    ///     The order number (orderId) has already been used. Order numbers must be unique for
    /// successful sales. The order number (orderId) was previously used during another successful
    /// checkout. Successful payment transaction should be checked, if you think that the payment
    /// has not been made, the payment should be tried again with a new order number.
    OrderIdAlreadyUsed,
    ///     Pickup the card     Bank of card refused the payment due to security/fraud suspicion. Payment
    /// transaction cannot be made with this card.
    PickupCard,
    ///     Virtual Pos balance is not sufficient. Virtual Pos balance is not sufficient.
    PosBalanceNotSufficient,
    ///     Get approval from your bank     Bank of card did not approve the payment. Buyer should contact
    /// to the bank for manual confirmation.
    ReferToCardIssuer,
    ///     The request received an error from the bank     Payment request is blocked by bank.
    RequestBlockedByBank,
    ///     Request sent to bank timed out     Bank could not respond within the expected time, there may
    /// have been a temporary interruption.
    RequestTimeout,
    ///     Cash up must be done. With the POS, this transaction cannot be done before the end of the
    /// day operation.
    RequiresDayEnd,
    ///     Your card is closed to e-commerce transactions. Call your bank.     The card is not permitted to
    /// e-commerce payments. Buyer should contact to the bank and authorize this card for e-commerce
    /// payment transactions.
    RestrictedByLaw,
    ///     Restricted card     The payment was not completed due to a restriction. Buyer can try again with
    /// different card information.
    RestrictedCard,
    ///     The sales amount cannot be lower than the award points. The reward points to be used cannot
    /// be higher than the payment amount. The point amount to be used should be updated to be equal
    /// to or less than the payment amount.
    SalesAmountLessThanAward,
    ///     Stolen card, pickup the card. Stolen card. Transactions cannot be made with this card.
    /// Payment attempt may be suspected of fraud.
    StolenCard,
    ///     3D Secure payment cannot be initialized     3D Secure init request is failed, contact to the
    /// bank for more details.
    ThreedsInitError,
    ///     An error occurred during the payment process. Payment failed, error group not detected.
    Unknown,
    #[serde(other)]
    Unhandled(String),
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ErrorCodeVisitor;

        impl<'de> Visitor<'de> for ErrorCodeVisitor {
            type Value = ErrorCode;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u32<E>(self, val: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ErrorCode(val))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u32>() {
                    Ok(val) => self.visit_u32(val),
                    Err(_) => Err(E::custom("failed to parse integer")),
                }
            }
        }

        deserializer.deserialize_any(ErrorCodeVisitor)
    }
}
