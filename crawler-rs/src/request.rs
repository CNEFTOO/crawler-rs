use std::collections::HashMap;
use std::hash::Hash;
use url::{Url as URL, Url, form_urlencoded};

pub struct Filter {
    pub marker_query_map: HashMap<String, serde_json::Value>,
    pub query_keys_id: String,
    pub query_map_id: String,
    pub marked_post_data_map: HashMap<String, serde_json::Value>,
    pub post_data_id: String,
    pub marked_path: String,
    pub fragment_id: String,
    pub path_id: String,
    pub unique_id: String,
}

pub struct Options {
    pub headers: HashMap<String, serde_json::Value>,
    pub post_data: String,
}

pub struct Request {
    pub url: URL,
    pub method: String,
    pub headers: HashMap<String, serde_json::Value>,
    pub post_data: String,
    pub filters: Filter,
    pub source: String,
    pub redirection_flag: bool,
    pub proxy: String,
}

pub static SUPPORT_CONTENT_TYPE: [&str; 2] = ["application/json", "application/x-www-form-urlencoded"];

impl Request {
    pub fn get_request(method: &str, url: Url, options: Option<Options>) -> Request {
        let mut request = Request{
            url,
            method: method.to_uppercase(),
            headers: HashMap::new(),
            post_data: String::new(),
            filters: Filter{
                marker_query_map: HashMap::new(),
                query_keys_id: String::new(),
                query_map_id: String::new(),
                marked_post_data_map: HashMap::new(),
                post_data_id: String::new(),
                marked_path: String::new(),
                fragment_id: String::new(),
                path_id: String::new(),
                unique_id: String::new(),
            },
            source: String::new(),
            redirection_flag: false,
            proxy: String::new(),
        };

        if let Some(options) = options {
            // if let Some(headers) = options.headers {
            //     request.headers = headers;
            // }
            request.headers = options.headers;

            if !options.post_data.is_empty() {
                request.post_data = options.post_data;
            }
        }

        request
    }

    pub fn format_print(&self) {
        let mut temp_str = self.method.clone();
        temp_str.push_str(&format!("{} HTTP/1.1\r\n", self.url));

        for (key, value) in &self.headers {
            temp_str.push_str(&format!("{}:{}\r\n", key, value));
        }
        temp_str.push_str("\r\n");
        if self.method == "POST" {
            temp_str.push_str(&self.post_data);
        }
        println!("{}", temp_str);
    }

    pub fn simple_format(&self) -> String {
        let mut temp_str = self.method.clone();
        temp_str.push_str(&format!("{} HTTP/1.1\r\n", self.url));
        if self.method == "POST" {
            temp_str.push_str(&self.post_data);
        }
        temp_str
    }

    pub fn no_headers_id(&self) -> String {
        format!("{:x}", md5::compute(self.method.clone() + &self.url.to_string() + &self.post_data))
    }

    pub fn unique_id(&self) -> String {
        if self.redirection_flag {
            format!("{:x}", md5::compute(self.no_headers_id() + "Redirection"))
        } else {
            self.no_headers_id()
        }
    }

    pub fn post_data_map(&self) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let content_type = self.get_content_type()?;

        if content_type.starts_with("application/json") {
            let result: HashMap<String, serde_json::Value> = serde_json::from_str(&self.post_data)?;
            Ok(result)
        } else if content_type.starts_with("application/x-www-form-urlencoded") {
            let mut result = HashMap::new();
            let parsed = form_urlencoded::parse(self.post_data.as_bytes());
            for (key, value) in parsed {
                result.insert(key.to_string(), serde_json::Value::String(value.to_string()));
            }
            Ok(result)
        } else {
            let mut result = HashMap::new();
            result.insert("key".to_string(), serde_json::Value::String(self.post_data.clone()));
            Ok(result)
        }
    }

    pub fn query_map(&self) -> HashMap<String, Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();
        if let Some(query) = self.url.query() {
            for (key, value) in form_urlencoded::parse(query.as_bytes()) {
                result.entry(key.to_string())
                    .or_insert(Vec::new())
                    .push(value.to_string());
            }
        }
        result
    }

    pub fn get_content_type(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(content_type) = self.headers.get("Content-Type")
            .or(self.headers.get("Content-type"))
            .or(self.headers.get("content-type")) {
            // for &ct in &SUPPORT_CONTENT_TYPE  {
            //     if content_type.starts_with(ct) {
            //         return Ok(content_type.clone());
            //     }
            // }
            if let Some(content_type) = content_type.as_str() {
                for &ct in &SUPPORT_CONTENT_TYPE {
                    if content_type.starts_with(ct) {
                        return Ok(ct.to_string());
                    }
                }
            }
            Err("Unsupported content type".into())
        } else {
            Err("No content type found".into())
        }
    }
}