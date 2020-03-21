extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::apps;
use heroku_rs::endpoints::dynos;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    // let app_name = String::from("heroku-rs-tests");

    // create_app(api_client);
    // delete_app(api_client);
    // patch_app(api_client);
    // get_app(api_client);
    // list_apps(api_client);
    // list_account_apps(api_client);
    // get_dyno(api_client);
    // list_dynos(api_client);
    // restart_dyno(api_client);
    // restart_all_dynos(api_client);

    // enable_app_acm(api_client, app_name);
    // disable_app_acm(api_client, app_name);
    // refresh_app_acm(api_client, app_name);
    // get_app_features(api_client, app_name);
    // get_app_feature(api_client, app_name);
    // patch_app_feature(api_client, app_name);

    // create_app_webhook(api_client, app_name);
    // get_app_webhooks(api_client, app_name);
    // get_app_webhook(api_client, app_name);
    // patch_app_webhook(api_client, app_name);
    // delete_app_webhook(api_client, app_name);

    // get_app_webhook_delivery(api_client, app_name);
    // get_app_webhook_deliveries(api_client, app_name);
}

/// Gets a list of webhook deliveries.
fn get_app_webhook_deliveries<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppWebhookDeliveryList {
        app_identifier: app_name,
    });
    print_response(response);
}

/// Gets details about a specific webhook delivery.
fn get_app_webhook_delivery<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let webhook_id = String::from("WEBHOOK_DELIVERY_ID");
    let response = api_client.request(&apps::AppWebhookDetails {
        app_identifier: app_name,
        webhook_identifier: webhook_id,
    });
    print_response(response);
}

/// Patch a specific webhook.
fn patch_app_webhook<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let webhook_id = String::from("WEBHOOK_ID");
    let webhook_include = vec!["api:release".to_owned()];
    let webhook_level = String::from("notify");
    let webhook_url = String::from("https://www.bing.com");
    let response = api_client.request(&apps::AppWebhookUpdate {
        app_identifier: app_name,
        webhook_identifier: webhook_id,
        params: apps::AppWebhookUpdateParams {
            authorization: None,
            include: Some(webhook_include),
            level: Some(webhook_level),
            secret: None,
            url: Some(webhook_url),
        },
    });
    print_response(response);
}

/// Gets details about a specific webhook.
fn get_app_webhook<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let webhook_id = String::from("WEBHOOK_ID");
    let response = api_client.request(&apps::AppWebhookDetails {
        app_identifier: app_name,
        webhook_identifier: webhook_id,
    });
    print_response(response);
}

/// Gets a list of all webhooks that are available in the specified app.
fn get_app_webhooks<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppWebhookList {
        app_identifier: app_name,
    });
    print_response(response);
}

/// Delete a specific app webhook by id
fn delete_app_webhook<ApiClientType: HerokuApiClient>(
    api_client: &ApiClientType,
    app_name: String,
) {
    let webhook_id = String::from("WEBHOOK_ID");
    let response = api_client.request(&apps::AppWebhookDelete {
        app_identifier: app_name,
        webhook_identifier: webhook_id,
    });
    print_response(response);
}

/// Create a new app webhook
fn create_app_webhook<ApiClientType: HerokuApiClient>(
    api_client: &ApiClientType,
    app_name: String,
) {
    let response = api_client.request(&apps::AppWebhookCreate {
        app_identifier: app_name,
        params: apps::AppWebhookCreateParams {
            authorization: None,
            include: vec!["api:release".to_owned()],
            level: String::from("notify"),
            secret: None,
            url: String::from("https://www.google.com"),
        },
    });
    print_response(response);
}

fn patch_app_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppFeatureUpdate {
        app_identifier: app_name,
        feature_identifier: String::from("spaces-dns-discovery"),
        params: apps::AppFeatureUpdateParams { enabled: false },
    });
    print_response(response);
}

fn get_app_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppFeatureDetails {
        app_identifier: app_name,
        feature_identifier: String::from("spaces-dns-discovery"),
    });
    print_response(response);
}

fn get_app_features<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppFeatureList {
        app_identifier: app_name,
    });
    print_response(response);
}

fn refresh_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppRefreshAcm {
        app_identifier: app_name,
    });
    print_response(response);
}

fn disable_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppDisableAcm {
        app_identifier: app_name,
    });
    print_response(response);
}

fn enable_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppEnableAcm {
        app_identifier: app_name,
    });
    print_response(response);
}

fn patch_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = String::from("heroku-rs-tests");

    let response = api_client.request(&apps::AppUpdate {
        app_identifier: app_name,
        params: apps::AppUpdateParams {
            build_stack: None,
            maintenance: Some(false),
            name: None,
        },
    });
    print_response(response);
}

fn delete_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = String::from("heroku-rs-tests");

    let response = api_client.request(&apps::AppDelete {
        app_identifier: app_name,
    });
    print_response(response);
}

fn create_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = Some(String::from("heroku-rs-tests"));

    let response = api_client.request(&apps::AppCreate {
        params: apps::AppCreateParams {
            name: app_name,
            region: None,
            stack: None,
        },
    });
    print_response(response);
}

fn get_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let id = String::from("heroku-rs-tests");

    let response = api_client.request(&apps::AppDetails { identifier: id });
    print_response(response);
}

fn list_account_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let resp = api_client.request(&apps::AccountAppList {
        account_identifier: String::from("my-heroku-email@here.io"),
    });
    print_response(resp);
}

fn list_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let resp = api_client.request(&apps::AppList {});
    print_response(resp);
}

fn get_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("heroku-rs-tests");
    let dyno_id = String::from("web.1");

    let response = api_client.request(&dynos::DynoDetails {
        app_identifier: application_id,
        identifier: dyno_id,
    });
    print_response(response);
}

fn list_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("testing-nell-bot");

    let resp = api_client.request(&dynos::DynoList {
        app_identifier: application_id,
    });
    print_response(resp);
}

fn restart_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("heroku-rs-tests");
    let dyno_id = String::from("web.1");

    let resp = api_client.request(&dynos::DynoRestart {
        app_identifier: application_id,
        identifier: dyno_id,
    });
    print_response(resp);
}

fn restart_all_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("heroku-rs-tests");

    let resp = api_client.request(&dynos::DynoAllRestart {
        app_identifier: application_id,
    });
    print_response(resp);

}
