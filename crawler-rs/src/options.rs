use clap::{Arg, ArgAction, Command};

pub fn banner() {}

pub fn cli() -> Command {
    Command::new("crawler-rs")
        .version("0.1.0")
        .author("seaung <https://www.github.com/seaung>")
        .about("A Crawler CLI tools")
        .arg(
            Arg::new("chromium-path")
                .short('c')
                .long("chromium-path")
                .value_name("CHROMIUM_PATH")
                .help("The path to the chromium")
                .required(true)
        )
        .arg(
            Arg::new("chromium-ws-url")
                .short('w')
                .long("chromium-ws-url")
                .value_name("CHROMIUM_WS_URL")
                .help("The url to the chromium ws server")
                .required(true)
        )
        .arg(
            Arg::new("custom-headers")
                .long("custom-headers")
                .value_name("CUSTOM_HEADERS")
                .help("The custom headers to use")
                .default_value(r#"{"Spider-Name: "crawler-rs", "User-Agent": "Crawler-rust"}"#)
        )
        .arg(
            Arg::new("post-data")
                .short('d')
                .long("post-data")
                .value_name("POST DATA")
                .help("The data from the crawler")
                .required(true)
        )
        .arg(
            Arg::new("no-headless")
                .long("no-headless")
                .action(ArgAction::SetTrue)
                .help("Disable headless mode"),
        )
        .arg(
            Arg::new("max-crawled-count")
                .long("max-crawled-count")
                .short('m')
                .help("Maximum number of crawled crawlers")
                .default_value("100")
        )
        .arg(
            Arg::new("filter-mode")
                .short('f')
                .long("filter-mode")
                .value_name("FILTER_MODE")
                .help("Filter mode")
                .default_value("smart")
        )
        .arg(
            Arg::new("output-mode")
                .short('o')
                .long("output-mode")
                .value_name("OUTPUT_MODE")
                .help("Output mode")
                .default_value("console")
        )
}