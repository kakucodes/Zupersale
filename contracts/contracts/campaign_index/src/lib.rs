pub mod contract;
mod error;
pub mod msg;
pub mod state;
pub mod utils;

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
mod interface;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::ZupersaleIndex;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::msg::{
    ExecuteMsgFns as ZupersaleIndexExecuteMsgFns, QueryMsgFns as ZupersaleIndexQueryMsgFns,
};

pub use crate::error::ContractError;
