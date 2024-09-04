use campaign::msg::{
    CampaignMetadata, CampaignStatus, ProtocolInstantiateSettings, ValidatedCampaignMetadata,
};
use campaign::utils::{funds_match_balance, required_incentives_tokens};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, CodeInfoResponse, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdError, Uint64, WasmMsg,
};
use cw_storage_plus::Bound;
use cw_utils::{must_pay, nonpayable};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    CampaignType, CampaignsResponse, ExecuteMsg, IndexSettings, InstantiateMsg, MigrateMsg,
    QueryMsg, WhitelistEntry,
};
use crate::state::{
    CAMPAIGN_CREATION_COUNT, MAIN_ARCHIVED_CAMPAIGNS, MAIN_CAMPAIGNS, STORE_SETTINGS,
    TEST_ARCHIVED_CAMPAIGNS, TEST_CAMPAIGNS, WHITELISTED_ADDRESSES,
};
use crate::utils::{instantiate_campaign, may_instantiate_campaign};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:zupersale-contract-index";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    InstantiateMsg {
        admin,
        enforce_whitelist,
        campaign_contract_code_id,
        campaign_funds_withdrawl_fee,
        creation_fee,
        treasury_address,
    }: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Ensure that the admin address is valid
    let admin_addr = admin.as_ref().map_or(Ok(info.sender), |admin| {
        deps.api
            .addr_validate(admin)
            .map_err(|_| ContractError::InvalidAdminAddress(admin.clone()))
    })?;

    STORE_SETTINGS.save(
        deps.storage,
        &IndexSettings {
            admin: admin_addr,
            enforce_whitelist,
            campaign_contract_code_id,
            campaign_funds_withdrawl_fee,
            creation_fee,
            treasury_address: deps.api.addr_validate(&treasury_address)?,
        },
    )?;

    CAMPAIGN_CREATION_COUNT.save(deps.storage, &0)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // ensure that the sender is the admin
    match msg {
        ExecuteMsg::EnforceWhitelist(_)
        | ExecuteMsg::GrantWhitelist { .. }
        | ExecuteMsg::RevokeWhitelist(_)
        | ExecuteMsg::DeleteCampaign { .. }
        | ExecuteMsg::SetCampaignContractCodeId(_)
        | ExecuteMsg::SetAdmin(_)
        | ExecuteMsg::MigrateCampaigns { .. } => {
            if STORE_SETTINGS.load(deps.storage)?.admin.ne(&info.sender) {
                return Err(ContractError::Unauthorized);
            }
            Ok::<(), ContractError>(())
        }
        _ => Ok(()),
    }?;

    // ensure that the sender has sent correct funds
    match &msg {
        ExecuteMsg::CreateCampaign {
            campaign_metadata:
                CampaignMetadata {
                    test_campaign,
                    incentives,
                    ..
                },
            ..
        } => {
            let IndexSettings { creation_fee, .. } = STORE_SETTINGS.load(deps.storage)?;

            // calculate the required fundds between the creation fee and the incentive tokens that will be distributed to donors
            let required_payment = required_incentives_tokens(incentives);
            let required_payment = if test_campaign.clone() {
                required_payment
            } else {
                required_payment + creation_fee
            };

            Ok::<(), ContractError>(funds_match_balance(&required_payment, &info.funds)?)
        }
        _ => Ok(nonpayable(&info)?),
    }?;

    match msg {
        ExecuteMsg::MigrateCampaigns {
            code_id,
            msg,
            limit,
            start_after,
        } => {
            let start_after_addr = match start_after {
                Some(addr) => Some(deps.api.addr_validate(&addr)?),
                None => None,
            };

            let migration_msgs = MAIN_CAMPAIGNS
                .keys(
                    deps.storage,
                    start_after_addr.as_ref().map(Bound::exclusive),
                    None,
                    Order::Ascending,
                )
                .take(limit.unwrap_or(30) as usize)
                .map(|item| {
                    let campaign_addr = item?;
                    Ok(CosmosMsg::Wasm(WasmMsg::Migrate {
                        contract_addr: campaign_addr.to_string(),
                        new_code_id: code_id.u64(),
                        msg: msg.clone(),
                    }))
                })
                .collect::<Result<Vec<CosmosMsg>, ContractError>>()?;

            STORE_SETTINGS.update(deps.storage, |mut settings| {
                settings.campaign_contract_code_id = code_id.u64();
                Ok::<IndexSettings, ContractError>(settings)
            })?;

            Ok(Response::new().add_messages(migration_msgs))
        }
        ExecuteMsg::SetAdmin(admin) => {
            let mut settings = STORE_SETTINGS.load(deps.storage)?;
            settings.admin = deps
                .api
                .addr_validate(&admin)
                .map_err(|_| ContractError::InvalidAdminAddress(admin))?;
            STORE_SETTINGS.save(deps.storage, &settings)?;
            Ok(Response::new())
        }
        ExecuteMsg::EnforceWhitelist(enforce) => {
            let mut settings = STORE_SETTINGS.load(deps.storage)?;
            settings.enforce_whitelist = enforce;
            STORE_SETTINGS.save(deps.storage, &settings)?;
            Ok(Response::new())
        }
        ExecuteMsg::SetCampaignContractCodeId(code_id) => {
            let mut settings = STORE_SETTINGS.load(deps.storage)?;
            settings.campaign_contract_code_id = code_id.into();
            STORE_SETTINGS.save(deps.storage, &settings)?;
            Ok(Response::new())
        }
        ExecuteMsg::GrantWhitelist {
            address,
            test_creation_only,
        } => {
            let addr = deps
                .api
                .addr_validate(&address)
                .map_err(|_| ContractError::InvalidUserAddress(address.clone()))?;
            WHITELISTED_ADDRESSES.save(deps.storage, &addr, &test_creation_only)?;
            Ok(Response::new())
        }
        ExecuteMsg::RevokeWhitelist(address) => {
            let addr = deps
                .api
                .addr_validate(&address)
                .map_err(|_| ContractError::InvalidUserAddress(address.clone()))?;
            WHITELISTED_ADDRESSES.remove(deps.storage, &addr);
            Ok(Response::new())
        }
        ExecuteMsg::ArchiveCampaign => {
            if TEST_CAMPAIGNS.has(deps.storage, &info.sender) {
                let campaign = TEST_CAMPAIGNS.load(deps.storage, &info.sender)?;
                TEST_CAMPAIGNS.remove(deps.storage, &info.sender);
                TEST_ARCHIVED_CAMPAIGNS.save(deps.storage, &info.sender, &campaign)?;
            } else if MAIN_CAMPAIGNS.has(deps.storage, &info.sender) {
                let campaign = MAIN_CAMPAIGNS.load(deps.storage, &info.sender)?;
                MAIN_CAMPAIGNS.remove(deps.storage, &info.sender);
                MAIN_ARCHIVED_CAMPAIGNS.save(deps.storage, &info.sender, &campaign)?;
            } else {
                return Err(ContractError::CampaignNotFound(info.sender.to_string()));
            }

            Ok(Response::new())
        }
        ExecuteMsg::DeleteCampaign { address } => {
            let campaign_addr = deps
                .api
                .addr_validate(&address)
                .map_err(|_| ContractError::CampaignNotFound(address.clone()))?;

            // remove the campaign from the main or test campaigns map
            MAIN_CAMPAIGNS.remove(deps.storage, &campaign_addr);
            MAIN_ARCHIVED_CAMPAIGNS.remove(deps.storage, &campaign_addr);

            TEST_CAMPAIGNS.remove(deps.storage, &campaign_addr);
            TEST_ARCHIVED_CAMPAIGNS.remove(deps.storage, &campaign_addr);

            Ok(Response::new())
        }
        ExecuteMsg::CreateCampaign { campaign_metadata } => {
            let IndexSettings {
                admin,
                campaign_contract_code_id: code_id,
                enforce_whitelist,
                campaign_funds_withdrawl_fee,
                treasury_address,
                creation_fee,
                ..
            } = STORE_SETTINGS.load(deps.storage)?;

            let campaign_creation_count = CAMPAIGN_CREATION_COUNT
                .update(deps.storage, |prev| Ok::<u64, ContractError>(prev + 1u64))?;

            let CampaignMetadata {
                test_campaign: creating_test_campaign,
                ..
            } = &campaign_metadata;

            may_instantiate_campaign(
                deps.as_ref(),
                &enforce_whitelist,
                &info.sender,
                creating_test_campaign,
                &admin,
            )?;

            let (instantiate_msg, campaign_addr) = instantiate_campaign(
                deps.as_ref(),
                &code_id,
                &env.contract.address,
                &campaign_creation_count,
                campaign_metadata.clone(),
                &campaign_funds_withdrawl_fee,
                &treasury_address,
            )?;

            // store the campaign address in the appropriate map
            if creating_test_campaign.clone() {
                TEST_CAMPAIGNS.save(deps.storage, &campaign_addr, &())?;
            } else {
                MAIN_CAMPAIGNS.save(deps.storage, &campaign_addr, &())?;
            };

            let mut response = Response::new()
                .add_message(instantiate_msg)
                .add_attribute("create_campaign", campaign_addr);

            // we only send the funds to the treasury if this is not a test campaign
            if !creating_test_campaign {
                response = response.clone().add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: treasury_address.into_string(),
                    amount: vec![creation_fee],
                }));
            }

            Ok(response)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Settings => Ok(to_json_binary(&STORE_SETTINGS.load(deps.storage)?)?),
        QueryMsg::CreatorWhitelist { limit, start_after } => {
            let limit = limit.unwrap_or(30);
            let start_after_addr = match start_after {
                Some(addr) => Some(deps.api.addr_validate(&addr)?),
                None => None,
            };

            let whitelist = WHITELISTED_ADDRESSES
                .range(
                    deps.storage,
                    start_after_addr.as_ref().map(Bound::exclusive),
                    None,
                    Order::Ascending,
                )
                .take(limit as usize)
                .map(|item| {
                    let (addr, test_creation_only) = item?;
                    Ok(WhitelistEntry {
                        address: addr,
                        test_creation_only,
                    })
                })
                .collect::<Result<Vec<WhitelistEntry>, ContractError>>()?;
            Ok(to_json_binary(&whitelist)?)
        }
        QueryMsg::Campaigns {
            campaign_type,
            campaign_status: _campaign_status,
            limit,
            start_after,
        } => {
            let limit = limit.unwrap_or(30);
            let start_after_addr = match start_after {
                Some(addr) => Some(deps.api.addr_validate(&addr)?),
                None => None,
            };

            let campaigns_map = match campaign_type {
                CampaignType::Main => MAIN_CAMPAIGNS,
                CampaignType::Test => TEST_CAMPAIGNS,
                CampaignType::MainArchived => MAIN_ARCHIVED_CAMPAIGNS,
                CampaignType::TestArchived => TEST_ARCHIVED_CAMPAIGNS,
            };

            let campaigns = campaigns_map
                .range(
                    deps.storage,
                    start_after_addr.as_ref().map(Bound::exclusive),
                    None,
                    Order::Ascending,
                )
                .take(limit as usize)
                .map(|item| -> Result<CampaignsResponse, ContractError> {
                    let (campaign_addr, _) = item?;
                    let metadata: ValidatedCampaignMetadata = deps.querier.query_wasm_smart(
                        campaign_addr.clone(),
                        &campaign::msg::QueryMsg::CampaignMetadata,
                    )?;

                    let status: CampaignStatus = deps.querier.query_wasm_smart(
                        campaign_addr.clone(),
                        &campaign::msg::QueryMsg::CampaignStatus,
                    )?;

                    let total_donations: Coin = deps.querier.query_wasm_smart(
                        campaign_addr.clone(),
                        &campaign::msg::QueryMsg::TotalDonations,
                    )?;

                    let total_donors: Uint64 = deps.querier.query_wasm_smart(
                        campaign_addr.clone(),
                        &campaign::msg::QueryMsg::TotalDonors,
                    )?;

                    Ok(CampaignsResponse {
                        campaign_addr,
                        metadata,
                        status,
                        total_donations,
                        total_donors,
                    })
                })
                .collect::<Result<Vec<CampaignsResponse>, ContractError>>()?;
            Ok(to_json_binary(&campaigns)?)
        }
    }
}
