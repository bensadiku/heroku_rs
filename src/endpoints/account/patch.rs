//Anything related to PATCH requests for account and it's properties goes here.
use super::{Account, AccountFeature, AppTransfer};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Update
///
/// Update account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update)
///
/// # Example:
///
/// AccountUpdate has no required parameters and returns the [`Account`][response] that was just updated.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let account_patch = &AccountUpdate::new()
///     .beta(false)
///     .allow_tracking(true)
///     .name("Tina Edmonds")
///     .build();
/// let response = api_client.request(account_patch);
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Account.html
pub struct AccountUpdate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: AccountUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AccountUpdate<'a> {
    pub fn new() -> AccountUpdate<'a> {
        AccountUpdate {
            params: AccountUpdateParams {
                allow_tracking: None,
                beta: None,
                name: None,
            },
        }
    }

    /// # allow_tracking: whether to allow third party web activity tracking
    /// 
    /// # default: true
    pub fn allow_tracking(&mut self, allow_tracking: bool) -> &mut Self {
        self.params.allow_tracking = Some(allow_tracking);
        self
    }

    /// # beta: whether allowed to utilize beta Heroku features.
    pub fn beta(&mut self, beta: bool) -> &mut Self {
        self.params.beta = Some(beta);
        self
    }

    /// # name: full name of the account owner
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn build(&self) -> AccountUpdate<'a> {
        AccountUpdate {
            params: AccountUpdateParams {
                allow_tracking: self.params.allow_tracking,
                beta: self.params.beta,
                name: self.params.name,
            },
        }
    }
}

/// Update account with parameters.
///
/// All three paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AccountUpdateParams<'a> {
    /// whether to allow third party web activity tracking, by default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tracking: Option<bool>,
    /// whether allowed to utilize beta Heroku features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    /// full name of the account owner [Nullable]
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Account, (), AccountUpdateParams<'a>> for AccountUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account",)
    }
    fn body(&self) -> Option<AccountUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Account Update By User
///
/// Update account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update-by-user)
///
/// # Example:
///
/// UserAccountUpdate has one required parameter, account_id, and returns the [`Account`][response] that was just updated.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
/// let account_id = "USER_ID_OR_EMAIL";
/// let account_patch = &UserAccountUpdate::new(account_id)
///     .beta(false)
///     .allow_tracking(true)
///     .name("Tina Edmonds")
///     .build();
/// let response = api_client.request(account_patch);
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Account.html
pub struct UserAccountUpdate<'a> {
    /// account_id can be the account email or id.
    pub account_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: UserAccountUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> UserAccountUpdate<'a> {
    pub fn new(account_id: &'a str) -> UserAccountUpdate<'a> {
        UserAccountUpdate {
            account_id,
            params: UserAccountUpdateParams {
                allow_tracking: None,
                beta: None,
                name: None,
            },
        }
    }

    /// # allow_tracking: whether to allow third party web activity tracking
    /// 
    /// # default: true
    pub fn allow_tracking(&mut self, allow_tracking: bool) -> &mut Self {
        self.params.allow_tracking = Some(allow_tracking);
        self
    }

    /// # beta: whether allowed to utilize beta Heroku features.
    pub fn beta(&mut self, beta: bool) -> &mut Self {
        self.params.beta = Some(beta);
        self
    }

    /// # name: full name of the account owner
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn build(&self) -> UserAccountUpdate<'a> {
        UserAccountUpdate {
            account_id: self.account_id,
            params: UserAccountUpdateParams {
                allow_tracking: self.params.allow_tracking,
                beta: self.params.beta,
                name: self.params.name,
            },
        }
    }
}

/// Update user account with parameters.
///
/// All three paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update-by-user-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct UserAccountUpdateParams<'a> {
    /// whether to allow third party web activity tracking, by default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tracking: Option<bool>,
    /// whether allowed to utilize beta Heroku features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    /// full name of the account owner [Nullable]
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Account, (), UserAccountUpdateParams<'a>> for UserAccountUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_id)
    }
    fn body(&self) -> Option<UserAccountUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Account Feature Update
///
/// Update an existing account feature.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature-update)
///
/// # Example:
///
/// AccountFeatureUpdate has two required parameters, feature_id and enabled, and returns the [`AccountFeature`][response] that was just updated.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
/// let feature_id = "FEATURE_NAME_OR_ID";
/// let account_patch = &AccountFeatureUpdate::new(feature_id, true);
/// let response = api_client.request(account_patch);
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AccountFeature.html
pub struct AccountFeatureUpdate<'a> {
    /// feature_id can be the feature name or id.
    pub feature_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: AccountFeatureUpdateParams,
}

#[cfg(feature = "builder")]
impl<'a> AccountFeatureUpdate<'a> {
    pub fn new(feature_id: &'a str, enabled: bool) -> AccountFeatureUpdate {
        AccountFeatureUpdate {
            feature_id,
            params: AccountFeatureUpdateParams { enabled },
        }
    }
}

/// Update account feature with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AccountFeatureUpdateParams {
    /// whether or not account feature has been enabled
    pub enabled: bool,
}

impl<'a> HerokuEndpoint<AccountFeature, (), AccountFeatureUpdateParams>
    for AccountFeatureUpdate<'a>
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account/features/{}", self.feature_id)
    }
    fn body(&self) -> Option<AccountFeatureUpdateParams> {
        Some(self.params.clone())
    }
}

/// App Transfer Update
///
/// Update an existing app transfer.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-update)
///
/// # Example:
///
/// AppTransferUpdate has two required parameters, transfer_id and state, and returns the [`AppTransfer`][response] that was just updated.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
/// let transfer_id = "TANSFER_NAME_OR_ID";
/// let state = "pending";
/// let account_patch = &AppTransferUpdate::new(transfer_id, state);
/// let response = api_client.request(account_patch);
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppTransfer.html
pub struct AppTransferUpdate<'a> {
    /// unique identifier or the transfer name
    pub transfer_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: AppTransferUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AppTransferUpdate<'a> {
    pub fn new(transfer_id: &'a str, state: &'a str) -> AppTransferUpdate<'a> {
        AppTransferUpdate {
            transfer_id,
            params: AppTransferUpdateParams { state },
        }
    }
}

/// Update account app transfer with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppTransferUpdateParams<'a> {
    /// the current state of an app transfer, one of:"pending" or "accepted" or "declined"
    pub state: &'a str,
}

impl<'a> HerokuEndpoint<AppTransfer, (), AppTransferUpdateParams<'a>> for AppTransferUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account/app-transfers/{}", self.transfer_id)
    }
    fn body(&self) -> Option<AppTransferUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
