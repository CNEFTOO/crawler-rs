use console::Style;

pub fn logger_message(level: &str, message: &str, color: Style) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_info() {
        logger_info("info message");
    }

    #[test]
    fn test_logger_warning() {
        logger_warning("warn message");
    }

    #[test]
    fn test_logger_error() {
        logger_error("error message");
    }

    #[test]
    fn test_logger_debug() {
        logger_debug("debug message");
    }

    #[test]
    fn test_logger_trace() {
        logger_trace("trace message");
    }

    #[test]
    fn test_logger_highlight() {
        logger_highlight("highlight message");
    }

    #[test]
    fn test_logger_success() {
        logger_success("success message");
    }
}