use std::collections::HashMap;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.132 Safari/537.36 C845D9D38B3A68F4F74057DB542AD252 tx/2.0";

const DEFAULT_TIMEOUT: u64 = 15;

const DEFAULT_RESPONSE_LENGTH: usize = 1024;

#[derive(Debug, Clone)]
pub struct ReqOptions {
    pub timeout: Option<u64>,
    pub retry: Option<usize>,
    pub verify_ssl: Option<bool>,
    pub allow_redirect: Option<bool>,
    pub proxy: Option<String>,
}

impl Default for ReqOptions {
    fn default() -> Self {
        Self{
            timeout: Some(DEFAULT_TIMEOUT),
            retry: Some(0),
            verify_ssl: Some(false),
            allow_redirect: Some(false),
            proxy: None,
        }
    }
}

pub struct HttpClient {
    client: reqwest::Client,
    options: ReqOptions,
}

impl HttpClient {
    pub fn new(options: ReqOptions) -> Self {
        let mut builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(options.timeout.unwrap_or(DEFAULT_TIMEOUT)));
        if !options.verify_ssl.unwrap_or(false) {
           builder = builder.danger_accept_invalid_certs(true);
        }

        if let Some(proxy) = &options.proxy {
            if let Ok(proxy_url) = reqwest::Proxy::http(proxy) {
                builder = builder.proxy(proxy_url);
            }
        }

        if !options.allow_redirect.unwrap_or(true) {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        let client = builder.build().expect("Failed to build HTTP client");
        Self {client, options}
    }

    pub async fn get(&self, url: &str, headers: HashMap<String, String>) -> Result<Response, reqwest::Error> {
        self.request("GET", url, headers, None)
    }

    pub async fn post(&self, url: &str, headers: HashMap<String, String>, body: Option<&[u8]>) -> Result<Response, reqwest::Error> {
        self.request("POST", url, headers, body)
    }

    pub async fn request(&self,
                         method: &str,
                         url: &str,
                         headers: HashMap<String, String>,
                         body: Option<&[u8]>,
    ) -> Result<Response, reqwest::Error> {
        let mut req = self.client.request(method.parse().unwrap(), url);
        let mut header = HeaderMap::new();
        for (k, v) in headers {
            header.insert(k.parse().unwrap(), HeaderValue::from_str(&v).unwrap());
        }
        header.insert("User-Agent", HeaderValue::from_static(DEFAULT_USER_AGENT));
        req = req.headers(header);

        if let Some(b) = body {
            req = req.body(b.to_vec());
        }

        let mut attemps = 0;

        let mut retries = self.options.retry.unwrap_or(0);
        loop {
            let result = req.try_clone().unwrap().send().await;
            if result.is_ok() || attemps > retries {
                return result;
            }
            attemps += 1;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
