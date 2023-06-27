use crate::{Deserialize, Serialize};
use cosmwasm_std::{CustomMsg, StdResult};
use router_wasm_bindings::{
    ethabi::{ethereum_types::U256, ParamType, Token},
    types::{ChainType, RequestMetaData},
    utils::convert_address_from_string_to_bytes,
    Bytes,
};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransferParams {
    pub nft_id: u64,
    pub recipient: String,
    pub uri: String,
}

impl TransferParams {
    pub fn get_evm_encoding(&self) -> StdResult<Token> {
        let token_id = Token::Uint(U256::from(self.nft_id.clone()));
        let recipient: Bytes = convert_address_from_string_to_bytes(
            self.recipient.clone(),
            ChainType::ChainTypeEvm.get_chain_code(), //:: this will not always be evm
        )?;
        let uri = Token::String(self.uri.clone());

        Ok(Token::Tuple(vec![token_id, Token::Bytes(recipient), uri]))
    }
    pub fn get_params_types() -> ParamType {
        return ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Bytes, ParamType::String]);
    }
    pub fn from_token_tuple(tuple: Vec<Token>) -> StdResult<Self> {
        let nft_id = tuple[0].clone().into_uint().unwrap().as_u64();

        let has_prefix = true;
        let prefix = if has_prefix { "0x" } else { "" };
        let recipient = format!(
            "{}{}",
            prefix,
            tuple[1]
                .clone()
                .into_bytes()
                .unwrap()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );
        let uri = tuple[2].clone().into_string().unwrap().to_ascii_lowercase();
        Ok(Self {
            nft_id,
            recipient,
            uri,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    EnrollRemoteContract {
        chain_id: String,
        remote_address: String,
    },
    TransferCrossChain {
        dst_chain_id: String,
        token_id: u64,
        recipient: String,
        request_metadata: RequestMetaData,
    },
    MintToken {
        token_uri: String,
        signature: String,
    },
}

impl CustomMsg for ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    GetOwner {},
    GetRemoteContract { chain_id: String },
    IsAlreadyMinted { owner: String }
}

impl CustomMsg for QueryMsg {}
