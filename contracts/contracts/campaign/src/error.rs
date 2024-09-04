use std::num::TryFromIntError;

use cosmwasm_std::{Coin, Decimal, OverflowError, StdError, Timestamp, Uint64};
use cw_utils::PaymentError;
use thiserror::Error;

use crate::msg::CampaignStatus;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Overflow error {0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unimplemented")]
    Unimplemented,

    #[error("Campaign duration invalid")]
    InvalidCampaignDuration,

    #[error("Invalid admin address {0}")]
    InvalidAdminAddress(String),

    #[error("Invalid user address {0}")]
    InvalidUserAddress(String),

    #[error("Invalid start timestamp {start_timestamp}. Current timestamp: {current_timestamp}")]
    InvalidStartTimestamp {
        start_timestamp: Timestamp,
        current_timestamp: Timestamp,
    },

    #[error("Expiration for {expiration_usage} cannot be earlier than the current time {current_timestamp}. Given timestamp: {given_timestamp}")]
    InvalidExpiration {
        expiration_usage: String,
        given_timestamp: Timestamp,
        current_timestamp: Timestamp,
    },

    #[error("No previous settings found. User: {0}, Strategy Id: {1}")]
    NoSettingsFound(String, Uint64),

    #[error("Invalid incentive {0}")]
    InvalidIncentive(String),

    #[error("Invalid withdrawl fee {0}")]
    InvalidWithdrawlFee(Decimal),

    #[error("Payment Error: {0}")]
    PaymentError(#[from] PaymentError),

    #[error("Invalid Int Conversion: {0}")]
    CoercionError(#[from] TryFromIntError),

    #[error("Cannot withdraw 0 tokens")]
    ZeroWithdrawl,

    #[error("Invalid Withdrawl Amount. Requested: {0}, Available: {1}")]
    InvalidWithdrawlAmount(Uint64, Uint64),

    #[error("Cannot cancel a campaign that has already started")]
    CampaignAlreadyStarted,

    #[error("Campaign not yet active. Deposits will start at {0}")]
    CampaignNotYetActive(Timestamp),

    #[error("Cannot change campaign state from {0} to {1}")]
    InvalidStateTransition(CampaignStatus, CampaignStatus),

    #[error("Unauthorized state transition from {0} to {1}")]
    UnauthorizedState(CampaignStatus, CampaignStatus),

    #[error("Campaign creation fee not met. Expected {0}, received {1}")]
    InsufficientCreationFee(String, String),

    #[error("Funds missmatch. Expected {expected}, received {received}")]
    FundsMissmatch { expected: Coin, received: Coin },

    #[error("Unexpected denom. Did not expect to receive {0}")]
    UnexpectedDenom(String),
}
