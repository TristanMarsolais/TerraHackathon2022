// Code inspiration for handaling send and receive tokens by https://github.com/Duel-Dojo/The-Duel-Dojo

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw2::set_contract_version;
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:Dlegacy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State {
        creator: info.sender.clone(),
        owner: info.sender,
    };
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        MsgExecute::CreateAccount { accountHolder: Addr, /*execList,*/ ahDateOfLost: Date,  token: Denom, amount: i32 } => try_create_account(deps, info, accountHolder, /*execList,*/ ahDateOfLost, token, amount),
        MsgExecute::AHStillAlive { accountHolder: Addr } => try_ah_still_alive(deps, info, accountHolder), 
        MsgExecute::AHSendFunds { accountHolder: Addr, token: Denom, amount: i32 } => try_ah_send_funds(deps, info, accountHolder, token, amount), 
        MsgExecute::AHWithdraw { accountHolder: Addr } => try_ah_withdraw(deps, info, accountHolder), 
        MsgExecute::EXWithdraw { accountHolder: Addr, /*execAddr: Addr,*/ token: Denom, amount: i32 } => try_ex_withdraw(deps, info, accountHolder, /*execList,*/),
    }
}

pub fn try_create_account() {
    /* 1. Instantiate new ACCOUNT

    fn demo() -> StdResult<()> {
    let mut store = MockStorage::new();
    let data = Data {
        name: "John".to_string(),
        age: 32,
    };
    
    
    */
}
pub fn try_ah_still_alive() {
    // 
}
pub fn try_ah_send_funds() {} 
pub fn try_ah_withdraw() {}
pub fn try_ex_withdraw() {}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_binary("placeholder")?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
}