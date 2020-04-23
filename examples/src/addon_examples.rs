extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::addons;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<T: HerokuApiClient>(api_client: &T) {
    let addon_id = "123";

    // get_addons(api_client);
    get_addon(api_client, addon_id);
    // get_addon_by_app(api_client, addon_id);
    // get_addon_by_account(api_client);
    // get_addon_by_team(api_client);
    // delete_addon(api_client, addon_id); // Careful here :)
    // update_addon(api_client, addon_id);
    // create_addon(api_client);
    // create_addon_resolution(api_client, addon_id);

    // provision_addon(api_client, addon_id);
    // deprovision_addon(api_client, addon_id);

    // create_addon_attachment(api_client, addon_id);
    // get_addon_attachment(api_client);
    // get_addon_attachments(api_client);
    // get_addon_attachments_by_addon(api_client, addon_id);
    // get_addon_attachments_by_app(api_client);
    // get_addon_attachment_by_app(api_client);
    // delete_addon_attachment(api_client); // Careful here :)

    // create_addon_attachment_resolution(api_client);
    // get_addon_config_list(api_client, addon_id);
    // update_addon_config(api_client, addon_id);
    // get_addon_regions_list(api_client);
    // get_addon_regions_list_by_service(api_client);
    // get_addon_regions_list_by_region(api_client);

    // get_addon_services(api_client);
    // get_addon_service(api_client);

    // create_addon_webhook(api_client, addon_id);
    // delete_addon_webhook(api_client, addon_id); // Careful here :)
    // get_addon_webhooks(api_client, addon_id);
    // get_addon_webhook(api_client, addon_id);
    // update_addon_webhook(api_client, addon_id);

    // get_webhook_delivery(api_client, addon_id);
    // get_webhook_deliveries(api_client, addon_id);

    // get_webhook_events(api_client, addon_id);
    // get_webhook_event(api_client, addon_id);
}

// get addon webhook event
fn get_webhook_event<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let event_id = "123";
    let response = api_client.request(&addons::WebhookEventDetails::new(addon_id, event_id));
    print_response(response);
}

// get addon webhook events
fn get_webhook_events<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::WebhookEventList::new(addon_id));
    print_response(response);
}

// get addon webhook deliveries
fn get_webhook_deliveries<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::WebhookDeliveryList::new(addon_id));
    print_response(response);
}

// get addon webhook delivery
fn get_webhook_delivery<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let delivery_id = "123";
    let response = api_client.request(&addons::WebhookDeliveryDetails::new(addon_id, delivery_id));
    print_response(response);
}

// update addon webhook
fn update_addon_webhook<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let webhook_id = "123";
    let webhook_include = vec!["api:release"];
    let response = api_client.request(&addons::WebhookUpdate::new(
        addon_id,
        webhook_id,
        None,
        Some(webhook_include),
        None,
        None,
        None,
    ));
    print_response(response);
}

// get addon webhook
fn get_addon_webhook<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let webhook_id = "123";
    let response = api_client.request(&addons::WebhookDetails::new(addon_id, webhook_id));
    print_response(response);
}

// get addon webhooks
fn get_addon_webhooks<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::WebhookList::new(addon_id));
    print_response(response);
}

// delete addon service
fn delete_addon_webhook<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let webhook_id = "123";
    let response = api_client.request(&addons::WebhookDelete::new(addon_id, webhook_id));
    print_response(response);
}

// create a new webhookaddon
fn create_addon_webhook<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let webhook_include = vec!["api:release"];
    let webhook_level = "notify";
    let webhook_url = "https://www.bing.com";
    let response = api_client.request(&addons::WebhookCreate::create(
        addon_id,
        webhook_include,
        webhook_level,
        webhook_url,
    ));
    print_response(response);
}

// get addon service
fn get_addon_service<T: HerokuApiClient>(api_client: &T) {
    let service_id = "8c38feae-1a02-4d15-a0a5-3b244cd7b37a";
    let response = api_client.request(&addons::AddonServiceDetails::new(service_id));
    print_response(response);
}

// get addon services
fn get_addon_services<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&addons::AddonServiceList::new());
    print_response(response);
}

// get addon region capabilities by region
fn get_addon_regions_list_by_region<T: HerokuApiClient>(api_client: &T) {
    let region_id = "0b344f4e-c515-4277-8685-8d147e41a378";
    let response = api_client.request(&addons::RegionCapabilityListByRegion::new(region_id));
    print_response(response);
}

// get addon region capabilities by service
fn get_addon_regions_list_by_service<T: HerokuApiClient>(api_client: &T) {
    let service_id = "aef68011-778e-43ba-8329-c67b1d5a265c";
    let response = api_client.request(&addons::RegionCapabilityListByService::new(service_id));
    print_response(response);
}

// get addon region capabilities
fn get_addon_regions_list<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&addons::RegionCapabilityList::new());
    print_response(response);
}

// update addon configs
fn update_addon_config<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let config_key = String::from("key");
    let config_value = String::from("value");
    let config = vec![addons::AddonConfig {
        name: config_key,
        value: config_value,
    }];
    let response = api_client.request(&addons::AddonConfigUpdate::new(addon_id, config));
    print_response(response);
}

// get addon configs
fn get_addon_config_list<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::AddonConfigList::new(addon_id));
    print_response(response);
}

// create heroku addon attachment resolution
fn create_addon_attachment_resolution<T: HerokuApiClient>(api_client: &T) {
    let attachment_id = "123";
    let response = api_client.request(&addons::AttachmentResolutionCreate::create(attachment_id));
    print_response(response);
}

// delete heroku addon attachment
fn delete_addon_attachment<T: HerokuApiClient>(api_client: &T) {
    let attachment_id = "123";
    let response = api_client.request(&addons::AttachmentDelete::new(attachment_id));
    print_response(response);
}

// get heroku addon attachment by app
fn get_addon_attachment_by_app<T: HerokuApiClient>(api_client: &T) {
    let app_id = "123";
    let attachment_id = "123";
    let response = api_client.request(&addons::AttachmentDetailsByApp::new(app_id, attachment_id));
    print_response(response);
}

// get heroku addon attachments by app
fn get_addon_attachments_by_app<T: HerokuApiClient>(api_client: &T) {
    let app_id = "123";
    let response = api_client.request(&addons::AttachmentListByApp::new(app_id));
    print_response(response);
}

// get heroku addon attachments by addon
fn get_addon_attachments_by_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::AttachmentListByAddon::new(addon_id));
    print_response(response);
}

// get heroku addon attachments
fn get_addon_attachments<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&addons::AttachmentList::new());
    print_response(response);
}

// get heroku addon attachment
fn get_addon_attachment<T: HerokuApiClient>(api_client: &T) {
    let attachment_id = "123";
    let response = api_client.request(&addons::AttachmentDetails::new(attachment_id));
    print_response(response);
}

// Create heroku addon attachment
fn create_addon_attachment<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AttachmentCreate::create(addon_id, app_id));
    print_response(response);
}

// Mark an add-on as deprovisioned for use.
fn deprovision_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::AddonActionDeprovision::new(addon_id));
    print_response(response);
}

// Mark an add-on as provisioned for use.
fn provision_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::AddonActionProvision::new(addon_id));
    print_response(response);
}

// Create heroku addon resolution
fn create_addon_resolution<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AddonResolutionCreate::create(addon_id));
    print_response(response);
}

// Create heroku addon
fn create_addon<T: HerokuApiClient>(api_client: &T) {
    let app_id = "123";
    let plan = "heroku-postgresql:dev";
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AddonCreate::create(app_id, plan));
    print_response(response);
}

// Update heroku addon
fn update_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    let plan = "heroku-postgresql:dev";
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AddonUpdate::create(app_id, addon_id, plan));
    print_response(response);
}

// Delete heroku addon
fn delete_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    let response = api_client.request(&addons::AddonDelete::new(app_id, addon_id));
    print_response(response);
}

// Get heroku addons by team
fn get_addon_by_team<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&addons::AddonListByTeam::new(team_id));
    print_response(response);
}

// Get heroku addons by account
fn get_addon_by_account<T: HerokuApiClient>(api_client: &T) {
    let account_id = "123";
    let response = api_client.request(&addons::AddonListByAccount::new(account_id));
    print_response(response);
}

// Get heroku addon by app
fn get_addon_by_app<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    let response = api_client.request(&addons::AddonDetailsByApp::new(app_id, addon_id));
    print_response(response);
}

// Get heroku addon
fn get_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::AddonDetails::new(addon_id));
    print_response(response);
}

// Get heroku addons
fn get_addons<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&addons::AddonList {});
    print_response(response);
}
