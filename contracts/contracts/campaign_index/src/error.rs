use campaign::ContractError as CampaignError;
use cosmwasm_std::{Coin, Instantiate2AddressError, StdError, Timestamp};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Payment Error: {0}")]
    PaymentError(#[from] PaymentError),

    #[error("Instantiate2 address error: {0}")]
    Instantiate2(#[from] Instantiate2AddressError),

    #[error("Campaign Error: {0}")]
    CampaignError(#[from] CampaignError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unimplemented")]
    Unimplemented,

    #[error("Invalid admin address {0}")]
    InvalidAdminAddress(String),

    #[error("Invalid user address {0}")]
    InvalidUserAddress(String),

    #[error("Campaign not found")]
    CampaignNotFound(String),

    #[error("Could not instantiate campaign address")]
    CampaignInstantiate,

    #[error("Funds missmatch. Expected {expected}, received {received}")]
    FundsMissmatch { expected: Coin, received: Coin },

    #[error("Unexpected denom. Did not expect to receive {0}")]
    UnexpectedDenom(String),
}
