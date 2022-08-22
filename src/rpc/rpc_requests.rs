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
    value: Option<JsonValue>,
    cluster: Cluster,
    extras: Vec<(String, JsonValue)>,
}

impl Default for RpcRequest {
    fn default() -> Self {
        RpcRequest::new()
    }
}

impl RpcRequest {
    pub fn new() -> Self {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: RpcMethod::GetAccountInfo,
            value: Option::None,
            cluster: Cluster::DevNet,
            extras: Vec::default(),
        }
    }

    pub fn change_jsonrpc(mut self, jsonrpc: &str) -> Self {
        self.jsonrpc = jsonrpc.to_owned();

        self
    }

    pub fn add_value(mut self, value: JsonValue) -> Self {
        self.value = Some(value);

        self
    }

    pub fn change_cluster(mut self, cluster: Cluster) -> Self {
        self.cluster = cluster;

        self
    }

    pub fn add_method(mut self, method: RpcMethod) -> Self {
        self.method = method;

        self
    }

    pub fn add_extra(mut self, key: &str, value: JsonValue) -> Self {
        self.extras.push((key.to_owned(), value));

        self
    }

    pub fn change_id(mut self, id: u8) -> Self {
        self.id = id;

        self
    }

    pub async fn request<T: fmt::Debug + DeserializeOwned>(self) -> AtollResult<HttpResponse<T>> {
        let method = self.method.to_upper_camel_case();

        let mut extra_parameters = json::object::Object::new();

        self.extras.into_iter().for_each(|(key, value)| {
            extra_parameters.insert(&key, value);
        });

        let json_body = json::object! {
            jsonrpc: self.jsonrpc,
            id: self.id,
            method: method,
            params: if extra_parameters.is_empty() {
                json::array![
                    self.value,
                ]
            } else {
                    json::array![
                    self.value,
                    extra_parameters
                ]
            }


        }
        .to_string();

        let http_client = minreq::post(self.cluster.url())
            .with_header("Content-Type", "application/json")
            .with_body(json_body)
            .with_timeout(60);

        let response = unblock(|| http_client.send()).await?;

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
    data: Option<String>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Deserialize)]
pub enum RequestOutcome<T> {
    Success(RpcResponse<T>),
    InvalidJson(RpcJsonError),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize, Deserialize)]
pub struct MalformedRequest {
    jsonrpc: String,
    id: u8,
    code: i16,
    message: String,
    data: Option<String>,
}
