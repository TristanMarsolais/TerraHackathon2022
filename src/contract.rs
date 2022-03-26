// Code inspiration for handaling send and receive tokens by https://github.com/Duel-Dojo/The-Duel-Dojo

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage,
};

use cosmwasm_storage::{singleton, Singleton};
use cw2::set_contract_version;
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg, Denom};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Executer, GenericBalance, State, Testament, Time, ACCOUNT};

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

    // config(deps.storage).save(&state)?;
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
        ExecuteMsg::CreateAccount {
            accountHolder,
            executers,
            /*execList,*/ ahDateOfLost,
            token,
            amount,
        } => try_create_account(
            deps,
            info,
            accountHolder,
            /*execList,*/ ahDateOfLost,
            token,
            amount,
        ),
        ExecuteMsg::AHStillAlive { accountHolder } => try_ah_still_alive(deps, info, accountHolder),
        ExecuteMsg::AHSendFunds {
            accountHolder,
            token,
            amount,
        } => try_ah_send_funds(deps, info, accountHolder, token, amount),
        ExecuteMsg::AHWithdraw { accountHolder } => try_ah_withdraw(deps, info, accountHolder),
        ExecuteMsg::EXWithdraw {
            accountHolder,
            execAddr,
            token,
            amount,
        } => try_ex_withdraw(deps, info, accountHolder /*execList,*/),
        ExecuteMsg::ModifyAccount {
            accountHolder,
            executers,
            ahDateOfLost,
        } => todo!(),
        ExecuteMsg::EXStillAlive {
            accountHolder,
            execAddr,
        } => todo!(),
    }
}

pub static CONFIG_KEY: &[u8] = b"config";

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn try_create_account(
    deps: DepsMut,
    info: MessageInfo,
    accountHolder: Addr,
    /*execList: Executer,*/ ahDateOfLost: Time,
    token: Denom,
    amount: i32,
) -> Result<Response, ContractError> {
    /* 1. Instantiate new ACCOUNT

    fn demo() -> StdResult<()> {
    let mut store = MockStorage::new();
    let data = Data {
        name: "John".to_string(),
        age: 32,
    };


    */

    let state = config(deps.storage).load()?;

    let testament = Testament {
        countdown_for_execution: 0,
        executer: Executer {
            executor_addr: Addr::unchecked("empty"),
            time_to_collect: 10,
        },
        vault: GenericBalance::new(),
    };

    ACCOUNT.update(deps.storage, info.sender.clone(), |exists| match exists {
        None => Ok(exists.unwrap()),
        Some(account) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res =
        Response::new().add_attributes(vec![("action", "create"), ("id", info.sender.as_str())]);
    Ok(res)
}

pub fn try_ah_still_alive(
    deps: DepsMut,
    info: MessageInfo,
    accountHolder: Addr,
) -> Result<Response, ContractError> {
    //
    Ok(Response::default())
}

pub fn try_ah_send_funds(
    deps: DepsMut,
    info: MessageInfo,
    accountHolder: Addr,
    token: Denom,
    amount: i32,
) -> Result<Response, ContractError> {
    //
    Ok(Response::default())
}

pub fn try_ah_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    accountHolder: Addr,
) -> Result<Response, ContractError> {
    //
    Ok(Response::default())
}
pub fn try_ex_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    accountHolder: Addr,
    /*execList: Executer,*/
) -> Result<Response, ContractError> {
    //
    Ok(Response::default())
}

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
