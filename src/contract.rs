#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

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
) -> Result<Response, ContractError> {
    // Instantiate Storage object

    set_contract_version(deps.Storage, CONTRACT_NAME, CONTRACT_VERSION)

    // save storage

    // Ok(Response)
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
        MsgExecute::ModifyAccount { accountHolder: Addr, /*execList,*/ ahDateOfLost: Date } => try_modify_account(deps, info, accountHolder, /*execList,*/ ahDateOfLost),
        MsgExecute::AHStillAlive { accountHolder: Addr } => try_ah_still_alive(deps, info, accountHolder), 
        MsgExecute::AHSendFunds { accountHolder: Addr, token: Denom, amount: i32 } => try_ah_send_funds(deps, info, accountHolder, token, amount), 
        MsgExecute::AHWithdraw { accountHolder: Addr } => try_ah_withdraw(deps, info, accountHolder), 
        MsgExecute::EXStillAlive { accountHolder: Addr, /*execAddr: Addr*/ } => try_exe_still_alive(deps, info, accountHolder, /*execList,*/), 
        MsgExecute::EXWithdraw { accountHolder: Addr, /*execAddr: Addr,*/ token: Denom, amount: i32 } => try_ex_withdraw(deps, info, accountHolder, /*execList,*/),
    }
}

pub fn try_create_account() {
    
}
pub fn try_modify_account() {}
pub fn try_ah_still_alive() {}
pub fn try_ah_send_funds() {} 
pub fn try_ah_withdraw() {}
pub fn try_exe_still_alive() {}
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