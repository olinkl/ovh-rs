//! # Client
//!
//! Main module to handle HTTP request signing
//! and Rest calls
//!

extern crate serde;
extern crate serde_json;

use config::Credential;
use std::io::Read;

use hyper::header::{Headers, UserAgent, Accept, qitem, ContentType};
use hyper::mime::{Value, Mime, TopLevel, SubLevel, Attr};
use hyper::client::Response;
use hyper::client::Client;
use hyper;

use chrono::*;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

// Required headers for auth
header! { (XOvhApplication, "X-Ovh-Application") => [String] }
header! { (XOvhTimestamp, "X-Ovh-Timestamp") => [String] }
header! { (XOvhSignature, "X-Ovh-Signature") => [String] }
header! { (XOvhConsumer, "X-Ovh-Consumer") => [String] }

#[derive(Debug,Clone)]
pub struct OVHClient {
    pub credential: Credential,
}

impl OVHClient {
    /// Initialize a new `Credential` from default path a App Key, App secret, Consumer token.
    pub fn new() -> OVHClient {
        OVHClient { credential: Credential::new() }
    }

    /// Compute signature for OVH.
    fn build_sig(method: &str,
                 query: &str,
                 body: &str,
                 timestamp: &str,
                 aas: &str,
                 ck: &str)
                 -> String {
        let sep = "+";
        let prefix = "$1$".to_string();

        let capacity = 1 + &aas.len() + &sep.len() + &ck.len() + &method.len() + &sep.len() +
                       &query.len() + &sep.len() +
                       &body.len() + &sep.len() + &timestamp.len();
        let mut signature = String::with_capacity(capacity);
        signature.push_str(&aas);
        signature.push_str(&sep);
        signature.push_str(&ck);
        signature.push_str(&sep);
        signature.push_str(&method);
        signature.push_str(&sep);
        signature.push_str(&query);
        signature.push_str(&sep);
        signature.push_str(&body);
        signature.push_str(&sep);
        signature.push_str(&timestamp);

        debug!("Signature: {}", &signature);
        let mut hasher = Sha1::new();
        hasher.input_str(&signature);
        let hex = hasher.result_str();
        debug!("hex: {}", &hex);

        let sign = prefix + &hex;
        sign
    }

    /// Ask time to OVH API server to compute delta time
    fn remote_time() -> u64 {
        let query = "https://eu.api.ovh.com/1.0/auth/time".to_string();
        // Create a client.
        let client = Client::new();

        // Creating an outgoing request.
        let mut res = client.get(&query)
            .send()
            .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body.parse::<u64>().unwrap()
    }

    /// compute delta time
    fn compute_time_delta() -> u64 {
        let localtime = Local::now()
            .format("%s")
            .to_string()
            .parse::<u64>()
            .unwrap();
        let remotetime = OVHClient::remote_time();
        let deltatime = remotetime - localtime;
        info!("Delta time: {:?}", deltatime);
        deltatime
    }

    /// Start a client request with given method
    /// Use Hyper client, maybe curl-rs in later version
    pub fn request(credential: &Credential, method: &str, query: &str, body: &str) -> String {
        let localtime = Local::now().format("%s").to_string().parse::<u64>().unwrap();
        let computed_time = localtime + OVHClient::compute_time_delta();
        let timestamp = computed_time.to_string();

        let protocol = "https://".to_string();
        let base_path = "/1.0";
        let url = protocol + &credential.host + &base_path + &query;
        let sign = OVHClient::build_sig(&method,
                                        &url,
                                        &body,
                                        &timestamp,
                                        credential.application_secret.as_str(),
                                        credential.consumer_key.as_str());

        // build headers
        let mut headers = Headers::new();
        headers.set(XOvhApplication(credential.application_key.to_string()));
        headers.set(XOvhTimestamp(timestamp.to_string()));
        headers.set(XOvhSignature(sign.to_string()));
        headers.set(XOvhConsumer(credential.consumer_key.to_string()));
        headers.set(Accept(vec![
                qitem(Mime(
                        TopLevel::Application,
                        SubLevel::Json,
                        vec![(Attr::Charset,
                              Value::Utf8)])
                ),
            ]));
        headers.set(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));
        headers.set(UserAgent("hyper/0.9.10/ovh-rs".to_owned()));

        // Create a client.
        let client = Client::new();

        debug!("Signature: {}", sign.to_string());

        // Creating an outgoing request.
        let res: Result<Response, hyper::Error> = match method {
            "HEAD" => {
                client.head(&url)
                    .headers(headers)
                    .send()
            }
            "GET" => {
                client.get(&url)
                    .headers(headers)
                    .send()
            }
            "POST" => {
                client.post(&url)
                    .headers(headers)
                    .body(body)
                    .send()
            }
            "PUT" => {
                client.put(&url)
                    .headers(headers)
                    .body(body)
                    .send()
            }
            "PATCH" => {
                client.patch(&url)
                    .headers(headers)
                    .body(body)
                    .send()
            }
            "DELETE" => {
                client.delete(&url)
                    .headers(headers)
                    .body(body)
                    .send()
            }
            _ => Err(hyper::Error::Method),
        };
        let mut body = String::new();
        res.unwrap().read_to_string(&mut body).unwrap();
        body
    }
}

#[cfg(test)]
mod tests {
    use super::OVHClient;
    extern crate serde;
    extern crate serde_json;

    #[test]
    fn test_build_sig() {
        let _ovh = OVHClient::new();
        let method = "GET";
        let query = "https://eu.api.ovh.com/1.0/ipLoadbalancing";
        let body = "";
        let timestamp = "1466716163";
        let aas = "somesecret";
        let ck = "fakeconsumerkey";
        let signature = OVHClient::build_sig(&method, &query, &body, &timestamp, &aas, &ck);
        assert_eq!(&signature, "$1$7ff04a6c8610e4f96a1c0a04dff50ed760a6b724");
    }

    #[test]
    fn test_remote_time() {
        let remote_time = OVHClient::remote_time();
        assert_eq!(true, remote_time > 0);
    }

    #[test]
    fn test_get() {
        let ovh = OVHClient::new();
        let cred = ovh.credential;

        let response = OVHClient::request(&cred,
                                          "GET",
                                          "/ipLoadbalancing",
                                          "");
        // should assert after json parse
        let deser_value: self::serde_json::Value = serde_json::from_str(&response).unwrap();
        assert_eq!(true, deser_value.is_array());
    }

    #[test]
    fn test_post() {
        let ovh = OVHClient::new();
        let cred = ovh.credential;

        let mut body = "{\"ovhSubsidiary\": \"FR\"}";
        let mut response = OVHClient::request(&cred,
                                          "POST",
                                          "/order/cart",
                                          &body);
        // should assert after json parse
        let deser_value: self::serde_json::Value = serde_json::from_str(&response).unwrap();
        assert_eq!(true, deser_value.is_object());
        let obj = deser_value.as_object().unwrap();
        assert_eq!(true, obj.get("cartId").unwrap().is_string());
        assert_eq!(true, obj.get("expire").unwrap().is_string());
        assert_eq!(true, obj.get("description").unwrap().is_string());
        assert_eq!("Default cart", obj.get("description").unwrap().as_str().unwrap());
        assert_eq!(true, obj.get("readOnly").unwrap().is_boolean());
        assert_eq!(true, obj.get("items").unwrap().is_array());

        //test_get_with_query
        let cart_id = obj.get("cartId").unwrap().as_str().unwrap();
        let mut url = "/order/cart/".to_string() + cart_id + "/domain?domain=rustyrust.fr";

        response = OVHClient::request(&cred,
                                          "GET",
                                          &url,
                                          "");
        let deser_value: self::serde_json::Value = serde_json::from_str(&response).unwrap();
        // should assert after json parse
        assert_eq!(true, deser_value.is_array());

        //test_put
        url = "/order/cart/".to_string() + cart_id;
        body = "{\"description\": \"a new rust cart description\"}";
        response = OVHClient::request(&cred,
                                          "PUT",
                                          &url,
                                          &body);
        // should assert after json parse
        let deser_value: self::serde_json::Value = serde_json::from_str(&response).unwrap();
        assert_eq!(true, deser_value.is_object());
        let obj = deser_value.as_object().unwrap();
        assert_eq!(true, obj.get("cartId").unwrap().is_string());
        assert_eq!(true, obj.get("expire").unwrap().is_string());
        assert_eq!(true, obj.get("description").unwrap().is_string());
        assert_eq!("a new rust cart description", obj.get("description").unwrap().as_str().unwrap());
        assert_eq!(true, obj.get("readOnly").unwrap().is_boolean());
        assert_eq!(true, obj.get("items").unwrap().is_array());

        //test assign
        url = "/order/cart/".to_string() + cart_id + "/assign";
        body = "";
        response = OVHClient::request(&cred,
                                          "POST",
                                          &url,
                                          &body);
        assert_eq!("null", response);

        //test_delete
        url = "/order/cart/".to_string() + cart_id;
        body = "";
        response = OVHClient::request(&cred,
                                          "DELETE",
                                          &url,
                                          &body);
        assert_eq!("null", response);
    }

}
