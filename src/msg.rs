// Code inspiration for handaling send and receive tokens by https://github.com/Duel-Dojo/The-Duel-Dojo

use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

use crate::state::GenericBalance;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub sender: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateAccount { accountHolder: Addr, executers: Executers, ahDateOfLost: Date,  token: Denom, amount: i32 },
    ModifyAccount { accountHolder: Addr, executers: Executers, ahDateOfLost: Date }, // implement executers list with 
    AHStillAlive { accountHolder: Addr }, 
    AHSendFunds { accountHolder: Addr, token: Denom, amount: i32 }, 
    AHWithdraw { accountHolder: Addr }, 
    EXStillAlive { accountHolder: Addr, execAddr: Addr }, 
    EXWithdraw { accountHolder: Addr, execAddr: Addr, token: Denom, amount: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAccountData{ accountHolder: Addr },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
}
