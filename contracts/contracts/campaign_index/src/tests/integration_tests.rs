use crate::{
    interface::ZupersaleIndex,
    msg::{CampaignType, ExecuteMsgFns, InstantiateMsg, QueryMsgFns},
    ContractError,
};
use campaign::{
    msg::{CampaignMetadata, IncentiveCriteria, Reward},
    ZupersaleCampaign,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, coins, to_json_binary, Addr, Decimal, StdError, Timestamp, Uint64};
use cw_orch::{anyhow, prelude::*};
use sha2::digest::generic_array::arr::Inc;

// consts for testing
const USER: &str = "user";
const CAMPAIGN_CREATOR: &str = "campaign_creator";
const ADMIN: &str = "admin";
const TREASURY: &str = "treasury";

/// Instantiate the contract in any CosmWasm environment
fn setup<Chain: CwEnv>(
    chain: Chain,
    admin: Addr,
    treasury: Addr,
) -> anyhow::Result<ZupersaleIndex<Chain>> {
    let index_contract = ZupersaleIndex::new(chain.clone());
    let campaign_contract = ZupersaleCampaign::new(chain.clone());

    // Upload the index contract
    let _ = index_contract.upload()?;

    let upload_campaign_resp = campaign_contract.upload()?;

    // Instantiate the contract
    let init_resp = index_contract.instantiate(
        &InstantiateMsg {
            campaign_contract_code_id: upload_campaign_resp.uploaded_code_id()?,
            admin: Some(admin.to_string()),
            enforce_whitelist: false,
            creation_fee: coin(10, "ubtc"),
            campaign_funds_withdrawl_fee: Decimal::percent(10u64),
            treasury_address: treasury.to_string(),
        },
        Some(&chain.sender()),
        None,
    )?;

    Ok(index_contract)
}

#[test]
pub fn campaign_creation_happy_path() -> anyhow::Result<()> {
    // mock chain with a bech32 prefix of "stars"- necessary for using instantiate2 in tests
    let chain = MockBech32::new_with_chain_id("stars", "stargaze-1");

    let admin = chain.addr_make(ADMIN);
    let campaign_creator = chain.addr_make_with_balance(CAMPAIGN_CREATOR, coins(1000, "ubtc"))?;
    let user = chain.addr_make(USER);
    let treasury = chain.addr_make(TREASURY);

    let sender = chain.sender();

    let index_contract = setup(chain.clone(), admin.clone(), treasury.clone())?;

    index_contract
        // Set the caller to user
        .call_as(&campaign_creator)
        // Call the increment function (auto-generated function provided by CounterExecuteMsgFns)
        .create_campaign(
            CampaignMetadata {
                name: "test campaign 1".to_string(),
                description:
                    "this campaign is simply a test and this is a description of the test campaign"
                        .to_string(),
                test_campaign: true,
                is_nsfw: false,
                owner: campaign_creator.to_string(),
                start_timestamp: chain.block_info()?.time.plus_seconds(3),
                end_timestamp: chain.block_info()?.time.plus_days(90),
                accepted_denom: "ubtc".to_string(),
                incentives: vec![
                    (
                        IncentiveCriteria::TokensDonatedByPosition {
                            from_position: 1,
                            to_position: 10,
                        },
                        Reward::NftAirdrop,
                    ),
                    (
                        IncentiveCriteria::FirstDonors {
                            count: 20,
                            min_donation: 10_000_000u64.into(),
                        },
                        Reward::NftAirdrop,
                    ),
                    (
                        IncentiveCriteria::PercentTokensDonated {
                            from_percent: Decimal::percent(0),
                            to_percent: Decimal::percent(100),
                        },
                        Reward::WhitelistSpot,
                    ),
                ],
            },
            &[],
        )?;

    index_contract
        // Set the caller to user
        .call_as(&campaign_creator)
        // Call the increment function (auto-generated function provided by CounterExecuteMsgFns)
        .create_campaign(
            CampaignMetadata {
                name: "main campaign 1".to_string(),
                description:
                    "this campaign is simply a main and this is a description of the main campaign"
                        .to_string(),
                test_campaign: false,
                is_nsfw: false,
                owner: campaign_creator.to_string(),
                start_timestamp: chain.block_info()?.time.plus_seconds(3),
                end_timestamp: chain.block_info()?.time.plus_days(90),
                accepted_denom: "ubtc".to_string(),
                incentives: vec![
                    (
                        IncentiveCriteria::TokensDonatedByPosition {
                            from_position: 1,
                            to_position: 10,
                        },
                        Reward::NftAirdrop,
                    ),
                    (
                        IncentiveCriteria::FirstDonors {
                            count: 20,
                            min_donation: 10_000_000u64.into(),
                        },
                        Reward::NftAirdrop,
                    ),
                    (
                        IncentiveCriteria::PercentTokensDonated {
                            from_percent: Decimal::percent(0),
                            to_percent: Decimal::percent(100),
                        },
                        Reward::WhitelistSpot,
                    ),
                ],
            },
            &coins(10, "ubtc"),
        )?;

    let treasury_btc_balance = chain
        .bank_querier()
        .balance(treasury.clone(), Some("ubtc".to_string()))
        .unwrap();
    // The two creations fee should be in the treasury
    assert_eq!(treasury_btc_balance, coins(10, "ubtc"));

    let created_campaigns = index_contract
        .campaigns(CampaignType::Main, None, None, None)
        .unwrap();

    assert_eq!(created_campaigns.len(), 1, "main campaigns count is wrong");
    assert_eq!(
        created_campaigns.first().unwrap().metadata.name,
        "main campaign 1",
        "main campaign name is wrong"
    );

    let created_test_campaigns = index_contract
        .campaigns(CampaignType::Test, None, None, None)
        .unwrap();
    assert_eq!(
        created_test_campaigns.len(),
        1,
        "test campaigns count is wrong"
    );
    assert_eq!(
        created_test_campaigns.first().unwrap().metadata.name,
        "test campaign 1",
        "test campaign name is wrong"
    );

    Ok(())
}

#[test]
pub fn campaign_creation_whitelist() -> anyhow::Result<()> {
    let chain = MockBech32::new_with_chain_id("stars", "stargaze-1");

    let admin = chain.addr_make(ADMIN);
    let campaign_creator = chain.addr_make_with_balance(CAMPAIGN_CREATOR, coins(1000, "ubtc"))?;
    let user = chain.addr_make(USER);
    let treasury = chain.addr_make(TREASURY);
    let sender = chain.sender();

    let index_contract = setup(chain.clone(), admin.clone(), treasury.clone())?;

    index_contract.call_as(&admin).enforce_whitelist(true)?;

    assert_eq!(
        index_contract.settings()?.enforce_whitelist,
        true,
        "whitelist not enforced"
    );

    let test_campaign_metadata = CampaignMetadata {
        name: "test campaign 1".to_string(),
        description:
            "this campaign is simply a test and this is a description of the test campaign"
                .to_string(),
        test_campaign: true,
        is_nsfw: false,
        owner: campaign_creator.to_string(),
        start_timestamp: chain.block_info()?.time.plus_seconds(3),
        end_timestamp: chain.block_info()?.time.plus_days(90),

        accepted_denom: "ubtc".to_string(),
        incentives: vec![
            (
                IncentiveCriteria::TokensDonatedByPosition {
                    from_position: 1,
                    to_position: 10,
                },
                Reward::NftAirdrop,
            ),
            (
                IncentiveCriteria::FirstDonors {
                    count: 20,
                    min_donation: 10_000_000u64.into(),
                },
                Reward::NftAirdrop,
            ),
            (
                IncentiveCriteria::PercentTokensDonated {
                    from_percent: Decimal::percent(0),
                    to_percent: Decimal::percent(100),
                },
                Reward::WhitelistSpot,
            ),
        ],
    };

    let main_campaign_metadata = CampaignMetadata {
        name: "main campaign 1".to_string(),
        description:
            "this campaign is simply a main caimpaign and this is a description of the main campaign"
                .to_string(),
        test_campaign: false,
        is_nsfw: false,
        owner: campaign_creator.to_string(),
        start_timestamp: chain.block_info()?.time.plus_seconds(3),
        end_timestamp: chain.block_info()?.time.plus_days(90),
        accepted_denom: "ubtc".to_string(),
        incentives: vec![
            (
                IncentiveCriteria::TokensDonatedByPosition {
                    from_position: 1,
                    to_position: 10,
                },
                Reward::NftAirdrop,
            ),
            (
                IncentiveCriteria::FirstDonors {
                    count: 20,
                    min_donation: 10_000_000u64.into(),
                },
                Reward::NftAirdrop,
            ),
            (
                IncentiveCriteria::PercentTokensDonated {
                    from_percent: Decimal::percent(0),
                    to_percent: Decimal::percent(100),
                },
                Reward::WhitelistSpot,
            ),
        ],
    };

    let test_campaign_creation_result = index_contract
        .call_as(&campaign_creator)
        .create_campaign(test_campaign_metadata.clone(), &[]);

    assert!(
        test_campaign_creation_result.is_err(),
        "campaign creation should fail since whitelist is being enforced"
    );

    let grant_whitelist_result = index_contract
        .call_as(&user)
        .grant_whitelist(campaign_creator.to_string(), true);

    assert!(
        grant_whitelist_result.is_err(),
        "grant whitelist should fail since it is not called by the admin"
    );

    let grant_whitelist_result = index_contract
        .call_as(&admin)
        .grant_whitelist(campaign_creator.to_string(), true)?;

    let test_campaign_creation_result = index_contract
        .call_as(&campaign_creator)
        .create_campaign(test_campaign_metadata.clone(), &[])?;

    let created_test_campaigns = index_contract
        .campaigns(CampaignType::Test, None, None, None)
        .unwrap();
    assert_eq!(
        created_test_campaigns.len(),
        1,
        "test campaigns count is wrong"
    );
    assert_eq!(
        created_test_campaigns.first().unwrap().metadata.name,
        "test campaign 1",
        "test campaign name is wrong"
    );

    let main_campaign_creation_result = index_contract
        .call_as(&campaign_creator)
        .create_campaign(main_campaign_metadata.clone(), &coins(10, "ubtc"));

    assert!(
        main_campaign_creation_result.is_err(),
        "main campaign creation should fail since whitelist is being enforced and they're only whitelisted for test campaigns"
    );

    let grant_whitelist_result = index_contract
        .call_as(&admin)
        .grant_whitelist(campaign_creator.to_string(), false)?;

    let main_campaign_creation_result = index_contract
        .call_as(&campaign_creator)
        .create_campaign(main_campaign_metadata.clone(), &coins(10, "ubtc"))?;

    let created_main_campaigns = index_contract
        .campaigns(CampaignType::Main, None, None, None)
        .unwrap();
    assert_eq!(
        created_main_campaigns.len(),
        1,
        "main campaigns count is wrong"
    );
    assert_eq!(
        created_main_campaigns.first().unwrap().metadata.name,
        "main campaign 1",
        "main campaign name is wrong"
    );

    Ok(())
}
