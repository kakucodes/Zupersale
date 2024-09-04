pub mod contract;
mod error;
pub mod execute;
pub mod msg;
pub mod queries;
pub mod state;
pub mod utils;

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
mod interface;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::ZupersaleCampaign;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::msg::{
    ExecuteMsgFns as ZupersaleCampaignExecuteMsgFns, QueryMsgFns as ZupersaleCampaignQueryMsgFns,
};

pub use crate::error::ContractError;
