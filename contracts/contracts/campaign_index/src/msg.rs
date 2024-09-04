use campaign::msg::{CampaignMetadata, CampaignStatus, ValidatedCampaignMetadata};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Coin, Decimal, Uint64};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    /// Contract administrator-
    /// if not specified it will default to the contract instantiator
    pub admin: Option<String>,
    pub enforce_whitelist: bool,
    pub campaign_contract_code_id: u64,
    pub creation_fee: Coin,
    pub campaign_funds_withdrawl_fee: Decimal,
    pub treasury_address: String,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    /// Migrate the campaigns from the old contract to the new one
    /// ADMIN ONLY ACTION
    MigrateCampaigns {
        code_id: Uint64,
        msg: Binary,
        limit: Option<u32>,
        start_after: Option<String>,
    },

    /// Global setting for whether or not to enforce the campaign creation whitelist
    /// ADMIN ONLY ACTION
    EnforceWhitelist(bool),

    /// Add a new whitelisted address that is allowed to create campaigns
    /// ADMIN ONLY ACTION
    GrantWhitelist {
        address: String,
        test_creation_only: bool,
    },

    /// Remove an address from the whitelist
    /// ADMIN ONLY ACTION
    RevokeWhitelist(String),

    /// For changing the contract's priviledged user
    /// ADMIN ONLY ACTION
    SetAdmin(String),

    /// ADMIN ONLY ACTION
    SetCampaignContractCodeId(Uint64),

    /// Create and store a new campaign
    #[cw_orch(payable)]
    CreateCampaign { campaign_metadata: CampaignMetadata },

    /// To be used when a campaign is completed and no longer needed
    /// ONLY THE CAMPAIGN ITSELF CAN CALL THIS
    ArchiveCampaign,

    /// Removes an campaign
    /// ADMIN ONLY ACTION
    DeleteCampaign { address: String },
    // PruneInactiveCampaigns { limit: u16, offset: u32 },
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    #[returns(Vec<WhitelistEntry>)]
    CreatorWhitelist {
        limit: Option<u32>,
        start_after: Option<String>,
    },

    #[returns(IndexSettings)]
    Settings,

    #[returns(Vec<CampaignsResponse>)]
    Campaigns {
        campaign_type: CampaignType,
        campaign_status: Option<CampaignStatus>,
        limit: Option<u32>,
        start_after: Option<String>,
    },
}

#[cw_serde]
pub struct CampaignsResponse {
    pub campaign_addr: Addr,
    pub metadata: ValidatedCampaignMetadata,
    pub status: CampaignStatus,
    pub total_donations: Coin,
    pub total_donors: Uint64,
}

#[cw_serde]
pub enum CampaignType {
    Main,
    Test,
    MainArchived,
    TestArchived,
}

#[cw_serde]
pub struct PaginationDetails {
    pub limit: Option<u32>,
    pub start_after: String,
}

#[cw_serde]
pub struct WhitelistEntry {
    pub address: Addr,
    pub test_creation_only: bool,
}

#[cw_serde]
pub struct IndexSettings {
    pub admin: Addr,
    pub enforce_whitelist: bool,
    pub campaign_contract_code_id: u64,
    pub creation_fee: Coin,
    pub campaign_funds_withdrawl_fee: Decimal,
    pub treasury_address: Addr,
}
