//Anything related to POST requests for account and it's properties goes here.
use super::{AppTransfer, Credit, PasswordResetResponse, SmsNumber};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Transfer Create
///
/// Create a new app transfer.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create)
pub struct AppTransferCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: AppTransferCreateParams<'a>,
}

impl<'a> AppTransferCreate<'a> {
    pub fn new(app: &'a str, recipient: &'a str) -> AppTransferCreate<'a> {
        AppTransferCreate {
            params: AppTransferCreateParams {
                app: app,
                recipient: recipient,
                silent: None,
            },
        }
    }

    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self.params.silent = Some(silent);
        self
    }

    pub fn build(&self) -> AppTransferCreate<'a> {
        AppTransferCreate {
            params: AppTransferCreateParams {
                app: self.params.app,
                recipient: self.params.recipient,
                silent: self.params.silent,
            },
        }
    }
}

/// Update account with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AppTransferCreateParams<'a> {
    /// unique identifier or name of app
    pub app: &'a str,
    /// unique email address, identifier of an account or implicit reference to currently authorized user
    pub recipient: &'a str,
    /// whether to suppress email notification when transferring apps
    pub silent: Option<bool>,
}

impl<'a> HerokuEndpoint<AppTransfer, (), AppTransferCreateParams<'a>> for AppTransferCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("account/app-transfers")
    }
    fn body(&self) -> Option<AppTransferCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Credit Create
///
/// Create a new credit.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-create)
pub struct AccountCreditCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: AccountCreditCreateParams<'a>,
}

impl<'a> AccountCreditCreate<'a> {
    pub fn new() -> AccountCreditCreate<'a> {
        AccountCreditCreate {
            params: AccountCreditCreateParams {
                code1: None,
                code2: None,
            },
        }
    }
    pub fn code_1(&mut self, code1: &'a str) -> &mut Self {
        self.params.code1 = Some(code1);
        self
    }
    pub fn code_2(&mut self, code2: &'a str) -> &mut Self {
        self.params.code2 = Some(code2);
        self
    }

    pub fn build(&self) -> AccountCreditCreate<'a> {
        AccountCreditCreate {
            params: AccountCreditCreateParams {
                code1: self.params.code1,
                code2: self.params.code2,
            },
        }
    }
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-create-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AccountCreditCreateParams<'a> {
    /// first code from a discount card
    pub code1: Option<&'a str>,
    /// second code from a discount card
    pub code2: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Credit, (), AccountCreditCreateParams<'a>> for AccountCreditCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("account/credits")
    }
    fn body(&self) -> Option<AccountCreditCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Reset Password
///
/// Reset account’s password. This will send a reset password link to the user’s email address.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password)
pub struct PasswordReset<'a> {
    /// The parameters to pass to the Heroku API
    pub params: PasswordResetParams<'a>,
}

impl<'a> PasswordReset<'a> {
    pub fn new(email: &'a str) -> PasswordReset {
        PasswordReset {
            params: PasswordResetParams { email },
        }
    }
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-reset-password-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PasswordResetParams<'a> {
    /// unique email address
    pub email: &'a str, // this isn't optional(inacurate Heroku docs)
}

impl<'a> HerokuEndpoint<PasswordResetResponse, (), PasswordResetParams<'a>> for PasswordReset<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("password-resets")
    }
    fn body(&self) -> Option<PasswordResetParams<'a>> {
        Some(self.params.clone())
    }
}

/// PasswordReset Complete Reset Password
///
/// Complete password reset.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-complete-reset-password)
pub struct PasswordResetConfirm<'a> {
    /// Password token
    pub password_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: PasswordResetConfirmParams<'a>,
}

impl<'a> PasswordResetConfirm<'a> {
    pub fn new(password_id: &'a str) -> PasswordResetConfirm<'a> {
        PasswordResetConfirm {
            password_id,
            params: PasswordResetConfirmParams {
                password: None,
                password_confirmation: None,
            },
        }
    }

    pub fn password(&mut self, password: &'a str) -> &mut Self {
        self.params.password = Some(password);
        self
    }

    pub fn password_confirmation(&mut self, password_confirmation: &'a str) -> &mut Self {
        self.params.password_confirmation = Some(password_confirmation);
        self
    }

    pub fn build(&self) -> PasswordResetConfirm<'a> {
        PasswordResetConfirm {
            password_id: self.password_id,
            params: PasswordResetConfirmParams {
                password: self.params.password,
                password_confirmation: self.params.password_confirmation,
            },
        }
    }
}

/// Update account credits with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#passwordreset-complete-reset-password-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct PasswordResetConfirmParams<'a> {
    /// current password on the account
    pub password: Option<&'a str>,
    /// confirmation of the new password
    pub password_confirmation: Option<&'a str>,
}

impl<'a> HerokuEndpoint<PasswordResetResponse, (), PasswordResetConfirmParams<'a>>
    for PasswordResetConfirm<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("password-resets/{}/actions/finalize", self.password_id)
    }
    fn body(&self) -> Option<PasswordResetConfirmParams<'a>> {
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
