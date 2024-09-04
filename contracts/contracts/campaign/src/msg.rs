use std::default;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    Addr, Api, Binary, Coin, Decimal, Deps, Order, Storage, Timestamp, Uint128, Uint64,
};

use crate::{state::CAMPAIGN_STATUS, ContractError};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ProtocolInstantiateSettings {
    pub withdrawl_fee: Decimal,
    pub fee_address: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub protocol_settings: ProtocolInstantiateSettings,
    pub campaign_metadata: CampaignMetadata,
}

#[cw_serde]
pub struct CampaignMetadata {
    pub name: String,
    pub description: String,
    pub test_campaign: bool,
    pub is_nsfw: bool,
    pub owner: String,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub accepted_denom: String,
    pub incentives: Incentives,
}

pub type Incentives = Vec<(IncentiveCriteria, Reward)>;

#[cw_serde]
pub enum IncentiveCriteria {
    AllDonationsAbove {
        min_donation: Uint64,
    },

    /// The donors from from_position to to_position inclusive will receive a reward
    /// The positions are 1-indexed
    /// If from_position is 1 and to_position is 100, then the top 100 donors will receive a reward
    TokensDonatedByPosition {
        from_position: u32,
        to_position: u32,
    },

    PercentTokensDonated {
        from_percent: Decimal,
        to_percent: Decimal,
    },

    FirstDonors {
        count: u32,
        min_donation: Uint64,
    },

    /// The first X% of donors will receive a reward
    FirstDonorsByPercentage {
        percentage: Decimal,
        min_donation: Uint64,
    },
    // RandomDonors {
    //     number_of_donors: u32,
    //     range_to_select_from: DonorRange,
    // },
}

#[cw_serde]
pub enum DonorRange {
    Percent {
        from_percent: Decimal,
        to_percent: Decimal,
    },
    Position {
        from_position: u32,
        to_position: u32,
    },
}

#[cw_serde]
pub enum Reward {
    OneOfOneNftAirdrop {},
    NftAirdrop {},
    WhitelistSpot {},
    // MintShare {
    //     percentage: Decimal,
    //     until: Timestamp,
    // },
    // MarketplaceRoyaltiesShare {
    //     percentage: Decimal,
    //     until: Timestamp
    // },
    TokenDistribution {
        token_to_airdrop: Coin,
        distribution_type: TokenAirdropDistributionType,
    },
    // NFTDistribution {
    //}
}

#[cw_serde]
pub enum TokenAirdropDistributionType {
    Equal,
    Proportional,
}

#[cw_serde]
pub struct ValidatedCampaignMetadata {
    pub name: String,
    pub description: String,
    pub test_campaign: bool,
    pub withdrawl_fee: Decimal,
    pub is_nsfw: bool,
    /// The address that will be the owner of the campaign
    /// This should be the external user who is creating the nft collection
    pub owner: Addr,
    /// The index contract that generated this contract and is the cosmwasm admin
    pub instantiated_by: Addr,
    pub start_timestamp: Timestamp,
    pub end_timestamp: Timestamp,
    pub accepted_denom: String,
    pub incentives: Vec<(IncentiveCriteria, Reward)>,
    pub fee_address: Addr,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    /// Change the owner address
    /// Only the owner can change the owner address
    SetOwner { owner: String },

    /// Deposit funds in order to participate in the campaign
    #[cw_orch(payable)]
    Deposit { donor_address: String },

    /// Withdraw all funds that you have deposited into the campaign
    UnDonate,

    /// Mark the campaign as complete
    CompleteCampaign {},

    /// Cancel the campaign
    /// Only the owner can cancel the campaign
    CancelCampaign,

    /// Distribute rewards to the donors
    DistributeRewards { limit: Option<u32> },
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    /// Gets the global settings of the whole storage contract
    #[returns(ValidatedCampaignMetadata)]
    CampaignMetadata,

    /// Gets the current status of the campaign
    #[returns(CampaignStatus)]
    CampaignStatus,

    /// List of the current donors to the campaign
    #[returns(Vec<(Addr, Coin)>)]
    DonationsBySize {
        limit: Option<u32>,
        start_after: Option<String>,
    },

    #[returns(Vec<(Addr, Coin, Timestamp)>)]
    DonationsByTime {
        ascending: bool,
        limit: Option<u32>,
        start_after: Option<Timestamp>,
    },

    /// Amount that the given user has donated to the campaign
    #[returns(UserDonationsResponse)]
    UserDonation { address: String },

    #[returns(Coin)]
    TotalDonations,

    #[returns(Uint64)]
    TotalDonors,
}

#[cw_serde]
pub struct UserDonationsResponse {
    pub donations: Vec<(Uint64, Timestamp)>,
    pub total: Uint128,
}

#[cw_serde]
#[derive(Default)]
pub enum CampaignStatus {
    /// Campaign is currently valid and not expired
    #[default]
    Active,
    /// Campaign was successfully funded and reward distribution is underway
    FundingClosed,
    /// Rewards have been distributed and the campaign is complete
    Completed,
    /// Strategy ran it's whole course without reaching it's funding minimum
    Expired,
    /// Strategy was cancelled before funds were donated
    Cancelled,
}

impl CampaignStatus {
    pub fn valid_transition(&self, new_status: &CampaignStatus) -> bool {
        match (self, new_status) {
            (
                CampaignStatus::Active,
                CampaignStatus::FundingClosed | CampaignStatus::Cancelled | CampaignStatus::Expired,
            ) => true,
            (CampaignStatus::FundingClosed, CampaignStatus::Completed) => true,
            _ => false,
        }
    }

    pub fn update_state(
        &self,
        storage: &mut dyn Storage,
        new_state: CampaignStatus,
    ) -> Result<(), ContractError> {
        if !self.valid_transition(&new_state) {
            return Err(ContractError::InvalidStateTransition(
                self.clone(),
                new_state,
            ));
        }
        CAMPAIGN_STATUS.save(storage, &new_state)?;
        Ok(())
    }

    pub fn validate_state(&self, expected_state: &CampaignStatus) -> Result<(), ContractError> {
        if self.eq(expected_state) {
            Ok(())
        } else {
            Err(ContractError::UnauthorizedState(
                self.clone(),
                expected_state.clone(),
            ))
        }
    }
}

impl std::fmt::Display for CampaignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CampaignStatus::Active => write!(f, "Active"),
            CampaignStatus::FundingClosed => write!(f, "Funding Closed"),
            CampaignStatus::Completed => write!(f, "Completed"),
            CampaignStatus::Expired => write!(f, "Expired"),
            CampaignStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl IncentiveCriteria {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            IncentiveCriteria::AllDonationsAbove { min_donation } => {
                return Ok(());
            }
            IncentiveCriteria::TokensDonatedByPosition {
                from_position,
                to_position,
            } => {
                if from_position.eq(&0) {
                    return Err(ContractError::InvalidIncentive(
                        "Tokens donated by position from_position must be greater than 0"
                            .to_string(),
                    ));
                }
                if from_position.gt(&to_position) {
                    return Err(ContractError::InvalidIncentive(
                        "Tokens donated by position from_position must be less than or equal to_position".to_string(),
                    ));
                }
            }
            IncentiveCriteria::PercentTokensDonated {
                from_percent,
                to_percent,
            } => {
                if from_percent.gt(&Decimal::percent(100)) || to_percent.gt(&Decimal::percent(100))
                {
                    return Err(ContractError::InvalidIncentive(
                        "Percent Tokens Donated from_percent and to_percent must be less than 100"
                            .to_string(),
                    ));
                }
                if from_percent.gt(&to_percent) {
                    return Err(ContractError::InvalidIncentive(
                        "Percent Tokens Donated from_percent must be less than to_percent"
                            .to_string(),
                    ));
                }
            }
            IncentiveCriteria::FirstDonors { count, .. } => {
                if count.eq(&0) {
                    return Err(ContractError::InvalidIncentive(
                        "First Donors count must be greater than 0".to_string(),
                    ));
                }
            }
            IncentiveCriteria::FirstDonorsByPercentage { percentage, .. } => {
                if percentage.gt(&Decimal::percent(100)) {
                    return Err(ContractError::InvalidIncentive(
                        "First Donors By Percentage percentage must be less than 100".to_string(),
                    ));
                }
            } // Incentive::RandomDonors {
              //     number_of_donors, ..
              // } => {
              //     if number_of_donors.eq(&0) {
              //         return Err(ContractError::InvalidIncentive(
              //             "number_of_donors must be greater than 0".to_string(),
              //         ));
              //     }
              // }
        }
        Ok(())
    }
}

// implement a validation trait for CampaignMetadata
impl CampaignMetadata {
    pub fn validate(
        &self,
        api: &dyn Api,
        current_time: Timestamp,
        instantiated_by: &Addr,
        ProtocolInstantiateSettings {
            withdrawl_fee,
            fee_address,
        }: &ProtocolInstantiateSettings,
    ) -> Result<ValidatedCampaignMetadata, ContractError> {
        let owner = api.addr_validate(&self.owner)?;

        // validate that the end_timestamp is in the future but not too far in the future
        if !(self.end_timestamp.gt(&current_time)
            && self.end_timestamp.gt(&self.start_timestamp)
            && self.end_timestamp.le(&current_time.plus_days(31 * 6)))
        {
            return Err(ContractError::InvalidExpiration {
                expiration_usage: "Campaign Validation".to_string(),
                given_timestamp: self.end_timestamp,
                current_timestamp: current_time.plus_days(31 * 6),
            });
        }

        if !self.start_timestamp.ge(&current_time) {
            return Err(ContractError::InvalidStartTimestamp {
                start_timestamp: self.start_timestamp,
                current_timestamp: current_time,
            });
        }

        let _ = self
            .incentives
            .iter()
            .try_for_each(|(incentive, _)| incentive.validate())?;

        if !(withdrawl_fee.ge(&Decimal::zero()) && withdrawl_fee.lt(&Decimal::percent(100))) {
            return Err(ContractError::InvalidWithdrawlFee(withdrawl_fee.clone()));
        }

        Ok(ValidatedCampaignMetadata {
            name: self.name.clone(),
            description: self.description.clone(),
            test_campaign: self.test_campaign,
            is_nsfw: self.is_nsfw,
            owner,
            instantiated_by: instantiated_by.clone(),
            start_timestamp: self.start_timestamp,
            end_timestamp: self.end_timestamp,
            accepted_denom: self.accepted_denom.clone(),
            incentives: self.incentives.clone(),
            withdrawl_fee: *withdrawl_fee,
            fee_address: api.addr_validate(&fee_address)?,
        })
    }
}
