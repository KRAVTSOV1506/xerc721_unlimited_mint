use cw_storage_plus::{Item, Map};

pub const OWNER: Item<String> = Item::new("owner");
// chain chain id => address of our contract in bytes
pub const REMOTE_CONTRACT_MAPPING: Map<String, String> = Map::new("remote_contract_mapping");
// who has already minted
pub const ALREADY_MINTED: Map<String, bool> = Map::new("already_minted");
pub const TOTAL_SUPPLY: Item<u64> = Item::new("total_supply");
