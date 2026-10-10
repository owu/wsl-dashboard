// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    #[serde(rename = "ip-address", alias = "ip_address")]
    pub ip_address: String,
    #[serde(rename = "is-loopback", alias = "is_loopback", default)]
    pub is_loopback: bool,
    #[serde(rename = "is-virtual", alias = "is_virtual", default)]
    pub is_virtual: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortProxyRule {
    pub id: String,
    #[serde(rename = "distro-name", alias = "distro_name")]
    pub distro_name: String,
    #[serde(rename = "listen-address", alias = "listen_address")]
    pub listen_address: String,
    #[serde(rename = "listen-port", alias = "listen_port")]
    pub listen_port: u16,
    #[serde(rename = "target-port", alias = "target_port")]
    pub target_port: u16,
    #[serde(rename = "enable-firewall", alias = "enable_firewall", default)]
    pub enable_firewall: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpProxyConfig {
    #[serde(rename = "is-enabled", alias = "is_enabled", default)]
    pub is_enabled: bool,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: String,
    #[serde(rename = "auth-enabled", alias = "auth_enabled", default)]
    pub auth_enabled: bool,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "no-proxy", alias = "no_proxy", default)]
    pub no_proxy: String,
}

pub fn default_no_proxy() -> String {
    "localhost,127.0.0.1,.example.com".to_string()
}

pub fn default_host() -> String {
    "e.g. 192.168.1.10".to_string()
}

pub fn default_port() -> String {
    "e.g. 10808".to_string()
}
