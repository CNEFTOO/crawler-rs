use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read};
use publicsuffix::{List, Psl};
use url::{Url};
use regex::Regex;

#[derive(Debug)]
pub struct MyUrl {
    url: Url
}

impl MyUrl {
    pub fn get_url(&self, _url: &str, parent_urls: Option<&MyUrl>) -> Result<MyUrl, Box<dyn Error>> {
        let mut u = MyUrl{url: Url::parse("")?};

        let parse_url = u.parse(_url, parent_urls)?;

        if let Some(p_url) = parent_urls {
            u.url = Url::parse(p_url.url.as_str())?;
            if u.url.path().is_empty() {
                u.url.set_path("/")
            }
        } else {
            u.url = Url::parse(parse_url.as_str())?;
            if u.url.path().is_empty() {
                u.url.set_path("/")
            }
        }

        // if parent_urls.is_some() {
        //     u.url = Url::parse(parent_urls.unwrap().url.as_str())?;
        //     if u.url.path() == "" {
        //         u.url.set_path("/");
        //     }
        // } else if let Some(p_url) = parent_urls {
        //     u.url = p_url.url.join(parse_url.as_str())?;
        //     if u.url.path() == "" {
        //         u.url.set_path("/");
        //     }
        // }

        let fix_path = Regex::new(r"^/{2,}")?;
        let current_path = u.url.path().to_owned();
        if fix_path.is_match(&current_path) {
            // u.url.set_path(fix_path.replace_all(u.url.path(), "/").as_ref());
            let new_path = fix_path.replace_all(&current_path, "/");
            u.url.set_path(new_path.as_ref());
        }

        Ok(u)
    }

    pub fn parse(&self, _url: &str, parent_urls: Option<&MyUrl>) -> Result<String, Box<dyn Error>> {
        let _url = _url.trim();
        if _url.is_empty() {
            return Err("invalid url, length 0".into());
        }

        if parent_urls.is_none() {
            return Ok(_url.to_string());
        }

        if _url.starts_with("http://") || _url.starts_with("https://") {
            return Ok(_url.to_string());
        }

        if _url.starts_with("javascript:") {
            return Err("invalid url, javascript:".into())
        }

        if _url.starts_with("mailto:") {
            return Err("invalid url, mailto:".into())
        }

        Ok(_url.to_string())
    }

    pub fn query_map(&self) -> HashMap<String, String> {
        let mut query_map: HashMap<String, String> = HashMap::new();
        for (k, v) in self.url.query_pairs() {
            query_map.insert(k.to_string(), v.to_string());
        }
        query_map
    }

    pub fn no_query_url(&self) -> String {
        format!("{}://{}{}", self.url.scheme(), self.url.host_str().unwrap(), self.url.path())
    }

    pub fn no_fragment_url(&self) -> String {
        self.url.as_str().replace(self.url.fragment().unwrap_or(""), "")
    }

    pub fn no_scheme_fragment_url(&self) -> String {
        format!("://{}{}", self.url.host_str().unwrap(), self.url.path())
    }

    pub fn navigation_url(&self) -> String {
        self.no_scheme_fragment_url()
    }

    pub fn root_domain(&self) -> Option<String> {
        let domain = self.url.host_str()?;

        // 读取公共后缀列表文件
        let mut file = match File::open("/tmp/public_suffix_list.dat") {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut contents = Vec::new();
        if let Err(_) = file.read_to_end(&mut contents) {return None};

        // 解析公共后缀列表
        let list_result = List::from_bytes(&contents);

        let list = match list_result {
            Ok(list) => list,
            Err(_) => return None,
        };

        // 获取域名后缀并判断有效性
        let suffix = match list.domain(domain.as_bytes()) {
            Some(suffix) => suffix,
            None => return None,
        };

        let suffix_str = suffix.suffix().as_bytes();

        let i = domain.len() - suffix_str.len() - 1;
        if i <= 0 || domain.as_bytes()[i] != b'.' {
            return None;
        }

        Some(domain[i + 1..].to_string())
    }

    pub fn filename(&self) -> Option<String> {
        self.url.path_segments()?.last().map(|s| s.to_string())
    }

    pub fn file_extension(&self) -> String {
        let path = self.url.path();
        path.rsplit('.').next().unwrap_or("").to_string()
    }

    pub fn parent_path(&self) -> Option<String> {
        let path = self.url.path();
        if path == "/" {
            None
        } else if path.ends_with('/') {
            if path.split('/').count() == 2 {
                Some("/".to_string())
            } else {
                Some(path.split('/').skip(1).collect())
            }
        } else {
            Some(path.rsplit('/').skip(1).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_absolute_url() {
        let my_url = MyUrl{url: Url::parse("https://example.com").unwrap()};

        let result = my_url.parse("https://example.com/path", None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://example.com/path");
    }

    #[test]
    fn test_parse_relative_url() {
        let parent_url = MyUrl{
            url: Url::parse("https://example.com").unwrap(),
        };

        let my_url = MyUrl{
            url: Url::parse("https://www.example.com").unwrap(),
        };

        let result = my_url.parse("/path", Some(&parent_url));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "/path");
    }
}