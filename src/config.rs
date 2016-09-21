//! # Config
//!
//! Just parse a toml file to extract
//! authentification tokens and
//! load into a credential struct for request signing.
//!
extern crate toml;

use std::fs::File;
use std::path::Path;
use std::io::{Read, Error};

const DEFAULT_CONFIG_PATH: &'static str = "Config.toml";

/// OVH API application credentials, including application key, application secret key,
/// consumer key, a temporary access token with access control to user API.
#[derive(Debug,Clone)]
pub struct Credential {
    path: Option<String>,
    toml: Option<toml::Value>,
    pub host: String,
    pub application_key: String,
    pub application_secret: String,
    pub consumer_key: String,
}

/// Utility fonction to read toml file by path
/// Currently only considere api on subsidiary : ovh-eu, ovh-ca.
fn read_from_path<'a, P: AsRef<Path>>(owner: &'a mut String,
                                      path: P)
                                      -> Result<(String, toml::Value), Error> {
    let mut fd = match File::open(path) {
        Err(_) => panic!("Cannot open given path"),
        Ok(fd) => fd,
    };
    match fd.read_to_string(owner) {
        Err(_) => panic!("Cannot read file"),
        Ok(s) => s,
    };
    let mut parser = toml::Parser::new(owner);
    let toml = match parser.parse() {
        None => panic!("Cannot parse toml content"),
        Some(_toml) => _toml,
    };
    let endpoint: toml::Value = toml.get("default").unwrap().lookup("endpoint").unwrap().clone();
    let endp: String = endpoint.as_str().unwrap().to_string();
    let host = host2host(endp.clone());

    Ok((host, toml.get(endp.as_str()).unwrap().clone()))
}

impl Credential {
    /// Initialize a new `Credential` from default path a App Key, App secret, Consumer token.
    pub fn new() -> Credential {
        let toml = &mut String::new();
        let (host, auth): (String, toml::Value) =
            match read_from_path(toml, DEFAULT_CONFIG_PATH.to_owned()) {
                Err(_) => panic!("Could not read auth"),
                Ok(_auth) => _auth,
            };
        let r_app_key = auth.lookup("application_key").unwrap().clone();
        let app_key = String::from(r_app_key.as_str().unwrap().clone());
        let r_app_secret = auth.lookup("application_secret").unwrap().clone();
        let app_secret = String::from(r_app_secret.as_str().unwrap().clone());
        let r_cons_key = auth.lookup("consumer_key").unwrap().clone();
        let cons_key = String::from(r_cons_key.as_str().unwrap().clone());

        Credential {
            toml: Some(auth),
            path: Some(DEFAULT_CONFIG_PATH.to_owned()),
            host: host,
            application_key: app_key,
            application_secret: app_secret,
            consumer_key: cons_key,
        }
    }

    /// Initialize a new `Credential` from given path a App Key, App secret, Consumer token.
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Credential {

        let toml = &mut String::new();
        let (host, auth): (String, toml::Value) = match read_from_path(toml, path) {
            Err(_) => panic!("Could not read auth"),
            Ok(_auth) => _auth,
        };
        let r_app_key = auth.lookup("application_key").unwrap().clone();
        let app_key = String::from(r_app_key.as_str().unwrap().clone());
        let r_app_secret = auth.lookup("application_secret").unwrap().clone();
        let app_secrets = String::from(r_app_secret.as_str().unwrap().clone());
        let r_cons_key = auth.lookup("consumer_key").unwrap().clone();
        let cons_key = String::from(r_cons_key.as_str().unwrap().clone());

        Credential {
            toml: Some(auth),
            path: Some("".to_string()),
            host: host,
            application_key: app_key,
            application_secret: app_secrets,
            consumer_key: cons_key,
        }
    }

    /// Initialize a new `Credential` from given an App Key and App secret.
    pub fn new_with_application(application_key: &str, application_secret: &str) -> Credential {
        Credential {
            toml: None,
            path: None,
            host: String::from("eu.api.ovh.com"),
            application_key: String::from(application_key),
            application_secret: String::from(application_secret),
            consumer_key: String::from(""),
        }
    }

    /// Initialize a new `Credential` from given an App Key, App Secret, and Consumer Key.
    pub fn new_with_credential(application_key: &str,
                               application_secret: &str,
                               consumer_key: &str)
                               -> Credential {
        Credential {
            toml: None,
            path: None,
            host: String::from("eu.api.ovh.com"),
            application_key: String::from(application_key),
            application_secret: String::from(application_secret),
            consumer_key: String::from(consumer_key),
        }
    }
}

fn host2host(host: String) -> String {
    match host.as_ref() {
        "ovh-eu" => "eu.api.ovh.com".to_string(),
        "ovh-ca" => "ca.api.ovh.com".to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::Credential;

    #[test]
    fn test_application_key() {
        let cred = Credential::new_from_file("Config.toml.dist");
        let res = cred.application_key;
        assert_eq!("ak", res);
    }

    #[test]
    fn test_application_secret() {
        let cred = Credential::new_from_file("Config.toml.dist");
        let res = cred.application_secret;
        assert_eq!("as", res);
    }

    #[test]
    fn test_consumer_key() {
        let cred = Credential::new_from_file("Config.toml.dist");
        let res = cred.consumer_key;
        assert_eq!("ck", res);
    }

    #[test]
    fn test_host() {
        let cred = Credential::new_from_file("Config.toml.dist");
        let res = cred.host;
        assert_eq!("eu.api.ovh.com", res);
    }

}
