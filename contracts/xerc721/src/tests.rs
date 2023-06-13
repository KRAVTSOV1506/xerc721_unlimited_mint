use crate::contract::{execute, instantiate, query};
use crate::execution::{Cw721ExecuteMsg, Cw721QueryMsg};
use cw721::{NftInfoResponse, OwnerOfResponse};
use new_crosstalk_sample::xerc721::{ExecuteMsg, InstantiateMsg, QueryMsg};
use router_wasm_bindings::types::RequestMetaData;
use router_wasm_bindings::RouterMsg;

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{CosmosMsg, Deps, Empty, Env, MessageInfo, Response, StdError, Uint128};

use cosmwasm_std::from_binary;
use cosmwasm_std::DepsMut;
use cosmwasm_std::OwnedDeps;
use std::marker::PhantomData;

const SENDER: &str = "router1sxc6t9uh9u8f252gl3yqetlt6qmp2syx3v0p3w";

fn get_mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData::default(),
    }
}

fn do_instantiate(mut deps: DepsMut) {
    let instantiate_msg = InstantiateMsg {
        name: "ERC721".into(),
        symbol: "ERC721".into(),
        public_key: "6a99e543f5eb501d51995083161e3b75528d77d26d62d83cda5439b057653d78".into(),
    };
    let info = mock_info(SENDER, &[]);
    let env = mock_env();
    let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

fn set_remote_contract(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    chain_id: String,
    remote_contract: String,
) {
    let extension_msg = ExecuteMsg::EnrollRemoteContract {
        chain_id: chain_id.clone(),
        remote_address: remote_contract.clone(),
    };
    let enroll_msg = cw721_base::ExecuteMsg::Extension { msg: extension_msg };
    let res = execute(deps, env.clone(), _info, enroll_msg.clone());
    assert!(res.is_ok());
}

fn get_nft_owner_of(deps: Deps, env: Env, token_id: String) -> Result<OwnerOfResponse, StdError> {
    let query_msg = Cw721QueryMsg::OwnerOf {
        token_id,
        include_expired: Some(false),
    };
    let owner_of = query(deps, env, query_msg);
    match owner_of {
        Ok(brr) => from_binary(&brr),
        Err(_) => Err(StdError::NotFound { kind: "nft".into() }),
    }
}

fn get_nft_info(
    deps: Deps,
    env: Env,
    token_id: String,
) -> Result<NftInfoResponse<Empty>, StdError> {
    let query_msg = Cw721QueryMsg::NftInfo { token_id };
    let nft_info = query(deps, env, query_msg);
    match nft_info {
        Ok(brr) => from_binary(&brr),
        Err(_) => Err(StdError::NotFound { kind: "nft".into() }),
    }
}

#[test]
fn test_basic() {
    let mut deps = get_mock_dependencies();
    do_instantiate(deps.as_mut());
}

#[test]
fn test_enroll_and_get_remote_contract() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(SENDER, &[]);
    let chain_id = "cosmos".to_string();
    let remote_contract = "wasm1kjd9yyyqx0jwfzzy9ls32vuuyfem38x2lg2y0g".to_string();

    do_instantiate(deps.as_mut());

    set_remote_contract(
        deps.as_mut(),
        env.clone(),
        info,
        chain_id.clone(),
        remote_contract.clone(),
    );
    // Get remote contract
    let extension_msg = QueryMsg::GetRemoteContract { chain_id };
    let query_msg = Cw721QueryMsg::Extension { msg: extension_msg };
    let res = query(deps.as_ref(), env.clone(), query_msg.clone());
    let remote_contract_result: String = from_binary(&res.unwrap()).unwrap();

    // Check if remote contract is set
    assert_eq!(remote_contract_result, remote_contract);
}

#[test]
fn test_mint_nft() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(SENDER, &[]);

    do_instantiate(deps.as_mut());

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/Qma49KCamwSberpE8whTz3ESk81PoGtLsimmj3YhsWJKPX".to_string(), 
        signature: "4676479df870e8fdb75b3b24fbe1ca7748bce5bfa9a6621087603d3ac97c1b5a00da3768adaefbf1ecd88d6ab576bb42c1c7325666349ecf7487385286757a09".to_string(), 
        owner: SENDER.to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_ok());
}

#[test]
fn test_can_not_mint_nft_with_wrong_sign() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(SENDER, &[]);

    do_instantiate(deps.as_mut());

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/Qma49KCamwSberpE8whTz3ESk81PoGtLsimmj3YhsWJKPX".to_string(), 
        signature: "4676479df870e8fdb75b3b24fbe1ca7748bce5bfa9a6621087603d3ac97c1b5a00da3768adaefbf1ecd88d6ab576bb42c1c7325666349ecf7487385286757a01".to_string(), 
        owner: SENDER.to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_err());
}

#[test]
fn test_can_not_mint_nft_twice() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(SENDER, &[]);

    do_instantiate(deps.as_mut());

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/Qma49KCamwSberpE8whTz3ESk81PoGtLsimmj3YhsWJKPX".to_string(), 
        signature: "4676479df870e8fdb75b3b24fbe1ca7748bce5bfa9a6621087603d3ac97c1b5a00da3768adaefbf1ecd88d6ab576bb42c1c7325666349ecf7487385286757a09".to_string(), 
        owner: SENDER.to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_ok());

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/QmakZtb3uKouz5QFGTaUvjN3akJDkAgGGSLh61aqEUnGt1".to_string(), 
        signature: "c2623cdaf5e714b7f7e64ae0ebb49b5ddeb199215016395a75ac6a81985a1dc97d2fe6f79ca4e5f7305ce49d9ad8c11a4ce2db1d499fdcc2f841fae20bf4b90c".to_string(), 
        owner: SENDER.to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_err());
}

#[test]
fn test_two_mint_nft() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(SENDER, &[]);

    do_instantiate(deps.as_mut());

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/Qma49KCamwSberpE8whTz3ESk81PoGtLsimmj3YhsWJKPX".to_string(), 
        signature: "4676479df870e8fdb75b3b24fbe1ca7748bce5bfa9a6621087603d3ac97c1b5a00da3768adaefbf1ecd88d6ab576bb42c1c7325666349ecf7487385286757a09".to_string(), 
        owner: SENDER.to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_ok());

    let response = get_nft_info(deps.as_ref(), env.clone(), "0".into());
    assert!(response.is_ok());

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/QmakZtb3uKouz5QFGTaUvjN3akJDkAgGGSLh61aqEUnGt1".to_string(), 
        signature: "c2623cdaf5e714b7f7e64ae0ebb49b5ddeb199215016395a75ac6a81985a1dc97d2fe6f79ca4e5f7305ce49d9ad8c11a4ce2db1d499fdcc2f841fae20bf4b90c".to_string(), 
        owner: "router1apapk9zfz3rp4x87fsm6h0s3zd0wlmkz0fx8tx".to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_ok());

    let response = get_nft_info(deps.as_ref(), env.clone(),"1".into());
    assert!(response.is_ok());
}

#[test]
fn test_transfer_crosschain() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(SENDER, &[]);
    let remote_contract = "wasm1kjd9yyyqx0jwfzzy9ls32vuuyfem38x2lg2y0g".to_string();

    do_instantiate(deps.as_mut());
    set_remote_contract(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "1".into(),
        remote_contract,
    );

    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/Qma49KCamwSberpE8whTz3ESk81PoGtLsimmj3YhsWJKPX".to_string(), 
        signature: "4676479df870e8fdb75b3b24fbe1ca7748bce5bfa9a6621087603d3ac97c1b5a00da3768adaefbf1ecd88d6ab576bb42c1c7325666349ecf7487385286757a09".to_string(), 
        owner: SENDER.to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_ok());

    let request_metadata: RequestMetaData = RequestMetaData {
        dest_gas_limit: 0,
        ack_gas_limit: 0,
        dest_gas_price: 0,
        ack_gas_price: 0,
        relayer_fee: Uint128::from(0u32),
        ack_type: router_wasm_bindings::types::AckType::AckOnBoth,
        is_read_call: false,
        asm_address: "".into(),
    };
    let response = get_nft_info(deps.as_ref(), env.clone(), "0".into());
    assert!(response.is_ok());

    let respone = get_nft_owner_of(deps.as_ref(), env.clone(), "0".into());
    assert!(response.is_ok());
    assert_eq!(respone.unwrap().owner, SENDER);

    let ext_cc_msg = ExecuteMsg::TransferCrossChain {
        dst_chain_id: "1".into(),
        token_id: 0,
        recipient: "0x1C609537a32630c054202e2B089B9Da268667C5D".to_string(),
        request_metadata,
    };
    let exec_msg = Cw721ExecuteMsg::Extension { msg: ext_cc_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone());
    assert!(res.is_ok());

    if let Ok(result) = res {
        let _ok = match result.messages[0].msg.clone() {
            CosmosMsg::Custom(msg) => match msg {
                RouterMsg::CrosschainCall {
                    version: _,
                    route_amount: _,
                    route_recipient: _,
                    request_packet: _,
                    request_metadata: _,
                    dest_chain_id: _,
                } => {
                    // in order to verify encoded payload
                    // println!("{:?}", hex::encode(request_packet));

                    //Binary(hex::decode(op).unwrap())
                    Ok(Response::<RouterMsg>::new())
                }
            },
            _ => Err(StdError::NotFound {
                kind: "isend".into(),
            }),
        };
    }
    // nft should be burned with id 0
    let response = get_nft_info(deps.as_ref(), env.clone(), "0".into());
    assert!(response.is_err());

    // try mint after cross chain transfer
    let mint_msg = ExecuteMsg::MintToken { 
        token_uri: "https://ipfs.io/ipfs/Qma49KCamwSberpE8whTz3ESk81PoGtLsimmj3YhsWJKPX".to_string(), 
        signature: "4676479df870e8fdb75b3b24fbe1ca7748bce5bfa9a6621087603d3ac97c1b5a00da3768adaefbf1ecd88d6ab576bb42c1c7325666349ecf7487385286757a09".to_string(), 
        owner: "router1apapk9zfz3rp4x87fsm6h0s3zd0wlmkz0fx8tx".to_string(), 
    };
    let mint_msg = Cw721ExecuteMsg::Extension{ msg: mint_msg };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), mint_msg.clone());
    assert!(res.is_ok());

    let response = get_nft_info(deps.as_ref(), env.clone(), "1".into());
    assert!(response.is_ok());
}
