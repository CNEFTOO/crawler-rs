use console::Style;

pub fn logger_message(level: &str, message: &str, color: Style) {
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    println!("{} [{}] {}", color.apply_to(level), timestamp, message);
}

pub fn logger_info(message: &str) {
    let style = Style::new().blue().bold();
    logger_message("INFO", message, style);
}

pub fn logger_warning(message: &str) {
    let style = Style::new().yellow().bold();
    logger_message("WARN", message, style);
}

pub fn logger_error(message: &str) {
    let style = Style::new().red().bold();
    logger_message("ERROR", message, style);
}

pub fn logger_debug(message: &str) {
    let style = Style::new().blue().bold();
    logger_message("DEBUG", message, style);
}

pub fn logger_trace(message: &str) {
    let style = Style::new().yellow().bold();
    logger_message("TRACE", message, style);
}

pub fn logger_highlight(message: &str) {
    let style = Style::new().yellow().bold();
    logger_message("HIGHLIGHT", message, style);
}

pub fn logger_success(message: &str) {
    let style = Style::new().green().bold();
    logger_message("SUCCESS", message, style);
}