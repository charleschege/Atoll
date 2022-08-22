use crate::{TransactionError, TransactionResult};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub block_height: u64,
    pub block_time: u64,
    pub blockhash: String,
    pub parent_slot: u64,
    pub previous_blockhash: String,
    pub rewards: Vec<Rewards>,
    pub transactions: Vec<TxWithMeta>,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Rewards {
    #[serde(rename = "pubkey")]
    pub public_key: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: RewardType,
    pub commission: Option<u8>,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TxWithMeta {
    pub meta: TxMetadata,
    pub transaction: (String, String),
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TxMetadata {
    pub err: Option<TransactionError>,
    pub status: TransactionResult<()>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub inner_instructions: Vec<InnerInstructions>,
    pub log_messages: Vec<String>,
    pub pre_token_balances: Vec<TokenBalances>,
    pub post_token_balances: Vec<TokenBalances>,
    pub rewards: Vec<Rewards>,
    pub loaded_addresses: Option<LoadedAddresses>,
    pub return_data: Option<TransactionReturnData>,
    pub compute_units_consumed: Option<u64>,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub ui_token_amount: TokenAmount,
    pub owner: Option<String>,
    pub program_id: Option<String>,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalances {
    pub account_index: u8,
    pub mint: String,
    pub owner: String,
    pub ui_token_amount: TokenAmount,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: f64,
    pub ui_amount_string: String,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct InnerInstructions {
    pub index: u8,
    pub instructions: Vec<Instruction>,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct AccountMeta {
    #[serde(rename = "pubkey")]
    pub public_key: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct LoadedAddresses {
    pub writable: Vec<String>,
    pub readonly: Vec<String>,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReturnData {
    pub program_id: String,
    pub data: Vec<u8>,
}
