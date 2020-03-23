//Anything related to getting apps and it's properties goes here.
use super::{App, AppFeature, AppWebhook, AppWebhookDelivery};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Info
/// 
/// Get info for existing app.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-info)
pub struct AppDetails {
    /// app_id can be the app id or app name.
    pub app_id: String,
}

impl HerokuEndpoint<App> for AppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_id)
    }
}

/// App List
/// 
/// List existing apps.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-list)
pub struct AppList {}

impl HerokuEndpoint<Vec<App>> for AppList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps")
    }
}

/// App List Owned and Collaborated
/// 
/// List owned and collaborated apps (excludes team apps).
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-list-owned-and-collaborated)
pub struct AccountAppList {
    /// account_id can be the account email, id or self.
    pub account_id: String,
}

impl HerokuEndpoint<Vec<App>> for AccountAppList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}/apps", self.account_id)
    }
}

/// App Feature Info
/// 
/// Info for an existing app feature.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-info)
pub struct AppFeatureDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// feature_id can be the feature name or id.
    pub feature_id: String,
}

impl HerokuEndpoint<AppFeature> for AppFeatureDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/features/{}", self.app_id, self.feature_id)
    }
}

/// App Feature List
/// 
/// List existing app features.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-list)
pub struct AppFeatureList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<AppFeature>> for AppFeatureList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/features", self.app_id)
    }
}

/// App Webhook List
/// 
/// List all webhook subscriptions for a particular app.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-list)
pub struct AppWebhookList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<AppWebhook>> for AppWebhookList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks", self.app_id)
    }
}

/// App Webhook Info
/// 
/// Returns the info for an app webhook subscription.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-info)
pub struct AppWebhookDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// webhook_id is the webhook id.
    pub webhook_id: String,
}

impl HerokuEndpoint<AppWebhook> for AppWebhookDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks/{}", self.app_id, self.webhook_id)
    }
}

/// App Webhook Delivery
/// 
/// Returns the info for an existing delivery.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delivery-info)
pub struct AppWebhookDeliveryDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// webhook_delivery_id is the webhook delivery id.
    pub webhook_delivery_id: String,
}

impl HerokuEndpoint<AppWebhookDelivery> for AppWebhookDeliveryDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/webhook-deliveries/{}",
            self.app_id, self.webhook_delivery_id
        )
    }
}

/// App Webhook Deliveries
/// 
/// Lists existing deliveries for an app.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delivery-list)
pub struct AppWebhookDeliveryList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<AppWebhookDelivery>> for AppWebhookDeliveryList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhook-deliveries", self.app_id,)
    }
}
