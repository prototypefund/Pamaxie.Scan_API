extern crate core;
use std::env;
use std::process::exit;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use std::string::String;
use crate::helper::web_helper;
use crate::services::file_recognition_service;

mod services
{
    pub mod file_recognition_service;
}

mod helper
{
    pub mod web_helper;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    validate_client_configuration();

    HttpServer::new(|| {
        App::new().app_data(web::PayloadConfig::new(1000000 * 250))
                .service(services::file_recognition_service::check_api)
                .service(services::file_recognition_service::echo)
    }).bind(("127.0.0.1", 8080))?.run().await
}

fn validate_client_configuration() {
    let mut error_data = "".to_string();
    let mut has_error = false;

    if get_s3_access_key().is_empty() {
        has_error = true;
        error_data = format!("{}The S3 Access Key has not been set. We require this key to be set to continue running. \
        Please refer to our documentation to see how to set this environment variable.\n", error_data)
    }

    if get_s3_secret_key().is_empty() {
        has_error = true;
        error_data = format!("{}The S3 Secret Key has not been set. We require this key to be set to continue running. \
        Please refer to our documentation to see how to set this environment variable.\r\n", error_data);
    }

    if get_s3_url().is_empty() {
        has_error = true;
        error_data = format!("{}The S3 Url has not been set. We require the URL to be set to continue running. \
        Please refer to our documentation to see how to set this environment variable.\r\n", error_data);
    }

    if file_recognition_service::get_pam_url().is_empty() {
        has_error = true;
        error_data = format!("{}The API base URL is empty. It is required to be set, to interact, test and authorize with our database. \
        Please refer to our documentation to see how to set this environment variable.\r\n", error_data);
    }

    if has_error {
        println!("{}", error_data);
        exit(-501);
    }
}

///Gets the S3 storage Access key
fn get_s3_access_key() -> String {
    return web_helper::get_env_variable("S3AccessKey".to_string(), "".to_string());
}

///Gets the S3 storage Secret Key
fn get_s3_secret_key() -> String {
    return web_helper::get_env_variable("S3SecretKey".to_string(), "".to_string());
}

///Gets the S3 storage URL
fn get_s3_url() -> String {
    return web_helper::get_env_variable("S3Url".to_string(), "https://pam-dev.sfo3.digitaloceanspaces.com".to_string());
}
