use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr; 
// import map 
use cw_storage_plus::(Item, Map);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Executers {
    pub exeAddr: Addr,
    pub share: i32,
    pub exeDateOfLost: Date,
}

pub struct Data { // Getting account holder data 
    pub paramAccTimeBeforeLost: Date, // Amount of time before the funds be avaliable for each reset
    pub accDateOfLost: Date, // The date that which executers(heirs) can access to the funds
    pub executers: [Executers], // List of heirs that will inherit the account. Array of (Addr, %ofShare, dateOfLost)
    pub paramExeTimeBeforeLost: Date,  // Amount of time before the avaliable funds for claim for each reset
    pub vault: tokens, // Holding accounts coins
}

pub const ACCOUNT: Map<Addr, Data]> = Map::new("account"); // Map<Addr, Account>