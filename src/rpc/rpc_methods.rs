use crate::{AtollResult, HttpResponse, RequestOutcome, RpcJsonError, RpcResponse};
use core::fmt;
use serde::de::DeserializeOwned;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RpcMethod {
    GetAccountInfo,
    GetBalance,
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

    fn is_ok_or<U: fmt::Debug + DeserializeOwned>(
        &self,
        response_body: &str,
    ) -> AtollResult<RequestOutcome<U>> {
        let outcome = match serde_json::from_str::<RpcResponse<U>>(response_body) {
            Ok(success) => RequestOutcome::Success(success),
            Err(_) => match serde_json::from_str::<RpcJsonError>(response_body) {
                Ok(json_error) => RequestOutcome::InvalidJson(json_error),
                Err(serde_json_error) => return Err(serde_json_error.into()),
            },
        };

        Ok(outcome)
    }

    pub fn to_upper_camel_case(&self) -> &str {
        match self {
            Self::GetAccountInfo => "getAccountInfo",
            Self::GetBalance => "getBalance",
        }
    }
}
