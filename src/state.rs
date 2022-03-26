// Code inspiration for handling send and receive tokens by https://github.com/Duel-Dojo/The-Duel-Dojo

// Imports
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Order, StdError, StdResult, Storage}; 
use cw_storage_plus::(Item, Map);

use cw_cw20_plus::(Message, Send);
use cw20::{Balance, Cw20CoinVerified};

// Initiate a basic state 
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct State {
    pub creator: Addr,
    pub owner: Addr,
}


// Handle send and receive tokens
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct GenericBalance {
    pub native: Vec<Coin>,
    pub cw20: Vec<Cw20CoinVerified>,
}

impl GenericBalance {
    pub fn new() -> GenericBalance {
        GenericBalance {
            native: vec![],
            cw20: vec![],
        }
    }
}

impl GenericBalance {
    pub fn add_tokens(&mut self, add: Balance) {
        match add {
            Balance::Native(balance) => {
                for token in balance.0 {
                    let index = self.native.iter().enumerate().find_map(|(i, exist)| {
                        if exist.denom == token.denom {
                            Some(i)
                        } else {
                            None
                        }
                    });
                    match index {
                        Some(idx) => self.native[idx].amount += token.amount,
                        None => self.native.push(token),
                    }
                }
            }
            Balance::Cw20(token) => {
                let index = self.cw20.iter().enumerate().find_map(|(i, exist)| {
                    if exist.address == token.address {
                        Some(i)
                    } else {
                        None
                    }
                });
                match index {
                    Some(idx) => self.cw20[idx].amount += token.amount,
                    None => self.cw20.push(token),
                }
            }
        };
    }
}


// an Account object
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Executer {
    pub exeAddr: Addr,
    pub share: i32,
    pub exeDateOfLost: Date, // block hight
}

pub struct Data { // Getting account holder data 
    pub paramAccTimeBeforeLost: Date, // !!! Block height Amount of time before the funds be avaliable for each reset
    pub accDateOfLost: Date, // !!! Block height The date that which executers(heirs) can access to the funds
    pub executer: Executer, // List of heirs that will inherit the account. Array of (Addr, %ofShare, dateOfLost)
    pub paramExeTimeBeforeLost: Date,  // !!! Block height Amount of time before the avaliable funds for claim for each reset
    pub vault: GenericBalance, // Holding accounts coins
}

pub const ACCOUNT: Map<Addr, Data]> = Map::new("account"); // Map<Addr, Account>