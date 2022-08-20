use crate::{AtollResult, RpcMethod};
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use json::JsonValue;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smol::unblock;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RpcRequest {
    jsonrpc: String,
    id: u8,
    method: RpcMethod,
    value: JsonValue,
    commitment: Commitment,
    cluster: Cluster,
    encoding: Encoding,
}

impl RpcRequest {
    pub fn new(value: JsonValue) -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: RpcMethod::GetAccountInfo,
            value,
            commitment: Commitment::Finalized,
            cluster: Cluster::DevNet,
            encoding: Encoding::Base64,
        }
    }

    pub fn change_jsonrpc(mut self, jsonrpc: &str) -> Self {
        self.jsonrpc = jsonrpc.to_owned();

        self
    }

    pub fn change_cluster(mut self, cluster: Cluster) -> Self {
        self.cluster = cluster;

        self
    }

    pub fn change_commitment(mut self, commitment: Commitment) -> Self {
        self.commitment = commitment;

        self
    }

    pub fn add_method(mut self, method: RpcMethod) -> Self {
        self.method = method;

        self
    }

    pub fn change_id(mut self, id: u8) -> Self {
        self.id = id;

        self
    }

    pub fn change_encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;

        self
    }

    pub async fn request<T: fmt::Debug + DeserializeOwned>(self) -> AtollResult<HttpResponse<T>> {
        let commitment: &str = self.commitment.into();
        let encoding: &str = self.encoding.into();
        let method = self.method.to_upper_camel_case();

        let json_body = json::object! {
            jsonrpc: self.jsonrpc,
            id: self.id,
            method: method,
            params: json::array![
                self.value,
                json::object! {
                    commitment: commitment,
                    encoding: encoding,
                }
            ]
        }
        .to_string();

        let http_client = minreq::post(self.cluster.url())
            .with_header("Content-Type", "application/json")
            .with_body(json_body)
            .with_timeout(60);

        let response = unblock(|| http_client.send()).await?;

        dbg!(&response.as_str());

        self.method.parse(response).await
    }
}

/// Configures the Solana RPC cluster to connect to
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum Cluster {
    /// A locally run Solana test validator
    LocalNet,
    /// Connect to the developer cluster
    DevNet,
    /// Connect to the testnet cluster for staging
    TestNet,
    /// Connect to the production cluster
    MainNetBeta,
}

impl Cluster {
    /// Convert the cluster selected to a URL
    pub fn url<'a>(&self) -> &'a str {
        match self {
            Cluster::LocalNet => "https://127.0.0.1:8899",
            Cluster::DevNet => "https://api.devnet.solana.com",
            Cluster::TestNet => "https://api.testnet.solana.com",
            Cluster::MainNetBeta => "https://api.mainnet-beta.solana.com",
        }
    }
}

impl Default for Cluster {
    fn default() -> Self {
        Cluster::DevNet
    }
}

/// The commitment metric aims to give clients a measure of the network confirmation
/// and stake levels on a particular block.
/// It implements `From<&str>` and `Into<&str>`
#[derive(
    Debug,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Copy,
    Clone,
)]
pub enum Commitment {
    /// A block is processed by RPC servers
    Processed,
    /// A block is has been confirmed
    Confirmed,
    /// A block has been finalized
    Finalized,
    /// The commitment level provided is invalid
    InvalidCommitment,
}

impl Default for Commitment {
    fn default() -> Self {
        Commitment::Finalized
    }
}

impl From<&str> for Commitment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "processed" => Commitment::Processed,
            "confirmed" => Commitment::Confirmed,
            "finalized" => Commitment::Finalized,
            _ => Commitment::InvalidCommitment,
        }
    }
}

impl<'a> Into<&'a str> for Commitment {
    fn into(self) -> &'a str {
        match self {
            Commitment::Processed => "processed",
            Commitment::Confirmed => "confirmed",
            Commitment::Finalized => "finalized",
            Commitment::InvalidCommitment => "invalid_commitment",
        }
    }
}

/// The encoding for the data format
#[derive(
    Debug,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Copy,
    Clone,
)]
pub enum Encoding {
    /// Base58 encoding
    Base58,
    /// Base64 Encoding
    Base64,
    /// The encoding provided is not supported yer
    UnsupportedEncoding,
}

impl From<&str> for Encoding {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "base58" => Encoding::Base58,
            "base64" => Encoding::Base64,
            _ => Encoding::UnsupportedEncoding,
        }
    }
}

impl<'a> Into<&'a str> for Encoding {
    fn into(self) -> &'a str {
        match self {
            Encoding::Base58 => "base58",
            Encoding::Base64 => "base64",
            Encoding::UnsupportedEncoding => "unsupported_encoding",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse<T> {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub reason_phrase: String,
    pub body: RequestOutcome<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u8,
    pub result: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResult<U> {
    pub context: Context,
    pub value: Option<U>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub api_version: String,
    pub slot: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize, Deserialize)]
pub struct RpcJsonError {
    jsonrpc: String,
    id: u8,
    error: JsonError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize, Deserialize)]
pub struct JsonError {
    code: i16,
    message: String,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Deserialize)]
pub enum RequestOutcome<T> {
    Success(RpcResponse<T>),
    InvalidJson(RpcJsonError),
}
