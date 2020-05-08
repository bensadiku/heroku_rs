extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::account;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    get_account(api_client);
    // patch_account(api_client);
    // delete_account(api_client); //Careful here :)
    // get_user_account(api_client);
    // patch_user_account(api_client);
    // delete_user_account(api_client); //Careful here :)

    // get_account_features(api_client);
    // get_account_feature(api_client);
    // patch_account_feature(api_client);

    // get_account_transfers(api_client);
    // get_account_transfer(api_client);
    // create_account_transfer(api_client);
    // patch_account_transfer(api_client);
    // delete_account_transfer(api_client);

    // create_account_credits(api_client);
    // get_account_credit(api_client);
    // get_account_credits(api_client);

    // reset_account_password(api_client);
    // confirm_password(api_client);

    // get_account_sms_number(api_client);
    // recover_account_number(api_client);
    // confirm_account_number(api_client);

    // get_invoices(api_client);
    // get_invoice(api_client);

    // get_invoice_address(api_client);
    // update_invoice_address(api_client);

    // get_key(api_client);
    // get_keys(api_client);
}

// get key by id or by fingerprint
fn get_key<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let fingerprint = "FINGER_PRINT_HERE";
    let response = api_client.request(&account::KeyDetails::new(fingerprint));
    print_response(response);
}
// get keys
fn get_keys<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::KeyList::new());
    print_response(response);
}

// Update invoice address
fn update_invoice_address<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let update_invoice = &account::InvoiceAddressUpdate::new()
        .address_1("Grove Street")
        .address_2("Not Grove Street")
        .build();
    let response = api_client.request(update_invoice);
    print_response(response);
}

// Get invoice address
fn get_invoice_address<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::InvoiceAddressDetails::new());
    print_response(response);
}

// Get invoice
fn get_invoice<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let invoice_number = "123";
    let response = api_client.request(&account::InvoiceDetails::new(invoice_number));
    print_response(response);
}

// Get invoices
fn get_invoices<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::InvoiceList::new());
    print_response(response);
}

// Confirm account sms number.
fn confirm_account_number<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = "123";
    let response = api_client.request(&account::SmsNumberRecover::new(account_id));
    print_response(response);
}

// Recover account sms number.
fn recover_account_number<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = "123";
    let response = api_client.request(&account::SmsNumberRecover::new(account_id));
    print_response(response);
}

// Get account sms number.
fn get_account_sms_number<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = "123";
    let response = api_client.request(&account::SmsNumberDetails::new(account_id));
    print_response(response);
}

// Confirm password reset.
fn confirm_password<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let password_id = "123";
    let password_confimation = &account::PasswordResetConfirm::new(password_id)
        .password("123")
        .password_confirmation("123")
        .build();
    let response = api_client.request(password_confimation);
    print_response(response);
}

// Reset password.
fn reset_account_password<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let email = "EMAIL";
    let response = api_client.request(&account::PasswordReset::new(email));
    print_response(response);
}

// Get account credits.
fn get_account_credits<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountCreditList {});
    print_response(response);
}

// Get account credit.
fn get_account_credit<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let credit_id = "012abc";

    let response = api_client.request(&account::AccountCreditDetails { credit_id });
    print_response(response);
}

// Create account credits.
fn create_account_credits<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let create_credit = &account::AccountCreditCreate::new()
        .code_1("012abc")
        .code_2("012abcd")
        .build();
    let response = api_client.request(create_credit);
    print_response(response);
}

// Delete heroku account app transfer.
fn delete_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferDelete::new("ID_HERE"));
    print_response(response);
}

// Patch heroku account app transfer.
fn patch_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferUpdate::new("TRANFER_ID", "declined"));
    print_response(response);
}

// Create heroku account app transfer.
fn create_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_id = "APP_ID_HERE";
    let recipient = "ID_OR_EMAIL_HERE";
    let transfer = &account::AppTransferCreate::new(app_id, recipient)
        .silent(false)
        .build();

    let response = api_client.request(transfer);
    print_response(response);
}

// Get heroku account app transfer.
fn get_account_transfer<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferDetails::new("transfer_id"));
    print_response(response);
}

// Get heroku account app transfers.
fn get_account_transfers<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AppTransferList {});
    print_response(response);
}

// Patch a specidic heroku account feature.
fn patch_account_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let feature_id = "team-internal-routing";
    let enable = true;

    let response = api_client.request(&account::AccountFeatureUpdate::new(feature_id, enable));
    print_response(response);
}

// Get a specidic heroku account feature.
fn get_account_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountFeatureDetails::new(
        "team-internal-routing",
    ));
    print_response(response);
}

// Get heroku account features.
fn get_account_features<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountFeatureList {});
    print_response(response);
}

// Delete heroku user account. NOTE that this action cannot be undone.
fn delete_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = "USER_ID_OR_EMAIL";

    let response = api_client.request(&account::UserAccountDelete { account_id });
    print_response(response);
}

// Patch heroku user account
fn patch_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = "USER_ID_OR_EMAIL";
    let account_patch = &account::UserAccountUpdate::new(account_id)
        .beta(false)
        .allow_tracking(true)
        .name("yet-another-name")
        .build();
    let response = api_client.request(account_patch);
    print_response(response);
}

// Get heroku user account by email or id
fn get_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::UserAccountDetails::new("USER_ID_OR_EMAIL"));
    print_response(response);
}

// Delete heroku account. NOTE that this action cannot be undone.
fn delete_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountDelete {});
    print_response(response);
}

// Patch heroku account
fn patch_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let patch = &account::AccountUpdate::new()
        .allow_tracking(true)
        .beta(false)
        .name("heroku-examples-new-name")
        .build();
    let response = api_client.request(patch);

    print_response(response);
}

// Get heroku account
fn get_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountDetails {});
    print_response(response);
}
