use web3utilities::UtilitiesError;

use borsh::{BorshDeserialize, BorshSerialize};

pub type AtollResult<T> = Result<T, AtollError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub enum AtollError {
    Utilities(UtilitiesError),
    /// The method is not supported by this library.
    /// File a bug report if the method should exist
    UnsupportedSolanaRpcMethod,
    /// Http Errors from the `minreq` crate used for HTTP requests
    Http(Minreq),
    SerdeJsonDeser(String),
}

/// Errors from the minreq crate
/// Manual implementation provides Comparison and Clone operations
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize)]
pub enum Minreq {
    /// The response body contains invalid UTF-8, so the `as_str()`
    /// conversion failed.
    InvalidUtf8InBody(String),
    /// Ran into a rustls error while creating the connection.
    RustlsCreateConnection(String),
    /// Couldn't parse the incoming chunk's length while receiving a
    /// response with the header `Transfer-Encoding: chunked`.
    MalformedChunkLength,
    /// The chunk did not end after reading the previously read amount
    /// of bytes.
    MalformedChunkEnd,
    /// Couldn't parse the `Content-Length` header's value as an
    /// `usize`.
    MalformedContentLength,
    /// The response contains headers whose total size surpasses
    HeadersOverflow,
    /// The response's status line length surpasses
    StatusLineOverflow,
    /// [ToSocketAddrs](std::net::ToSocketAddrs) did not resolve to an
    /// address.
    AddressNotFound,
    /// The response was a redirection, but the `Location` header is
    /// missing.
    RedirectLocationMissing,
    /// The response redirections caused an infinite redirection loop.
    InfiniteRedirectionLoop,
    /// Redirections, won't follow any more.
    TooManyRedirections,
    /// The response contained invalid UTF-8 where it should be valid
    /// (eg. headers), so the response cannot interpreted correctly.
    InvalidUtf8InResponse,
    /// The provided url contained a domain that has non-ASCII
    /// characters, and could not be converted into punycode. It is
    /// probably not an actual domain.
    PunycodeConversionFailed,
    /// Tried to send a secure request (ie. the url started with
    /// `https://`), but the crate's `https` feature was not enabled,
    /// and as such, a connection cannot be made.
    HttpsFeatureNotEnabled,
    /// The provided url contained a domain that has non-ASCII
    /// characters, but it could not be converted into punycode
    /// because the `punycode` feature was not enabled.
    PunycodeFeatureNotEnabled,
    /// The provided proxy information was not properly formatted.
    /// Supported proxy format is `[user:password@]host:port`.
    BadProxy,
    /// The provided credentials were rejected by the proxy server.
    BadProxyCreds,
    /// The provided proxy credentials were malformed.
    ProxyConnect,
    /// The provided credentials were rejected by the proxy server.
    InvalidProxyCreds,

    /// This is a special error case, one that should never be
    /// returned! Think of this as a cleaner alternative to calling
    /// `unreachable!()` inside the library. If you come across this,
    /// please open an issue in the minreq crate repository, and include the string inside this
    /// error, as it can be used to locate the problem.
    Other(String),
}

impl From<minreq::Error> for AtollError {
    fn from(minreq_error: minreq::Error) -> Self {
        match minreq_error {
            minreq::Error::InvalidUtf8InBody(utf8_error) => {
                AtollError::Http(Minreq::InvalidUtf8InBody(utf8_error.to_string()))
            }
            minreq::Error::RustlsCreateConnection(rustls_error) => {
                AtollError::Http(Minreq::RustlsCreateConnection(rustls_error.to_string()))
            }
            minreq::Error::IoError(io_error) => AtollError::Utilities(io_error.into()),
            minreq::Error::MalformedChunkLength => AtollError::Http(Minreq::MalformedChunkLength),
            minreq::Error::MalformedChunkEnd => AtollError::Http(Minreq::MalformedChunkEnd),
            minreq::Error::MalformedContentLength => {
                AtollError::Http(Minreq::MalformedContentLength)
            }
            minreq::Error::HeadersOverflow => AtollError::Http(Minreq::HeadersOverflow),
            minreq::Error::StatusLineOverflow => AtollError::Http(Minreq::StatusLineOverflow),
            minreq::Error::AddressNotFound => AtollError::Http(Minreq::AddressNotFound),
            minreq::Error::RedirectLocationMissing => {
                AtollError::Http(Minreq::RedirectLocationMissing)
            }
            minreq::Error::InfiniteRedirectionLoop => {
                AtollError::Http(Minreq::InfiniteRedirectionLoop)
            }
            minreq::Error::TooManyRedirections => AtollError::Http(Minreq::TooManyRedirections),
            minreq::Error::InvalidUtf8InResponse => AtollError::Http(Minreq::InvalidUtf8InResponse),
            minreq::Error::PunycodeConversionFailed => {
                AtollError::Http(Minreq::PunycodeConversionFailed)
            }
            minreq::Error::HttpsFeatureNotEnabled => {
                AtollError::Http(Minreq::HttpsFeatureNotEnabled)
            }
            minreq::Error::PunycodeFeatureNotEnabled => {
                AtollError::Http(Minreq::PunycodeFeatureNotEnabled)
            }
            minreq::Error::BadProxy => AtollError::Http(Minreq::BadProxy),
            minreq::Error::BadProxyCreds => AtollError::Http(Minreq::BadProxyCreds),
            minreq::Error::ProxyConnect => AtollError::Http(Minreq::ProxyConnect),
            minreq::Error::InvalidProxyCreds => AtollError::Http(Minreq::InvalidProxyCreds),
            minreq::Error::Other(other_error) => {
                AtollError::Http(Minreq::Other(other_error.to_owned()))
            }
        }
    }
}

impl From<serde_json::Error> for AtollError {
    fn from(error: serde_json::Error) -> Self {
        AtollError::SerdeJsonDeser(error.to_string())
    }
}
