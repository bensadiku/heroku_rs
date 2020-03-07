use heroku_rs as hk;
extern crate serde_json;
use hk::client::Executor;
use hk::defaults::{ BuildPackUpdate, BuildpackInstallation};
use hk::StatusCode;
use serde_json::Value;
mod utils;
use serde::{Deserialize, Serialize};
use utils::*;

//POST /apps/{app_id_or_name}/dynos
#[test]
fn post_app_dyno() {

    #[derive(Serialize, Deserialize)]
    struct DynoCmd {
        command: String,
    };

    let cmd = DynoCmd {
        command: String::from("bash"),
    };
    let hc = setup_heroku_connection();

    let (headers, status, json) = hc
        .post(cmd)
        .apps()
        .app_name(TEST_APP_NAME)
        .app_dyno()
        .execute::<Value>()
        .expect(utils::FAILED_HEROKU_CONNECTION);

    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    assert_eq!(status, StatusCode::CREATED);
}

//GET /apps
#[test]
fn get_apps_list() {
    let hc = setup_heroku_connection();
    let (headers, status, json) = hc
        .get()
        .apps()
        .execute::<Value>()
        .expect(utils::FAILED_HEROKU_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    assert_eq!(status, StatusCode::OK);
}

//PATCH /apps/{app_id_or_name}
#[test]
fn patch_app() {
    #[derive(Serialize, Deserialize)]
    struct PatchApp {
        maintenance: bool,
    };

    let cmd = PatchApp { maintenance: true };
    let hc = setup_heroku_connection();
    let (headers, status, json) = hc
        .patch(cmd)
        .apps()
        .app_name(TEST_APP_NAME)
        .execute::<Value>()
        .expect(utils::FAILED_HEROKU_CONNECTION);

    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    assert_eq!(status, StatusCode::OK);
}

//PUT /apps/{app_id_or_name}/buildpack-installations
#[test]
fn put_buildpack_installations() {
    let bp_update = BuildPackUpdate {
        buildpack: String::from("https://github.com/heroku/heroku-buildpack-python"),
    };

    let bp_install = BuildpackInstallation {
        updates: vec![bp_update],
    };

    let hc = setup_heroku_connection();
    let (headers, status, json) = hc
        .put(bp_install)
        .apps()
        .app_name(TEST_APP_NAME)
        .app_buildpack_installation()
        .execute::<Value>()
        .expect(utils::FAILED_HEROKU_CONNECTION);

    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    assert_eq!(status, StatusCode::OK);
}

//DELETE /apps/{app_id_or_name}/dynos
#[test]
fn delete_dynos() {
    let hc = setup_heroku_connection();

    let (headers, status, json) = hc
        .delete_empty()
        .apps()
        .app_name(TEST_APP_NAME)
        .app_dynos()
        .execute::<Value>()
        .expect(utils::FAILED_HEROKU_CONNECTION);

    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    assert_eq!(status, StatusCode::ACCEPTED);
}
