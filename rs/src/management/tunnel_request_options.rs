use reqwest::header::{HeaderName, HeaderValue};

use super::Authorization;

#[derive(Default, Clone)]
pub struct TunnelRequestOptions {
    /// Gets or sets authorization for the request.
    ///
    /// Note this should not be a _user_ access token (such as AAD or GitHub); use the
    /// callback parameter to the `TunnelManagementHttpClient` constructor to
    /// supply user access tokens.
    pub authorization: Option<Authorization>,

    /// Gets or sets additional headers to be included in the request.
    pub headers: Vec<(HeaderName, HeaderValue)>,

    /// Gets or sets a flag that requests tunnel ports when retrieving a tunnel object.
    pub include_ports: bool,

    /// Gets or sets an optional list of token scopes that
    /// are requested when retrieving a tunnel or tunnel port object.
    pub token_scopes: Vec<String>,

    /// Gets or sets an optional list of scopes that should be authorized when
    /// retrieving a tunnel or tunnel port object.
    pub scopes: Vec<String>,
}

pub const NO_REQUEST_OPTIONS: &TunnelRequestOptions = &TunnelRequestOptions {
    authorization: None,
    headers: Vec::new(),
    include_ports: false,
    token_scopes: Vec::new(),
    scopes: Vec::new(),
};
