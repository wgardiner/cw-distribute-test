use cosmwasm_std::{
    // to_binary,
    Api, Binary, Env, Extern, HandleResponse, InitResponse, MessageInfo, Querier,
    StdResult, Storage, attr, CosmosMsg, BankMsg,
};

use crate::error::ContractError;
use crate::msg::{
    // CountResponse,
    HandleMsg,
    InitMsg,
    QueryMsg
};
use crate::state::{
    config,
    // config_read,
    State
};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        // count: msg.count,
        owner: deps.api.canonical_address(&info.sender)?,
    };
    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        // HandleMsg::Increment {} => try_increment(deps),
        // HandleMsg::Reset { count } => try_reset(deps, info, count),
        HandleMsg::Donate {} => try_donate(deps),
        HandleMsg::Distribute {} => try_distribute(deps, env, info),
    }

}

pub fn try_donate<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
) -> Result<HandleResponse, ContractError> {
    // config(&mut deps.storage).update(|mut state| -> Result<_, ContractError> {
    //     state.count += 1;
    //     Ok(state)
    // })?;

    Ok(HandleResponse::default())
}

pub fn try_distribute<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
) -> Result<HandleResponse, ContractError> {
    let contract_address = env.contract.address.clone();
    let recipient = info.sender;


    let message = CosmosMsg::Bank(BankMsg::Send {
        from_address: contract_address,
        to_address: recipient,
        amount: deps.querier.query_all_balances(&env.contract.address)?,
        // amount: info.sent_funds,
    });

    let res = HandleResponse {
        messages: vec![message],
        attributes: vec![attr("action", "distribute funds")],
        data: None,
    };

    Ok(res)
}



// pub fn try_increment<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
// ) -> Result<HandleResponse, ContractError> {
//     config(&mut deps.storage).update(|mut state| -> Result<_, ContractError> {
//         state.count += 1;
//         Ok(state)
//     })?;

//     Ok(HandleResponse::default())
// }

// pub fn try_reset<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     info: MessageInfo,
//     count: i32,
// ) -> Result<HandleResponse, ContractError> {
//     let api = &deps.api;
//     config(&mut deps.storage).update(|mut state| -> Result<_, ContractError> {
//         if api.canonical_address(&info.sender)? != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(HandleResponse::default())
// }

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        // QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

// fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CountResponse> {
//     let state = config_read(&deps.storage).load()?;
//     Ok(CountResponse { count: state.count })
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        // let msg = InitMsg { count: 17 };
        let msg = InitMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        // it worked, let's query the state
        // let res = query(&deps, mock_env(), QueryMsg::GetCount {}).unwrap();
        // let res = query(&deps, mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn it_works() {
        let mut deps = mock_dependencies(&[]);

        // let msg = InitMsg { count: 17 };
        let init_msg = InitMsg {};
        let init_info = mock_info("creator", &coins(1, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, mock_env(), init_info, init_msg).unwrap();
        assert_eq!(0, res.messages.len());


        let donor_info = mock_info("donor", &coins(1, "earth"));
        let donor_msg = HandleMsg::Donate {};
        let _donor_res = handle(&mut deps, mock_env(), donor_info, donor_msg).unwrap();

        let distributor_info = mock_info("distributor", &vec![]);
        // let distributor_info = mock_info("distributor", &coins(1, "earth"));
        let distributor_msg = HandleMsg::Distribute {};
        let mut env = mock_env();
        env.block.time = env.block.time + 86400 * 1;
        let distributor_res = handle(&mut deps, env, distributor_info, distributor_msg).unwrap();

        println!("{:?}", distributor_res);
    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies(&coins(2, "token"));

    //     let msg = InitMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = init(&mut deps, mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Increment {};
    //     let _res = handle(&mut deps, mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(&deps, mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies(&coins(2, "token"));

    //     let msg = InitMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = init(&mut deps, mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let res = handle(&mut deps, mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let _res = handle(&mut deps, mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(&deps, mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
