use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::msg::IndexSettings;

// General settings for the whole contract
pub const STORE_SETTINGS: Item<IndexSettings> = Item::new("store_settings");

// Count of all the campaigns that have been created
pub const CAMPAIGN_CREATION_COUNT: Item<u64> = Item::new("campaign_creation_count");

// Map of all the whitelisted addresses that are allowed to create campaigns
// and whether or not they are only allowed to create only test campaigns
pub const WHITELISTED_ADDRESSES: Map<&Addr, bool> = Map::new("whitelisted_addresses");

pub const MAIN_CAMPAIGNS: Map<&Addr, ()> = Map::new("campaigns");
pub const MAIN_ARCHIVED_CAMPAIGNS: Map<&Addr, ()> = Map::new("archive_campaigns");

pub const TEST_CAMPAIGNS: Map<&Addr, ()> = Map::new("test_campaigns");
pub const TEST_ARCHIVED_CAMPAIGNS: Map<&Addr, ()> = Map::new("test_archive_campaigns");
