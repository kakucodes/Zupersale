use cosmwasm_std::{coin, Addr, Coin, Deps, Env, Order, Storage, Timestamp, Uint64};
use cw_storage_plus::Bound;

use crate::{
    contract::u128_to_u64,
    msg::ValidatedCampaignMetadata,
    state::{DONATIONS_BY_ADDR, DONATIONS_BY_DONOR_SIZE, DONATION_TOTALS, METADATA},
    ContractError,
};

pub fn query_donations_by_size(
    deps: Deps,
    env: Env,
    limit: Option<u32>,
    start_after: Option<String>,
) -> Result<Vec<(Addr, Coin)>, ContractError> {
    let ValidatedCampaignMetadata { accepted_denom, .. } = METADATA.load(deps.storage)?;

    let start_after_addr = match start_after {
        Some(addr) => Some(deps.api.addr_validate(&addr)?),
        None => None,
    };

    let donations = DONATIONS_BY_DONOR_SIZE
        .range(
            deps.storage,
            // start_after_addr.as_ref().map(Bound::exclusive),
            None,
            None,
            Order::Descending,
        )
        .take(limit.unwrap_or(30u32) as usize)
        .map(|item| {
            let ((donation_size, addr), _) = item?;
            Ok((
                addr,
                coin(u128_to_u64(donation_size).into(), &accepted_denom),
            ))
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    Ok(donations)
}

pub fn query_donations_by_time(
    deps: Deps,
    env: Env,
    order: Order,
    limit: Option<u32>,
    start_after: Option<Timestamp>,
) -> Result<Vec<(Uint64, Uint64)>, ContractError> {
    let ValidatedCampaignMetadata { accepted_denom, .. } = METADATA.load(deps.storage)?;

    // let donations = DONATION_BY_TIME
    //     .range(
    //         deps.storage,
    //         start_after_time.as_ref().map(Bound::exclusive),
    //         None,
    //         order,
    //     )
    //     .take(limit.unwrap_or(30u32) as usize)
    //     .map(|item| {
    //         let (time, donation_size) = item?;
    //         Ok((time, coin(donation_size.u64().into(), &accepted_denom)))
    //     })
    //     .collect::<Result<Vec<_>, ContractError>>()?;

    // Ok(donations)
    Ok(vec![])
}
