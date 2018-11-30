//! This provides Azure Active Directory (OpenID Connect) support for the inth-oauth2 crate.
//!
//! # Example
//! 
//! ```rust
//! extern crate inth_oauth2 as oauth;
//! extern crate inth_oauth2_azure;
//! 
//! use inth_oauth2_azure::AzureCommon;
//! 
//! let client = oauth::Client::new(
//!     AzureCommon,
//!     "client-id".into(),
//!     "client-secret".into(),
//!     Some("redirect-uri".into())
//! );
//! ```
//! 
//! Azure provides multiple endpoints which can be used depending on the type of end user
//! you're wishing to authenticate. [More info...](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#fetch-the-openid-connect-metadata-document)

extern crate inth_oauth2;
extern crate url;

#[macro_use]
extern crate lazy_static;

use url::Url;
use inth_oauth2::provider::Provider;
use inth_oauth2::token::{Bearer, Refresh};

lazy_static! {
    // Users with either a personal or organisation MSFT account can sign in with these
    static ref COMMON_AUTH_URL: Url = Url::parse("https://login.microsoftonline.com/common/oauth2/v2.0/authorize").unwrap();
    static ref COMMON_TOKEN_URL: Url = Url::parse("https://login.microsoftonline.com/common/oauth2/v2.0/token").unwrap();

    // Only users with an organisation (work/school) account can sign in with these
    static ref ORGANIZATIONS_AUTH_URL: Url = Url::parse("https://login.microsoftonline.com/organizations/oauth2/v2.0/authorize").unwrap();
    static ref ORGANIZATIONS_TOKEN_URL: Url = Url::parse("https://login.microsoftonline.com/organizations/oauth2/v2.0/token").unwrap();

    // Only users wtih a personal account can sign in with these
    static ref CONSUMERS_AUTH_URL: Url = Url::parse("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize").unwrap();
    static ref CONSUMERS_TOKEN_URL: Url = Url::parse("https://login.microsoftonline.com/consumers/oauth2/v2.0/token").unwrap();
}

/// Users with both a personal Microsoft account and a work or school account from Azure Active Directory (Azure AD) can sign in to the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AzureCommon;

impl Provider for AzureCommon {
    type Lifetime = Refresh;
    type Token = Bearer<Self::Lifetime>;

    fn auth_uri(&self) -> &Url { &COMMON_AUTH_URL }
    fn token_uri(&self) -> &Url { &COMMON_TOKEN_URL }
}

/// Only users with work or school accounts from Azure AD can sign in to the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AzureOrganization;

impl Provider for AzureOrganization {
    type Lifetime = Refresh;
    type Token = Bearer<Self::Lifetime>;

    fn auth_uri(&self) -> &Url { &ORGANIZATIONS_AUTH_URL }
    fn token_uri(&self) -> &Url { &ORGANIZATIONS_TOKEN_URL }
}

/// Only users with a personal Microsoft account can sign in to the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AzureConsumer;

impl Provider for AzureConsumer {
    type Lifetime = Refresh;
    type Token = Bearer<Self::Lifetime>;

    fn auth_uri(&self) -> &Url { &CONSUMERS_AUTH_URL }
    fn token_uri(&self) -> &Url { &CONSUMERS_TOKEN_URL }
}

/// Only users with a work or school account from a specific Azure AD tenant can
/// sign in to the application. Either the friendly domain name of the Azure AD tenant
/// or the tenant's GUID identifier can be used.
/// 
/// Eg `8eaef023-2b34-4da1-9baa-8bc8c9d6a490` or `contoso.onmicrosoft.com`
/// 
/// ```rust
/// let client = oauth::Client::new(
///     AzureTenant::new("8eaef023-2b34-4da1-9baa-8bc8c9d6a490"),
///     "client-id".into(),
///     "client-secret".into(),
///     "redirect-uri".into()
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzureTenant {
    auth_uri: Url,
    token_uri: Url
}

impl AzureTenant {
    pub fn new(id: &str) -> Self {
        let auth_uri = Url::parse( &format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", id) ).unwrap();
        let token_uri = Url::parse( &format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", id) ).unwrap();

        Self { auth_uri, token_uri }
    }
}

impl Provider for AzureTenant {
    type Lifetime = Refresh;
    type Token = Bearer<Self::Lifetime>;

    fn auth_uri(&self) -> &Url { &self.auth_uri }
    fn token_uri(&self) -> &Url { &self.token_uri }
}