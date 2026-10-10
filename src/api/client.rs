// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use ureq::{Agent, tls::{TlsConfig, TlsProvider}};
use std::time::Duration;
use std::thread;
use std::sync::OnceLock;
use tracing::{trace, debug, error};


#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub err: i32,
    pub msg: String,
    pub data: T,
}

pub struct WslUiClient {
    api1_url: String,
    agent: Agent,
}

// Metadata sent with every API request (non-sensitive: version / system language / device ID).
// Populated once by `set_request_meta()` early during app startup, then read-only.
#[derive(Debug, Clone, Default)]
pub struct RequestMeta {
    pub app_version: String,
    pub system_language: String,
    pub device_id: String,
}

// Global request metadata cache. OnceLock takes effect permanently on the first successful set;
// when uninitialized (after the first upgrade from an older version, or in test environments) the headers are skipped automatically and requests are not blocked.
static REQUEST_META: OnceLock<RequestMeta> = OnceLock::new();

// Populate the global request metadata, injected into request headers uniformly by `request_url()`.
// Called only once early during startup; repeated calls keep the first value.
pub fn set_request_meta(meta: RequestMeta) {
    let _ = REQUEST_META.set(meta);
}

impl WslUiClient {
    pub fn new() -> Self {
        let mut config = Agent::config_builder();
        if cfg!(debug_assertions) {
            if let Ok(_tls_connector) = native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .build()
            {
                config = config.tls_config(
                    TlsConfig::builder()
                        .provider(TlsProvider::NativeTls)
                        .disable_verification(true)
                        .build()
                );
                trace!("Developer mode: HTTPS certificate verification is disabled (-k)");
            }
        }
        let agent = Agent::new_with_config(config.build());

        Self {
            api1_url: crate::app::API1_URL.to_string(),
            agent,
        }
    }

    #[allow(dead_code)]
    pub fn request_api1<T>(&self, method: &str, uri: &str, body: Option<serde_json::Value>) -> Result<(ApiResponse<T>, Option<String>), String>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        self.request_api1_with_timeout(method, uri, body, None)
    }

    pub fn request_api1_with_timeout<T>(&self, method: &str, uri: &str, body: Option<serde_json::Value>, timeout_ms: Option<u64>) -> Result<(ApiResponse<T>, Option<String>), String>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let url = format!("{}{}", self.api1_url, uri);
        self.request_url(method, &url, body, timeout_ms)
    }

    pub fn request_full_url<T>(&self, method: &str, url: &str, body: Option<serde_json::Value>, timeout_ms: Option<u64>) -> Result<(ApiResponse<T>, Option<String>), String>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        self.request_url(method, url, body, timeout_ms)
    }

    fn request_url<T>(&self, method: &str, url: &str, body: Option<serde_json::Value>, timeout_ms: Option<u64>) -> Result<(ApiResponse<T>, Option<String>), String>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(5000));
        let do_request = || -> Result<(ApiResponse<T>, Option<String>), String> {
            trace!("WSLUI API Request: method={}, url={}, body={:?}", method, url, body);

            let is_post = method.to_uppercase().as_str() == "POST";

            let req = if is_post {
                self.agent.post(url)
                    .config()
                    .timeout_global(Some(timeout))
                    .build()
            } else {
                self.agent.get(url)
                    .config()
                    .timeout_global(Some(timeout))
                    .build()
                    .force_send_body()
            };

            // Inject request headers uniformly: User-Agent + global request metadata (version/system language/device ID)
            let mut req = req.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");
            if let Some(meta) = REQUEST_META.get() {
                if !meta.app_version.is_empty() {
                    req = req.header("App-Version", meta.app_version.as_str());
                }
                if !meta.system_language.is_empty() {
                    req = req.header("System-Language", meta.system_language.as_str());
                }
                if !meta.device_id.is_empty() {
                    req = req.header("Device-Id", meta.device_id.as_str());
                }
            }

            let resp = if let Some(ref b) = body {
                req.send_json(b)
            } else {
                req.send_empty()
            };

            let mut resp = resp.map_err(|e: ureq::Error| {
                let error_msg = e.to_string();
                if Self::is_timeout_error(&error_msg) {
                    error!("WSLUI API Timeout Error: {}", error_msg);
                    "RequestTimeOut".to_string()
                } else {
                    error!("WSLUI API Network Error: {}", error_msg);
                    error_msg
                }
            })?;

            let date_header = resp.headers().get("Date")
                .and_then(|v: &ureq::http::HeaderValue| v.to_str().ok())
                .map(|s: &str| s.to_string());
            let status = resp.status().as_u16();

            let resp_text = resp.body_mut().read_to_string().map_err(|e: ureq::Error| {
                error!("WSLUI API Read Body Error: {}", e);
                e.to_string()
            })?;

            let url_path = url.split_once("://")
                .and_then(|(_, rest)| rest.split_once('/'))
                .map(|(_, path)| format!("/{}", path.split('?').next().unwrap_or(path)))
                .unwrap_or_else(|| "/".to_string());

            let compact_body = serde_json::from_str::<serde_json::Value>(&resp_text)
                .and_then(|v| serde_json::to_string(&v))
                .unwrap_or(resp_text.clone());
            debug!("WSLUI API Response: method={}, path={}, status={}, body={}", method, url_path, status, compact_body);

            let clean_text = resp_text.trim_start_matches('\u{FEFF}').trim();

            let api_resp: ApiResponse<T> = serde_json::from_str(clean_text).map_err(|e| {
                error!("WSLUI API JSON Parse Error: {}", e);
                e.to_string()
            })?;

            if api_resp.err != 0 {
                return Err(format!("API business error: err={}, msg={}", api_resp.err, api_resp.msg));
            }

            Ok((api_resp, date_header))
        };

        match do_request() {
            Ok(res) => Ok(res),
            Err(e) => {
                if e == "RequestTimeOut" {
                    debug!("WSLUI API Request timed out. Retrying after 200ms...");
                } else {
                    debug!("WSLUI API Request failed: {}. Retrying after 200ms...", e);
                }
                thread::sleep(Duration::from_millis(200));
                do_request().map_err(|e2| {
                    if e2 == "RequestTimeOut" {
                        error!("WSLUI API Retry timed out");
                    } else {
                        error!("WSLUI API Retry failed: {}", e2);
                    }
                    e2
                })
            }
        }
    }

    fn is_timeout_error(error_msg: &str) -> bool {
        let lower_msg = error_msg.to_lowercase();
        lower_msg.contains("timed out") 
            || lower_msg.contains("timeout")
            || lower_msg.contains("connection timed out")
            || lower_msg.contains("request timed out")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex};

    // Start a local HTTP echo server that captures received request headers and returns a wslui-style JSON response.
    fn start_echo_server() -> (String, Arc<Mutex<Vec<(String, String)>>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let captured: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let captured_clone = captured.clone();

        std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 8192];
                let n = stream.read(&mut buf).unwrap_or(0);
                let req = String::from_utf8_lossy(&buf[..n]).to_string();

                // Parse request headers (skip the request line, until the blank line)
                let mut store = captured_clone.lock().unwrap();
                for line in req.lines().skip(1).take_while(|l| !l.is_empty()) {
                    if let Some((k, v)) = line.split_once(':') {
                        store.push((k.trim().to_string(), v.trim().to_string()));
                    }
                }
                drop(store);

                let body = r#"{"err":0,"msg":"success","data":{}}"#;
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes());
                let _ = stream.flush();
            }
        });

        (format!("http://{}", addr), captured)
    }

    // Verify that the request actually sent by `request_url()` contains the three metadata headers.
    // This is an automated check of "whether the headers were sent successfully": capture the real HTTP request headers with a local echo server and assert on them.
    #[test]
    fn test_request_url_injects_meta_headers() {
        let (base_url, captured) = start_echo_server();

        set_request_meta(RequestMeta {
            app_version: "0.12.0".to_string(),
            system_language: "zh-CN".to_string(),
            device_id: "a3f5c2d9e8b74f16a2c9d5e7f1b3a6c4".to_string(),
        });

        let client = WslUiClient::new();
        let result: Result<
            (ApiResponse<crate::api::models::UtilsPingData>, Option<String>),
            String,
        > = client.request_full_url("GET", &base_url, None, Some(5000));
        assert!(result.is_ok(), "request should succeed, got: {:?}", result.err());

        let headers = captured.lock().unwrap();
        let has = |name: &str, val: &str| {
            headers
                .iter()
                .any(|(k, v)| k.eq_ignore_ascii_case(name) && v == val)
        };
        assert!(has("App-Version", "0.12.0"), "App-Version missing: {:?}", *headers);
        assert!(has("System-Language", "zh-CN"), "System-Language missing: {:?}", *headers);
        assert!(has("Device-Id", "a3f5c2d9e8b74f16a2c9d5e7f1b3a6c4"), "Device-Id missing: {:?}", *headers);
    }
}