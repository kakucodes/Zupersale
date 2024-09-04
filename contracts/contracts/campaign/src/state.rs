use cosmwasm_std::{Addr, Binary, Timestamp, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

use crate::msg::{CampaignStatus, ValidatedCampaignMetadata};

pub const METADATA: Item<ValidatedCampaignMetadata> = Item::new("metadata");

pub const CAMPAIGN_STATUS: Item<CampaignStatus> = Item::new("campaign_state");

// Address and timestamp seconds since epoch are the key
pub const DONATIONS_BY_ADDR: Map<(Addr, u64), u128> = Map::new("donations_by_addr");
// Timestamp seconds since epoch and address are the key
pub const DONATIONS_BY_TIME: Map<(u64, Addr), u128> = Map::new("donation_by_times");
pub const DONATIONS_BY_DONOR_SIZE: Map<(u128, Addr), ()> = Map::new("donation_by_size");

// Cache of the total donations for each donor
pub const DONATION_TOTALS: Map<&Addr, u128> = Map::new("donation_totals");

// Cache of the total donations for the campaign
pub const TOTAL_CAMPAIGN_DONATIONS: Item<Uint128> = Item::new("total_campaign_donations");
