#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::testing::BankQuerier;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    WasmMsg, BankMsg, Coin, Addr, Uint128, BalanceResponse,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, self};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:wba-project";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPLY_SUCCESS_ID: u64 = 123;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(info.sender.as_str())?;

    CONFIG.save(deps.storage, &Config { owner, denom: msg.denom })?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env),
    }
}

fn get_balance(deps: DepsMut, env: Env, config: &Config) -> Result<Coin, ContractError> {
    Ok(deps.querier.query_balance(env.contract.address, &config.denom)?)
}

fn execute_withdraw(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let balance = get_balance(deps, env, &config)?;

    let msg = BankMsg::Send { to_address: config.owner.to_string(), amount: vec![balance] };

    Ok(Response::new().add_message(msg))
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
//     // match msg.id {
//     //     REPLY_SUCCESS_ID => handle_transfer_reply(deps, msg),
//     //     id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
//     // }
//     unimplemented!()
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: DepsMut, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::Balance {} => to_binary(&query_balance(deps, env)?),
    }
}

fn query_balance(deps: DepsMut, env: Env) -> StdResult<BalanceResponse> {
    let config = CONFIG.load(deps.storage)?;
    let balance = get_balance(deps, env, &config).unwrap_or(Coin {amount: Uint128::new(0), denom: config.denom});

    Ok(BalanceResponse{amount: balance})
}

// #[cfg(test)]
// mod tests {}