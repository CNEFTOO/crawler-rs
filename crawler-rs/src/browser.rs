use std::ffi::OsStr;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use headless_chrome::{Browser, LaunchOptions, Tab};

pub struct BrowserManager {
    browser: headless_chrome::Browser,
}

impl BrowserManager {
    pub fn new(
        headless: bool,
        proxy: Option<&str>,
        extract_args: Vec<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = extract_args;
        if let Some(proxy) = proxy {
            args.push(format!("--proxy-server={}", proxy));
        };

        let args_options: Vec<&OsStr> = args.iter().map(|arg| OsStr::new(arg)).collect();

        let options = LaunchOptions{
            headless,
            args: args_options,
            ..Default::default()
        };

        let b = Browser::new(options).map_err(|err| err.into());
        Ok(Self { browser })
    }

    pub fn new_tab(&self, timeout: Option<Duration>) -> Result<Arc<Tab>, Box<dyn std::error::Error>>  {
        let (tx, rx) = mpsc::channel();
        let browser = self.browser.clone();
        std::thread::spawn(move || {
            let tab = browser.new_tab();
            let _ = tx.send(tab);
        });
        let tab_arc = match timeout {
            Some(timeout) => rx.recv_timeout(timeout).map_err(|e| match e {
                mpsc::RecvTimeoutError::Timeout => "create new tab error".into(),
                mpsc::RecvTimeoutError::Disconnected => "disconnect error".into(),
            })?,
            None => rx.recv().map_err(|_| "new browser tab failed".into())?,
        };

        tab_arc.map_err(|e| Box::<dyn std::error::Error>::from(e).into())
    }

    pub fn close_tab(&self, tab: Arc<Tab>, fire_unload: bool) -> Result<bool, Box<dyn std::error::Error>> {
        tab.close(fire_unload).map_err(|e| Box::<dyn std::error::Error>::from(e))
    }
}