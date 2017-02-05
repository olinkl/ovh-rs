//! #OVH-rs
//!
//! OVH-rs, is a lightweight wrapper for OVH's APIs.
//! That's an easy way to connect to api.ovh.com, to manage
//! and configure your products from Rust applications.
//!
//! It handles for you credential management
//! and requests signing.
//!
extern crate chrono;
extern crate crypto;

#[macro_use] extern crate hyper;
#[macro_use] extern crate reqwest;

#[macro_use]
extern crate log;

pub use config::Credential;
pub use client::OVHClient;

pub mod client;
pub mod config;
