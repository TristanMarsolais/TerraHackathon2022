use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]


pub struct Account { // Getting account holder data 
    pub accountHolder: Addr,  // Wallet address
    pub paramAccTimeBeforeLost: Date, // Amount of time before the funds be avaliable for each reset
    pub accDateOfLost: Date, // The date that which executers(heirs) can access to the funds
    pub executers: Tuple, // List of heirs that will inherit the account. Array of (Addr, %ofShare, dateOfLost)
    pub paramExeTimeBeforeLost: Date,  // Amount of time before the avaliable funds for claim for each reset
    pub exeDateOfLost: Date, // Last date to claim for executers
    pub vault: tokens, // Holding accounts coins
}

pub const ACCOUNT: Item<Account> = Item::new("account");