use campaign::msg::{CampaignMetadata, Incentives, ProtocolInstantiateSettings, Reward};
use cosmwasm_std::{
    instantiate2_address, to_json_binary, Addr, Binary, CodeInfoResponse, Coin, CosmosMsg, Decimal,
    Deps, WasmMsg,
};
use cw_utils::NativeBalance;
use sha2::{Digest, Sha256};

use crate::{state::WHITELISTED_ADDRESSES, ContractError};

/// Checks if the sender is allowed to create a campaign
pub fn may_instantiate_campaign(
    deps: Deps,
    enforce_whitelist: &bool,
    sender: &Addr,
    creating_test_campaign: &bool,
    index_admin: &Addr,
) -> Result<(), ContractError> {
    // Error out if not allowed to make a campaign
    match (enforce_whitelist, sender, creating_test_campaign) {
        // if we aren't enforcing the whitelist, anyone can create a campaign
        (false, _, _) => Ok::<(), ContractError>(()),
        // the index admin may always create campaigns
        (_, sender, _) if sender.eq(index_admin) => Ok(()),
        // if a test campaign is being created and the sender is whitelisted in any manner, they can create a campaign
        (_, sender, true) if WHITELISTED_ADDRESSES.has(deps.storage, sender) => Ok(()),
        // if a prod campaign is being created and the sender is whitelisted for prod/live campaigns, they can create a campaign
        (_, sender, false)
            if WHITELISTED_ADDRESSES
                .may_load(deps.storage, sender)
                .unwrap_or(Some(true))
                .unwrap_or(true)
                .eq(&false) =>
        {
            Ok(())
        }
        _ => {
            return Err(ContractError::Unauthorized);
        }
    }
}

/// Prepares the instantiate message for a campaign contract and returns the address of the campaign contract
pub fn instantiate_campaign(
    deps: Deps,
    campaign_code_id: &u64,
    instantiator: &Addr,
    campaign_creation_count: &u64,
    campaign_metadata: CampaignMetadata,
    campaign_funds_withdrawl_fee: &Decimal,
    treasury_address: &Addr,
) -> Result<(CosmosMsg, Addr), ContractError> {
    let CampaignMetadata {
        name: campaign_name,
        test_campaign,
        owner,
        ..
    } = &campaign_metadata;

    let campaign_admin_addr = deps
        .api
        .addr_validate(owner)
        .map_err(|_| ContractError::InvalidAdminAddress(owner.clone()))?;

    // Get the canonical address of the contract creator
    let canonical_creator = deps.api.addr_canonicalize(instantiator.as_str())?;

    // get the checksum of the contract we're going to instantiate
    let CodeInfoResponse { checksum, .. } = deps
        .querier
        .query_wasm_code_info(campaign_code_id.clone())?;

    // concatenate the campaign admin address, creation count, and whether or not this is a test campaign and convert it all to bytes
    let salt_str = format!(
        "{}{}{}",
        campaign_admin_addr, campaign_creation_count, test_campaign
    );

    let salt = Binary::from(sha256(salt_str.as_bytes()));

    let campaign_addr = instantiate2_address(&checksum, &canonical_creator, &salt)?;
    let campaign_addr = deps.api.addr_humanize(&campaign_addr)?;

    let instantiate_msg = CosmosMsg::Wasm(WasmMsg::Instantiate2 {
        admin: Some(instantiator.to_string()),
        code_id: *campaign_code_id,
        label: format!("zupersale-campaign-({})", campaign_name),

        msg: to_json_binary(&campaign::msg::InstantiateMsg {
            campaign_metadata: campaign_metadata.clone(),
            protocol_settings: ProtocolInstantiateSettings {
                withdrawl_fee: *campaign_funds_withdrawl_fee,
                fee_address: treasury_address.to_string(),
            },
        })?,
        funds: vec![],
        salt,
    });

    Ok((instantiate_msg, campaign_addr))
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}
