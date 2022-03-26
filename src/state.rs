use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Account {
    pub accountHolder: Addr,
    pub paramAccTimeBeforeLost: Date,
    pub accDateOfLost: Date,
    pub executers: Tuple, 
    pub paramExeTimeBeforeLost: Date,
    pub exeDateOfLost: Date,
    pub vault: tokens,
}

pub const ACCOUNT: Item<Account> = Item::new("account");