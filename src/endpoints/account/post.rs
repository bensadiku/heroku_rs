//Anything related to POST requests for account and it's properties goes here.
use super::{AppTransfer, Credit, PasswordResetResponse};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Transfer Create
///
/// Create a new app transfer.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create)
pub struct AppTransferCreate {
    /// The parameters to pass to the Heroku API
    pub params: AppTransferCreateParams,
}

/// Update account with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppTransferCreateParams {
    /// unique identifier or name of app
    pub app: String,
    /// unique email address, identifier of an account or implicit reference to currently authorized user
    pub recipient: String,
    /// whether to suppress email notification when transferring apps
    pub silent: Option<bool>,
}

impl HerokuEndpoint<AppTransfer, (), AppTransferCreateParams> for AppTransferCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("account/app-transfers")
    }
    fn body(&self) -> Option<AppTransferCreateParams> {
        Some(self.params.clone())
    }
}

/// Credit Create
///
/// Create a new credit.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-create)
pub struct AccountCreditCreate {
    /// The parameters to pass to the Heroku API
    pub params: AccountCreditCreateParams,
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-create-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AccountCreditCreateParams {
    /// first code from a discount card
    pub code1: Option<String>,
    /// second code from a discount card
    pub code2: Option<String>,
}

impl HerokuEndpoint<Credit, (), AccountCreditCreateParams> for AccountCreditCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("account/credits")
    }
    fn body(&self) -> Option<AccountCreditCreateParams> {
        Some(self.params.clone())
    }
}

/// Reset Password
///
/// Reset account’s password. This will send a reset password link to the user’s email address.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password)
pub struct PasswordReset {
    /// The parameters to pass to the Heroku API
    pub params: PasswordResetParams,
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PasswordResetParams {
    /// unique email address
    pub email: String, // this isn't optional apparently...
}

impl HerokuEndpoint<PasswordResetResponse, (), PasswordResetParams> for PasswordReset {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("password-resets")
    }
    fn body(&self) -> Option<PasswordResetParams> {
        Some(self.params.clone())
    }
}


/// PasswordReset Complete Reset Password
///
/// Complete password reset.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-complete-reset-password)
pub struct PasswordResetConfirm {
    /// Password token 
    pub password_id: String,
    /// The parameters to pass to the Heroku API
    pub params: PasswordResetConfirmParams,
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PasswordResetConfirmParams {
    /// current password on the account
    pub password: String,
    /// confirmation of the new password
    pub password_confirmation: String,
}

impl HerokuEndpoint<PasswordResetResponse, (), PasswordResetConfirmParams> for PasswordResetConfirm {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("password-resets/{}/actions/finalize", self.password_id)
    }
    fn body(&self) -> Option<PasswordResetConfirmParams> {
        Some(self.params.clone())
    }
}
