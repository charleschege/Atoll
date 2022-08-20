use serde::Deserialize;

type Base58String = String;
type Base64String = String;
type Encoding = String;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfo {
    pub data: (Base64String, Encoding),
    pub executable: bool,
    pub lamports: u64,
    pub owner: Base58String,
    pub rent_epoch: u64,
}
