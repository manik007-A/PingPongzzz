pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!("[INFO] {}", message);
    }

    pub fn warning(message: &str) {
        println!("[WARNING] {}", message);
    }

    pub fn error(message: &str) {
        println!("[ERROR] {}", message);
    }
}