# OVH-rs

A lightweight wrapper around OVH's APIs.

Handles credential management and requests signing.

Prerequisites
------------

Considere using stable Rust
Tested only on Linux, Windows coming.

Quickstart
----------

Compile and fetch dependencies

```bash
cargo build
```

## Fill your credentials

``` bash
$cp Config.toml.dist Config.toml
```

Edit your Config.toml file.

``` ini
[default]
endpoint = "ovh-eu"

[ovh-eu]
application_key = "<APPLICATION KEY>"
application_secret = "<APPLICATION SECRET>"
```

Create your credential via : https://eu.api.ovh.com/createApp/
or via a simple curl but take care of accessRules given,

```bash
$curl -XPOST -H"X-Ovh-Application: <APPLICATION KEY>" \
-H "Content-type: application/json" \
https://eu.api.ovh.com/1.0/auth/credential  -d '{
    "accessRules": [
        {"method": "GET","path": "/*"},
        {"method": "POST","path": "/*"},
        {"method": "PUT","path": "/*"},
        {"method": "DELETE","path": "/*"}
    ],
    "redirection":"https://api.ovh.com/"
}'
```

With great power comes with great responsibility ;)

Edit your Config.toml file.

``` ini
consumer_key = "<CONSUMER KEY>"
```

How to run tests?
-----------------

```bash
RUST_LOG=rust-ovh=info cargo test
```

How to build doc?
-----------------

```bash
cargo doc --no-deps
```
and explore ./target/doc/ovh/index.html
with your favorite browser

Tested on APIs
--------------

## OVH Europe

 * Documentation: https://eu.api.ovh.com/
 * Community support: api-subscribe@ml.ovh.net
 * Console: https://eu.api.ovh.com/console
 * Create application credentials: https://eu.api.ovh.com/createApp/
 * Create script credentials (all keys at once): https://eu.api.ovh.com/createToken/

License
-------
MIT


Thanks to Rust community
