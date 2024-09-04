use std::f32::consts::E;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, Timestamp,
    Uint128, Uint64,
};
use cw_storage_plus::Bound;
use cw_utils::{must_pay, nonpayable};
// use cw2::set_contract_version;

use crate::{
    error::ContractError,
    execute::{cancel_campaign, deposit, undeposit},
    msg::{
        CampaignStatus, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, UserDonationsResponse,
        ValidatedCampaignMetadata,
    },
    queries::{query_donations_by_size, query_donations_by_time},
    state::{
        CAMPAIGN_STATUS, DONATIONS_BY_ADDR, DONATION_TOTALS, METADATA, TOTAL_CAMPAIGN_DONATIONS,
    },
    utils::{funds_match_balance, required_incentives_tokens},
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:zupersale-campaign";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    InstantiateMsg {
        campaign_metadata,
        protocol_settings,
    }: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    METADATA.save(
        deps.storage,
        &campaign_metadata.validate(deps.api, env.block.time, &info.sender, &protocol_settings)?,
    )?;

    CAMPAIGN_STATUS.save(deps.storage, &CampaignStatus::default())?;

    TOTAL_CAMPAIGN_DONATIONS.save(deps.storage, &Uint128::zero())?;

    // ensure that the sender has sent correct funds
    funds_match_balance(
        &required_incentives_tokens(&campaign_metadata.incentives),
        &info.funds,
    )?;

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
        ExecuteMsg::SetOwner { .. } | ExecuteMsg::CancelCampaign => {
            if METADATA.load(deps.storage)?.owner.ne(&info.sender) {
                return Err(ContractError::Unauthorized);
            }
            Ok::<(), ContractError>(())
        }
        _ => Ok(()),
    }?;

    let ValidatedCampaignMetadata { accepted_denom, .. } = METADATA.load(deps.storage)?;

    // ensure that the sender has sent correct funds
    match &msg {
        ExecuteMsg::Deposit { .. } => {
            Ok::<(), ContractError>(must_pay(&info, &accepted_denom).map(|_| ())?)
        }
        _ => Ok(nonpayable(&info)?),
    }?;

    match msg {
        ExecuteMsg::SetOwner { owner } => {
            METADATA.update(deps.storage, |mut metadata| {
                metadata.owner = deps.api.addr_validate(&owner)?;
                Ok::<ValidatedCampaignMetadata, ContractError>(metadata)
            })?;
            Ok(Response::new())
        }
        ExecuteMsg::Deposit { donor_address } => deposit(
            deps,
            &env,
            &info,
            &donor_address,
            must_pay(&info, &accepted_denom).unwrap(),
        ),
        ExecuteMsg::UnDonate => undeposit(deps, env, info, &accepted_denom),
        ExecuteMsg::CancelCampaign => cancel_campaign(deps, env, info),
        ExecuteMsg::CompleteCampaign {} => todo!(),
        ExecuteMsg::DistributeRewards { limit } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::CampaignMetadata => Ok(to_json_binary(&METADATA.load(deps.storage)?)?),
        QueryMsg::CampaignStatus => Ok(to_json_binary(&CAMPAIGN_STATUS.load(deps.storage)?)?),
        QueryMsg::DonationsBySize { limit, start_after } => Ok(to_json_binary(
            &query_donations_by_size(deps, env, limit, start_after)?,
        )?),
        QueryMsg::DonationsByTime {
            ascending,
            limit,
            start_after,
        } => Ok(to_json_binary(&query_donations_by_time(
            deps,
            env,
            if ascending {
                Order::Ascending
            } else {
                Order::Descending
            },
            limit,
            start_after,
        )?)?),
        QueryMsg::TotalDonors => {
            let total_donors = DONATION_TOTALS
                .range(deps.storage, None, None, Order::Ascending)
                .count();
            Ok(to_json_binary(&Uint64::from(total_donors as u64))?)
        }
        QueryMsg::TotalDonations => {
            let total_donations = TOTAL_CAMPAIGN_DONATIONS.load(deps.storage)?;
            let ValidatedCampaignMetadata { accepted_denom, .. } = METADATA.load(deps.storage)?;

            Ok(to_json_binary(&coin(
                total_donations.u128().into(),
                &accepted_denom,
            ))?)
        }
        QueryMsg::UserDonation { address } => {
            let addr = deps.api.addr_validate(&address)?;

            let donations = DONATIONS_BY_ADDR
                .prefix(addr.clone())
                .range(deps.storage, None, None, Order::Ascending)
                .map(|item| {
                    item.map(|(timestamp_seconds, donation_size)| {
                        (
                            u128_to_u64(donation_size).into(),
                            Timestamp::from_seconds(timestamp_seconds),
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let total = Uint128::from(
                DONATION_TOTALS
                    .may_load(deps.storage, &addr)?
                    .unwrap_or_default(),
            );

            Ok(to_json_binary(&UserDonationsResponse { donations, total })?)
        }
    }
}

pub fn u128_to_u64(value: u128) -> u64 {
    value.clamp(u64::MIN.into(), u64::MAX.into()) as u64
}
