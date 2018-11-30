# inth-oauth2-azure

This provides Azure Active Directory (OpenID Connect) support for the inth-oauth2 crate.

### Example

```rust
extern crate inth_oauth2 as oauth;
extern crate inth_oauth2_azure;

use inth_oauth2_azure::AzureCommon;

let client = oauth::Client::new(
    AzureCommon,
    "client-id".into(),
    "client-secret".into(),
    Some("redirect-uri".into())
);
```