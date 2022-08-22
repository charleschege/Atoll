use crate::{AtollError, AtollResult, HttpResponse, RequestOutcome, RpcJsonError, RpcResponse};
use core::fmt;
use serde::de::DeserializeOwned;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RpcMethod {
    GetAccountInfo,
    GetBalance,
    GetBlock,
    GetBlockHeight,
}

impl RpcMethod {
    pub async fn parse<T: fmt::Debug + DeserializeOwned>(
        &self,
        response: minreq::Response,
    ) -> AtollResult<HttpResponse<T>> {
        let response_body = response.as_str()?;

        let http_response = match self {
            Self::GetAccountInfo => {
                self.build_http_response::<T>(&response, self.is_ok_or::<T>(response_body)?)
            }
            Self::GetBalance => {
                self.build_http_response::<T>(&response, self.is_ok_or::<T>(response_body)?)
            }
            Self::GetBlock => {
                self.build_http_response::<T>(&response, self.is_ok_or::<T>(response_body)?)
            }
            Self::GetBlockHeight => {
                self.build_http_response::<T>(&response, self.is_ok_or::<T>(response_body)?)
            }
        };

        Ok(http_response)
    }

    fn build_http_response<T>(
        &self,
        response: &minreq::Response,
        body: RequestOutcome<T>,
    ) -> HttpResponse<T> {
        HttpResponse {
            status_code: response.status_code as u16,
            headers: response.headers.clone(),
            reason_phrase: response.reason_phrase.clone(),
            body,
        }
    }

    pub fn is_ok_or<T: fmt::Debug + DeserializeOwned>(
        &self,
        response_body: &str,
    ) -> AtollResult<RequestOutcome<T>> {
        let jd = &mut serde_json::Deserializer::from_str(response_body);

        let result: Result<RpcResponse<T>, _> = serde_path_to_error::deserialize(jd);
        match result {
            Ok(success) => Ok(RequestOutcome::Success(success)),
            Err(serde_path_error) => match serde_json::from_str::<RpcJsonError>(response_body) {
                Ok(json_error) => Ok(RequestOutcome::InvalidJson(json_error)),
                Err(_) => Err(AtollError::SerdeJsonDeser(serde_path_error.to_string())),
            },
        }
    }

    pub fn to_upper_camel_case(&self) -> &str {
        match self {
            Self::GetAccountInfo => "getAccountInfo",
            Self::GetBalance => "getBalance",
            Self::GetBlock => "getBlock",
            Self::GetBlockHeight => "getBlockHeight",
        }
    }
}
