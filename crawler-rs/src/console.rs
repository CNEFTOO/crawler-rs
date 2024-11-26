use crate::options;

pub fn start() {
    let cli = options::cli();
    let matches = cli.get_matches();

    if let Some(chromium_path) = matches.get_one::<String>("chromium-path") {}

}