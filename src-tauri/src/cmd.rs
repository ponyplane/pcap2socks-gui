use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TestPayload {
    pub protocol: usize,
    pub destination: String,
    pub authentication: bool,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RunPayload {
    pub interface: String,
    pub mtu: usize,
    pub preset: usize,
    pub source: String,
    pub publish: String,
    pub protocol: usize,
    pub destination: String,
    pub authentication: bool,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Interface {
    name: String,
    alias: Option<String>,
    mtu: usize,
}

impl Interface {
    pub fn new(name: String, alias: Option<String>, mtu: usize) -> Interface {
        Interface { name, alias, mtu }
    }
}

#[derive(Debug, Serialize)]
pub struct TestResponse {
    pub nat: String,
    pub ip: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResponse {
    pub nat: String,
    pub remote_ip: Option<String>,
    pub src_ip: String,
    pub mask: String,
    pub gateway: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatusResponse {
    pub run: bool,
    pub latency: isize,
    pub upload_size: usize,
    pub upload_count: usize,
    pub download_size: usize,
    pub download_count: usize,
}
