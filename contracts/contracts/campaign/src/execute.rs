use cosmwasm_std::{
    coin, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Order, OverflowError, Response, Uint128,
    Uint64,
};

use crate::{
    msg::{CampaignStatus, ValidatedCampaignMetadata},
    state::{
        CAMPAIGN_STATUS, DONATIONS_BY_ADDR, DONATIONS_BY_DONOR_SIZE, DONATIONS_BY_TIME,
        DONATION_TOTALS, METADATA, TOTAL_CAMPAIGN_DONATIONS,
    },
    ContractError,
};

pub fn undeposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    accepted_denom: &str,
) -> Result<Response, ContractError> {
    // check that the campaign is active
    CAMPAIGN_STATUS
        .load(deps.as_ref().storage)?
        .validate_state(&CampaignStatus::Active)?;

    let withdrawl_amount = DONATION_TOTALS.load(deps.as_ref().storage, &info.sender)?;

    TOTAL_CAMPAIGN_DONATIONS.update(deps.storage, |total_donations| {
        Ok::<Uint128, ContractError>(total_donations.checked_sub(withdrawl_amount.into())?)
    })?;
    DONATION_TOTALS.remove(deps.storage, &info.sender);
    DONATIONS_BY_DONOR_SIZE.remove(
        deps.storage,
        (withdrawl_amount.clone(), info.sender.clone()),
    );

    let sender_donations_key = DONATIONS_BY_ADDR.prefix(info.sender.clone());

    sender_donations_key
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .for_each(|(timestamp_seconds, _)| {
            DONATIONS_BY_ADDR.remove(deps.storage, (info.sender.clone(), timestamp_seconds));
            DONATIONS_BY_TIME.remove(deps.storage, (timestamp_seconds, info.sender.clone()));
        });

    let withdraw_coin = coin(withdrawl_amount.into(), accepted_denom);

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![withdraw_coin.clone()],
        }))
        .add_attribute("new_donation_balance", withdraw_coin.to_string()))
}

pub fn deposit(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    donor_address: &str,
    deposit_amount: Uint128,
) -> Result<Response, ContractError> {
    // check that the campaign is active
    CAMPAIGN_STATUS
        .load(deps.as_ref().storage)?
        .validate_state(&CampaignStatus::Active)?;

    let ValidatedCampaignMetadata {
        start_timestamp, ..
    } = METADATA.load(deps.as_ref().storage)?;

    if env.block.time.lt(&start_timestamp) {
        return Err(ContractError::CampaignNotYetActive(start_timestamp));
    }

    let donor_addr = deps.api.addr_validate(donor_address)?;

    // first we should bump the total campaign donations
    TOTAL_CAMPAIGN_DONATIONS.update(deps.storage, |total_donations| {
        Ok::<Uint128, ContractError>(total_donations.checked_add(deposit_amount)?)
    })?;

    // get the user's previous donation total so we can use it to update the donationsbydonorsize
    let prev_user_donation_total = DONATION_TOTALS
        .may_load(deps.storage, &donor_addr)
        .unwrap_or_default()
        .unwrap_or_default();

    // update the user's donation_total
    let new_user_total = prev_user_donation_total
        .checked_add(deposit_amount.u128())
        .ok_or(ContractError::OverflowError(OverflowError::new(
            cosmwasm_std::OverflowOperation::Add,
            prev_user_donation_total.clone(),
            deposit_amount.u128(),
        )))?;

    DONATION_TOTALS.save(deps.storage, &donor_addr, &new_user_total)?;

    // then bump the user's donation_by_donor_size
    DONATIONS_BY_DONOR_SIZE.remove(
        deps.storage,
        (prev_user_donation_total.clone(), donor_addr.clone()),
    );
    DONATIONS_BY_DONOR_SIZE.save(deps.storage, (new_user_total, donor_addr.clone()), &())?;

    // add the new donations to the donations_by_addr and donations_by_time maps
    DONATIONS_BY_ADDR.save(
        deps.storage,
        (donor_addr.clone(), env.block.time.seconds()),
        &deposit_amount.u128(),
    )?;
    DONATIONS_BY_TIME.save(
        deps.storage,
        (env.block.time.seconds(), donor_addr),
        &deposit_amount.u128(),
    )?;

    Ok(Response::new().add_attributes(vec![
        ("action", "deposit"),
        ("amount", &deposit_amount.to_string()),
    ]))
}

pub fn cancel_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // check that the DONATIONS map is empty
    if TOTAL_CAMPAIGN_DONATIONS.load(deps.storage)?.is_zero() {
        return Err(ContractError::CampaignAlreadyStarted);
    }

    CAMPAIGN_STATUS
        .load(deps.storage)?
        .update_state(deps.storage, CampaignStatus::Cancelled)?;

    Ok(Response::new().add_attribute("action", "cancel"))
}
