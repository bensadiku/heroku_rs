extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::apps;
use heroku_rs::endpoints::builds;
use heroku_rs::endpoints::collaborators;
use heroku_rs::endpoints::domains;
use heroku_rs::endpoints::dynos;
use heroku_rs::endpoints::formations;
use heroku_rs::framework::apiclient::HerokuApiClient;
use std::collections::HashMap;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = String::from("heroku-rs-tests");

    // create_app(api_client, app_name);
    // delete_app(api_client, app_name); // Careful here :)
    // patch_app(api_client, app_name);
    // get_app(api_client, app_name);
    // list_apps(api_client);
    // list_account_apps(api_client);
    // get_dyno(api_client, app_name);
    // list_dynos(api_client, app_name);
    // create_dyno_simple(api_client, app_name);
    // create_dyno_complex(api_client, app_name);
    // list_dynos(api_client, app_name);
    // restart_dyno(api_client);
    // restart_all_dynos(api_client, app_name);

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

    // create_app_build(api_client, app_name);
    // get_app_builds(api_client, app_name);
    // get_app_build(api_client, app_name);
    // delete_app_build(api_client, app_name);

    // update_buildpack_installation(api_client, app_name);
    // get_buildpack_installations(api_client, app_name);

    // create_app_collaborate(api_client, app_name);
    // get_app_collaborators(api_client, app_name);
    // get_app_collaborator(api_client, app_name);
    // delete_app_collaborator(api_client, app_name);

    // create_app_domain(api_client, app_name);
    // get_app_domains(api_client, app_name);
    // get_app_domain(api_client, app_name);
    // delete_app_domain(api_client, app_name);

    // get_app_formation(api_client, app_name);
    // list_app_formations(api_client, app_name);
}

/// Delete domain
fn delete_app_domain<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let domain_id = String::from("DOMAIN_ID_OR_HOSTNAME");
    let response = api_client.request(&domains::DomainDelete { app_id, domain_id });
    print_response(response);
}

/// Get domain
fn get_app_domain<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let domain_id = String::from("DOMAIN_ID_OR_HOSTNAME");
    let response = api_client.request(&domains::DomainDetails { app_id, domain_id });
    print_response(response);
}

/// Get domains list
fn get_app_domains<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let response = api_client.request(&domains::DomainList { app_id });
    print_response(response);
}

/// Create domain
fn create_app_domain<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let hostname = String::from("heroku-rs.tests.com");
    let response = api_client.request(&domains::DomainCreate {
        app_id,
        params: domains::DomainCreateParams { hostname },
    });
    print_response(response);
}

/// Delete app collaborator
fn delete_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let collaborator_id = String::from("COLLAB_EMAIL_OR_ID");
    let response = api_client.request(&collaborators::CollaboratorDelete {
        app_id,
        collaborator_id,
    });
    print_response(response);
}

/// Get app collaborator
fn get_app_collaborator<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let collaborator_id = String::from("COLLAB_EMAIL_OR_ID");
    let response = api_client.request(&collaborators::CollaboratorDetails {
        app_id,
        collaborator_id,
    });
    print_response(response);
}

/// Get a list of app collaborators
fn get_app_collaborators<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let response = api_client.request(&collaborators::CollaboratorList { app_id });
    print_response(response);
}

/// Create build pack installations
fn create_app_collaborate<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let user = String::from("EMAIL_or_ID_HERE");
    let silent = Some(false);
    let response = api_client.request(&collaborators::CollaboratorCreate {
        app_id,
        params: collaborators::CollaboratorCreateParams { user, silent },
    });
    print_response(response);
}

/// Get a list of build pack installations
fn get_buildpack_installations<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let response = api_client.request(&builds::BuildPackInstallationList { app_id });
    print_response(response);
}

/// Update build pack installations
fn update_buildpack_installation<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let buildpack = String::from("https://github.com/heroku/heroku-buildpack-ruby");
    let response = api_client.request(&builds::BuildpackInstallationUpdate {
        app_id,
        params: builds::BuildpackInstallationUpdateParams {
            updates: vec![builds::Update { buildpack }],
        },
    });
    print_response(response);
}

/// Delete build cache
fn delete_app_build<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let response = api_client.request(&builds::BuildDelete { app_id });
    print_response(response);
}

/// Gets info about a specific build
fn get_app_build<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let build_id = String::from("Build_ID");
    let response = api_client.request(&builds::BuildDetails {
        app_id,
        build_id,
    });
    print_response(response);
}

/// Gets a list of builds
fn get_app_builds<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let response = api_client.request(&builds::BuildList { app_id });
    print_response(response);
}

/// Create a new build
fn create_app_build<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&builds::BuildCreate {
        app_id: app_name,
        params: builds::BuildCreateParams {
            buildpacks: None,
            source_blob: builds::SourceBlob {
                checksum: Some(String::from(
                    "SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                )),
                url: String::from("https://example.com/source.tgz?token=xyz"),
                version: Some(String::from("2")),
            },
        },
    });
    print_response(response);
}

/// Gets a list of webhook deliveries.
fn get_app_webhook_deliveries<ApiClientType: HerokuApiClient>(
    api_client: &ApiClientType,
    app_name: String,
) {
    let response = api_client.request(&apps::AppWebhookDeliveryList {
        app_id: app_name,
    });
    print_response(response);
}

/// Gets details about a specific webhook delivery.
fn get_app_webhook_delivery<ApiClientType: HerokuApiClient>(
    api_client: &ApiClientType,
    app_name: String,
) {
    let webhook_id = String::from("WEBHOOK_DELIVERY_ID");
    let response = api_client.request(&apps::AppWebhookDetails {
        app_id: app_name,
        webhook_id: webhook_id,
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
        app_id: app_name,
        webhook_id: webhook_id,
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
        app_id: app_name,
        webhook_id: webhook_id,
    });
    print_response(response);
}

/// Gets a list of all webhooks that are available in the specified app.
fn get_app_webhooks<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppWebhookList {
        app_id: app_name,
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
        app_id: app_name,
        webhook_id: webhook_id,
    });
    print_response(response);
}

/// Create a new app webhook
fn create_app_webhook<ApiClientType: HerokuApiClient>(
    api_client: &ApiClientType,
    app_name: String,
) {
    let response = api_client.request(&apps::AppWebhookCreate {
        app_id: app_name,
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
        app_id: app_name,
        feature_id: String::from("spaces-dns-discovery"),
        params: apps::AppFeatureUpdateParams { enabled: false },
    });
    print_response(response);
}

fn get_app_feature<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppFeatureDetails {
        app_id: app_name,
        feature_id: String::from("spaces-dns-discovery"),
    });
    print_response(response);
}

fn get_app_features<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppFeatureList {
        app_id: app_name,
    });
    print_response(response);
}

fn refresh_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppRefreshAcm {
        app_id: app_name,
    });
    print_response(response);
}

fn disable_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppDisableAcm {
        app_id: app_name,
    });
    print_response(response);
}

fn enable_app_acm<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_name: String) {
    let response = api_client.request(&apps::AppEnableAcm { app_id: app_name });
    print_response(response);
}

fn patch_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let response = api_client.request(&apps::AppUpdate {
        app_id,
        params: apps::AppUpdateParams {
            build_stack: None,
            maintenance: Some(false),
            name: None,
        },
    });
    print_response(response);
}

fn delete_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let response = api_client.request(&apps::AppDelete { app_id });
    print_response(response);
}

fn create_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let response = api_client.request(&apps::AppCreate {
        params: apps::AppCreateParams {
            name: Some(app_id),
            region: None,
            stack: None,
        },
    });
    print_response(response);
}

fn get_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let response = api_client.request(&apps::AppDetails { app_id });
    print_response(response);
}

fn list_account_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("my-heroku-email@here.io");
    let resp = api_client.request(&apps::AccountAppList { account_id });
    print_response(resp);
}

fn list_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let resp = api_client.request(&apps::AppList {});
    print_response(resp);
}

fn get_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let dyno_id = String::from("web.1");

    let response = api_client.request(&dynos::DynoDetails { app_id, dyno_id });
    print_response(response);
}

fn list_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let resp = api_client.request(&dynos::DynoList { app_id });
    print_response(resp);
}

fn create_dyno_simple<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let response = api_client.request(&dynos::DynoCreate {
        app_id,
        params: dynos::DynoCreateParams {
            command: "bash".to_string(),
            attach: None,
            env: None,
            force_no_tty: None,
            size: None,
            time_to_live: None,
            r#type: None,
        },
    });

    print_response(response);
}

fn create_dyno_complex<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let mut custom_env_vars = HashMap::new();

    custom_env_vars.insert(
        "COLUMNS".to_string(),
        "80".to_string()
    );

    custom_env_vars.insert(
        "LINES".to_string(),
        "24".to_string(),
    );

    let response = api_client.request(&dynos::DynoCreate {
        app_id: app_id,
        params: dynos::DynoCreateParams {
            command: "bash".to_string(),
            attach: Some(true),
            env: Some(custom_env_vars),
            force_no_tty: None,
            size: None,
            time_to_live: None,
            r#type: None,
        },
    });

    print_response(response);
}

fn restart_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_id = String::from("heroku-rs-tests");
    let dyno_id = String::from("web.1");

    let resp = api_client.request(&dynos::DynoRestart { app_id, dyno_id });
    print_response(resp);
}

fn restart_all_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String ) {
    let resp = api_client.request(&dynos::DynoAllRestart { app_id });
    print_response(resp);
}

fn list_app_formations<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let resp = api_client.request(&formations::FormationList { app_id });
    print_response(resp);
}

fn get_app_formation<ApiClientType: HerokuApiClient>(api_client: &ApiClientType, app_id: String) {
    let resp = api_client.request(&formations::FormationDetails { app_id, formation_id: "web".to_string() });
    print_response(resp);
}
