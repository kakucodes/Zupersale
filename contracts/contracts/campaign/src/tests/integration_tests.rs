use crate::{
    msg::{
        CampaignMetadata, IncentiveCriteria, InstantiateMsg, ProtocolInstantiateSettings, Reward,
        TokenAirdropDistributionType,
    },
    ContractError, ZupersaleCampaign, ZupersaleCampaignExecuteMsgFns, ZupersaleCampaignQueryMsgFns,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, coins, to_json_binary, Addr, Decimal, StdError, Timestamp, Uint128, Uint64,
};
use cw_orch::{anyhow, prelude::*};

fn setup<Chain: CwEnv>(
    chain: Chain,
    owner: Addr,
    treasury: Addr,
    campaign_start_offset_seconds: Option<u64>,
) -> anyhow::Result<ZupersaleCampaign<Chain>> {
    let campaign_contract = ZupersaleCampaign::new(chain.clone());

    let _ = campaign_contract.upload()?;

    let upload_campaign_resp = campaign_contract.upload()?;

    // Instantiate the contract
    let init_resp = campaign_contract.instantiate(
        &InstantiateMsg {
            protocol_settings: ProtocolInstantiateSettings {
                withdrawl_fee: Decimal::percent(10),
                fee_address: treasury.to_string(),
            },
            campaign_metadata: CampaignMetadata {
                name: "Test Campaign".to_string(),
                description: "Test Campaign Description".to_string(),
                test_campaign: true,
                is_nsfw: true,
                owner: owner.into_string(),
                start_timestamp: chain
                    .block_info()?
                    .time
                    .plus_seconds(campaign_start_offset_seconds.unwrap_or(3)),
                end_timestamp: chain.block_info()?.time.plus_days(90),
                accepted_denom: "ubtc".to_string(),
                incentives: vec![],
            },
        },
        Some(&chain.sender_addr()),
        None,
    )?;

    Ok(campaign_contract)
}

fn setup_with_token_incentives<Chain: CwEnv>(
    chain: Chain,
    owner: Addr,
    treasury: Addr,
    instantiation_funds: Option<&[Coin]>,
) -> anyhow::Result<ZupersaleCampaign<Chain>> {
    let campaign_contract = ZupersaleCampaign::new(chain.clone());

    let _ = campaign_contract.upload()?;

    let upload_campaign_resp = campaign_contract.upload()?;

    // Instantiate the contract
    let init_resp = campaign_contract.instantiate(
        &InstantiateMsg {
            protocol_settings: ProtocolInstantiateSettings {
                withdrawl_fee: Decimal::percent(10),
                fee_address: treasury.to_string(),
            },
            campaign_metadata: CampaignMetadata {
                name: "Test Campaign".to_string(),
                description: "Test Campaign Description".to_string(),
                test_campaign: true,
                is_nsfw: true,
                owner: owner.into_string(),
                start_timestamp: chain.block_info()?.time.plus_seconds(3),
                end_timestamp: chain.block_info()?.time.plus_days(90),
                accepted_denom: "ubtc".to_string(),
                incentives: vec![(
                    IncentiveCriteria::AllDonationsAbove {
                        min_donation: Uint64::from(5_000_000u64),
                    },
                    Reward::TokenDistribution {
                        token_to_airdrop: coin(100_000_000, "uzuper"),
                        distribution_type: TokenAirdropDistributionType::Proportional,
                    },
                )],
            },
        },
        Some(&chain.sender_addr()),
        instantiation_funds,
    )?;

    Ok(campaign_contract)
}

#[test]
pub fn campaign_happy_path() -> anyhow::Result<()> {
    let admin = Addr::unchecked("admin");
    let treasury = Addr::unchecked("treasury");
    let donor_1 = Addr::unchecked("donor_1");
    let donor_2 = Addr::unchecked("donor_2");

    let chain = Mock::new(&admin);

    chain.add_balance(donor_1.clone(), coins(10_000_000, "ubtc"))?;
    chain.add_balance(donor_2.clone(), coins(20_000_000, "ubtc"))?;

    let campaign_contract = setup(chain.clone(), admin.clone(), treasury.clone(), Some(100))?;

    // This should fail because the campaign has not started yet
    let deposit_response = campaign_contract
        .call_as(&donor_1)
        .deposit(donor_1.clone(), &coins(1_000_000, "ubtc"));
    assert!(deposit_response.is_err(), "Deposit should fail");

    // Wait for the campaign to start
    chain.wait_seconds(100)?;

    campaign_contract
        .call_as(&donor_1)
        .deposit(donor_1.clone(), &coins(1_000_000, "ubtc"))?;

    let user_donation = campaign_contract.user_donation(&donor_1)?;
    assert_eq!(user_donation.total, Uint128::from(1_000_000u128));

    campaign_contract
        .call_as(&donor_2)
        .deposit(donor_2.clone(), &coins(2_000_000, "ubtc"))?;

    campaign_contract
        .call_as(&donor_1)
        .deposit(donor_1.clone(), &coins(500_000, "ubtc"))?;

    let user_donation = campaign_contract.user_donation(&donor_1)?;
    assert_eq!(user_donation.total, Uint128::from(1_500_000u128));

    let user_donation = campaign_contract.user_donation(&donor_2)?;
    assert_eq!(user_donation.total, Uint128::from(2_000_000u128));

    let donations_by_size = campaign_contract.donations_by_size(None, None)?;
    assert_eq!(donations_by_size.len(), 2);

    campaign_contract.call_as(&donor_1).un_donate()?;

    let donations_by_size = campaign_contract.donations_by_size(None, None)?;
    assert_eq!(donations_by_size.len(), 1);

    let user_donation = campaign_contract.user_donation(&donor_1)?;
    assert_eq!(user_donation.total, Uint128::zero());

    Ok(())
}

#[test]
pub fn campaign_with_token_incentives() -> anyhow::Result<()> {
    let admin = Addr::unchecked("admin");
    let treasury = Addr::unchecked("treasury");
    let donor_1 = Addr::unchecked("donor_1");
    let donor_2 = Addr::unchecked("donor_2");

    let chain = Mock::new(&admin);

    chain.add_balance(donor_1.clone(), coins(10_000_000, "ubtc"))?;
    chain.add_balance(donor_2.clone(), coins(20_000_000, "ubtc"))?;
    chain.add_balance(
        chain.sender.clone(),
        vec![coin(100_000_000, "uzuper"), coin(100_000_000, "ubtc")],
    )?;

    let campaign_contract =
        setup_with_token_incentives(chain.clone(), admin.clone(), treasury.clone(), None);
    assert!(
        campaign_contract.is_err(),
        "campaign shouldn't be created without sending in funds for token incentives"
    );

    let campaign_contract = setup_with_token_incentives(
        chain.clone(),
        admin.clone(),
        treasury.clone(),
        Some(&coins(100_000_000, "ubtc")),
    );
    assert!(
        campaign_contract.is_err(),
        "campaign shouldn't be created when passing the wrong funds for token incentives"
    );

    let campaign_contract = setup_with_token_incentives(
        chain.clone(),
        admin.clone(),
        treasury.clone(),
        Some(&coins(100_000_000, "uzuper")),
    );
    assert!(
        campaign_contract.is_ok(),
        "campaign should be created since the incentive funds are being passed through"
    );

    Ok(())
}
