//Anything related to POST requests for account and it's properties goes here.
use super::{AppTransfer, Credit, PasswordResetResponse, SmsNumber};

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

impl AppTransferCreate {
    pub fn new(app: String, recipient: String, silent: Option<bool>) -> AppTransferCreate {
        AppTransferCreate {
            params: AppTransferCreateParams {
                app: app,
                recipient: recipient,
                silent: silent,
            },
        }
    }

    pub fn create(app: String, recipient: String) -> AppTransferCreate {
        AppTransferCreate {
            params: AppTransferCreateParams {
                app: app,
                recipient: recipient,
                silent: None,
            },
        }
    }
}

/// Update account with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create-required-parameters)
#[serde_with::skip_serializing_none]
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

impl AccountCreditCreate {
    pub fn new(code1: Option<String>, code2: Option<String>) -> AccountCreditCreate {
        AccountCreditCreate {
            params: AccountCreditCreateParams { code1, code2 },
        }
    }
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-create-optional-parameters)
#[serde_with::skip_serializing_none]
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

impl PasswordReset {
    pub fn new(email: String) -> PasswordReset {
        PasswordReset {
            params: PasswordResetParams { email },
        }
    }
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password-optional-parameters)
#[serde_with::skip_serializing_none]
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

impl PasswordResetConfirm {
    pub fn new(
        password_id: String,
        password: String,
        password_confirmation: String,
    ) -> PasswordResetConfirm {
        PasswordResetConfirm {
            password_id,
            params: PasswordResetConfirmParams {
                password,
                password_confirmation,
            },
        }
    }
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct PasswordResetConfirmParams {
    /// current password on the account
    pub password: String,
    /// confirmation of the new password
    pub password_confirmation: String,
}

impl HerokuEndpoint<PasswordResetResponse, (), PasswordResetConfirmParams>
    for PasswordResetConfirm
{
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

/// SMS Number Recover
///
/// Recover an account using an SMS recovery code
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password)
pub struct SmsNumberRecover<'a> {
    /// unique identifier, email or account id
    pub account_id: &'a str,
}

impl<'a> SmsNumberRecover<'a> {
    pub fn new(account_id: &'a str) -> SmsNumberRecover<'a> {
        SmsNumberRecover { account_id }
    }
}

impl<'a> HerokuEndpoint<SmsNumber> for SmsNumberRecover<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("users/{}/sms-number/actions/recover", self.account_id)
    }
}

/// SMS Number Confirm
///
/// Confirm an SMS number change with a confirmation code
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sms-number-confirm)
pub struct SmsNumberConfirm<'a> {
    /// unique identifier, email or account id
    pub account_id: &'a str,
}

impl<'a> SmsNumberConfirm<'a> {
    pub fn new(account_id: &'a str) -> SmsNumberConfirm<'a> {
        SmsNumberConfirm { account_id }
    }
}

impl<'a> HerokuEndpoint<SmsNumber> for SmsNumberConfirm<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("users/{}/sms-number/actions/confirm", self.account_id)
    }
}
