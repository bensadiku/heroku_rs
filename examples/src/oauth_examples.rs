extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::oauth;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    get_oauth_list(api_client);
    // get_oauth_info(api_client);
    // create_oauth(api_client);
    // regenerate_oauth(api_client);
    // delete_oauth(api_client); // Careful here :)

    // create_oauth_client(api_client);
    // get_oauth_client(api_client);
    // get_oauth_clients(api_client);
    // patch_oauth_client(api_client);
    // rotate_client_credentials(api_client);
    // delete_client(api_client); // Careful here :)

    // create_oauth_token(api_client);
    // delete_oauth_token(api_client); // Careful here :)
}

// delete client
fn delete_oauth_token<T: HerokuApiClient>(api_client: &T) {
    let token_id = String::from("TOKEN_ID");
    let response = api_client.request(&oauth::OAuthTokenDelete { token_id });
    print_response(response);
}

// create auth token
// these are dummy uuid / data
fn create_oauth_token<T: HerokuApiClient>(api_client: &T) {
    let client = oauth::post::Client {
        secret: String::from("01234567-89ab-cdef-0123-456789abcdef"),
    };
    let grant = oauth::post::Grant {
        code: String::from("01234567-89ab-cdef-0123-456789abcdef"),
        type_field: String::from("authorization_code"),
    };

    let refresh_token = oauth::post::RefreshToken {
        token: String::from("01234567-89ab-cdef-0123-456789abcdef"),
    };

    let response = api_client.request(&oauth::OAuthTokenCreate {
        params: oauth::OAuthTokenCreateParams {
            client,
            grant,
            refresh_token,
        },
    });
    print_response(response);
}

// delete client
fn delete_client<T: HerokuApiClient>(api_client: &T) {
    let client_id = String::from("CLIENT_ID");
    let response = api_client.request(&oauth::OAuthClientDelete { client_id });
    print_response(response);
}

// rotate client credentials
fn rotate_client_credentials<T: HerokuApiClient>(api_client: &T) {
    let client_id = String::from("CLIENT_ID");
    let response = api_client.request(&oauth::OAuthClientRotateCredentials { client_id });
    print_response(response);
}

// update auth client
fn patch_oauth_client<T: HerokuApiClient>(api_client: &T) {
    let name = String::from("Client Name Son!");
    let client_id = String::from("CLIENT_ID");
    let redirect_uri = String::from("https://www.blog.redirecting_site_here.dev");
    let response = api_client.request(&oauth::OAuthClientUpdate {
        client_id,
        params: oauth::OAuthClientUpdateParams { name, redirect_uri },
    });
    print_response(response);
}

// get all clients
fn get_oauth_clients<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&oauth::OAuthClientList {});
    print_response(response);
}

// get client with id
fn get_oauth_client<T: HerokuApiClient>(api_client: &T) {
    let client_id = String::from("CLIENT_ID");
    let response = api_client.request(&oauth::OAuthClientDetails { client_id });
    print_response(response);
}

// create a auth client
fn create_oauth_client<T: HerokuApiClient>(api_client: &T) {
    let name = String::from("Client Name");
    let redirect_uri = String::from("https://www.redirecting_site_here.dev");
    let response = api_client.request(&oauth::OAuthClientCreate {
        params: oauth::OAuthClientCreateParams { name, redirect_uri },
    });
    print_response(response);
}

// delete specific oauth token
fn delete_oauth<T: HerokuApiClient>(api_client: &T) {
    let oauth_id = String::from("OAUTH_ID");
    let response = api_client.request(&oauth::OAuthDelete { oauth_id });
    print_response(response);
}

// regenerate specific oauth token
fn regenerate_oauth<T: HerokuApiClient>(api_client: &T) {
    let oauth_id = String::from("OAUTH_ID");
    let response = api_client.request(&oauth::OAuthRegenerate { oauth_id });
    print_response(response);
}

// create auth token
fn create_oauth<T: HerokuApiClient>(api_client: &T) {
    let auth_scope = vec!["global".to_owned()];
    let response = api_client.request(&oauth::OAuthCreate {
        params: oauth::OAuthCreateParams {
            scope: auth_scope,
            client: None,
            description: None,
            expires_in: None,
        },
    });
    print_response(response);
}

// get specific oauth info
fn get_oauth_info<T: HerokuApiClient>(api_client: &T) {
    let oauth_id = String::from("OAUTH_ID");
    let response = api_client.request(&oauth::OAuthDetails { oauth_id });
    print_response(response);
}

// get oauth list info
fn get_oauth_list<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&oauth::OAuthList {});
    print_response(response);
}
