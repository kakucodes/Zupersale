use campaign::{
    msg::{CampaignMetadata, IncentiveCriteria, Reward},
    ZupersaleCampaign, ZupersaleCampaignExecuteMsgFns, ZupersaleCampaignQueryMsgFns,
};
use campaign_index::{ZupersaleIndex, ZupersaleIndexExecuteMsgFns, ZupersaleIndexQueryMsgFns};
use cosmwasm_std::{coin, coins, to_json_binary, Decimal, Timestamp};
use cw_orch::{anyhow, daemon::TxSender, prelude::*};
use networks::{ChainKind, NetworkInfo};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Eq, Debug)]
pub enum DeploymentType {
    Prod,
    Dev,
}

pub const STARGAZE_NETWORK: NetworkInfo = NetworkInfo {
    pub_address_prefix: "stars",
    coin_type: 118u32,
    chain_name: "stargaze",
};

const DEPLOYMENT: DeploymentType = DeploymentType::Dev;

pub fn main() -> anyhow::Result<()> {
    // let rt = Runtime::new().unwrap();
    dotenv::dotenv().ok();
    env_logger::init();

    let stargaze = if DEPLOYMENT.eq(&DeploymentType::Dev) {
        ChainInfo {
            kind: ChainKind::Testnet,
            chain_id: "elgafar-1",
            gas_denom: "ustars",
            gas_price: 0.04,
            grpc_urls: &["http://grpc-1.elgafar-1.stargaze-apis.com:26660"],
            network_info: STARGAZE_NETWORK,
            lcd_url: None,
            fcd_url: None,
        }
    } else {
        ChainInfo {
            kind: ChainKind::Mainnet,
            chain_id: "stargaze-1",
            gas_denom: "ustars",
            gas_price: 1.1,
            grpc_urls: &["http://stargaze-grpc.polkachu.com:13790"],
            network_info: STARGAZE_NETWORK,
            lcd_url: None,
            fcd_url: None,
        }
    };

    let stargaze_chain = Daemon::builder(stargaze)
        //.handle(rt.handle())
        .build()?;

    let zupersale_campaign = ZupersaleCampaign::new(stargaze_chain.clone());
    let zupersale_index = ZupersaleIndex::new(stargaze_chain.clone());

    zupersale_index.upload_if_needed()?;
    zupersale_campaign.upload_if_needed()?;

    if zupersale_index.address().is_err() {
        println!(
            "instantiating zupersale index. using campaign code id: {}",
            zupersale_campaign.code_id()?
        );
        zupersale_index.instantiate(
            &campaign_index::msg::InstantiateMsg {
                admin: None,
                enforce_whitelist: false,
                campaign_contract_code_id: zupersale_campaign.code_id()?,
                creation_fee: coin(100_000_000, "ustars"),
                campaign_funds_withdrawl_fee: Decimal::percent(5),
                treasury_address: stargaze_chain.sender().address().to_string(),
            },
            Some(&Addr::unchecked(
                stargaze_chain.sender().address().to_string(),
            )),
            None,
        )?;

        let current_timestamp = Timestamp::from_seconds(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Need current epoch")
                .as_secs(),
        );

        zupersale_index.create_campaign(
            CampaignMetadata {
                name: "test campaign 1".to_string(),
                description:
                    "this campaign is simply a test and this is a description of the test campaign"
                        .to_string(),
                test_campaign: true,
                is_nsfw: false,
                owner: stargaze_chain.sender().address().to_string(),
                // start the campaign at the current time
                start_timestamp: current_timestamp.plus_seconds(3),
                end_timestamp: current_timestamp.plus_days(90),
                accepted_denom: "ustars".to_string(),
                incentives: vec![
                    (
                        IncentiveCriteria::TokensDonatedByPosition {
                            from_position: 1,
                            to_position: 10,
                        },
                        Reward::NftAirdrop {},
                    ),
                    (
                        IncentiveCriteria::FirstDonors {
                            count: 20,
                            min_donation: 10_000_000u64.into(),
                        },
                        Reward::NftAirdrop {},
                    ),
                    (
                        IncentiveCriteria::PercentTokensDonated {
                            from_percent: Decimal::percent(0),
                            to_percent: Decimal::percent(100),
                        },
                        Reward::WhitelistSpot {},
                    ),
                ],
            },
            &[],
        )?;
    } else {
        zupersale_index.migrate(
            &campaign_index::msg::MigrateMsg {},
            zupersale_index.code_id()?,
        )?;

        zupersale_index.migrate_campaigns(
            zupersale_campaign.code_id()?,
            to_json_binary(&campaign::msg::MigrateMsg {})?,
            None,
            None,
        )?;
    }

    Ok(())
}
