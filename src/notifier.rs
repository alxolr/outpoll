use std::process::Command;

pub struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Notifier
    }

    pub fn notify(&self, message: &str) {
        Command::new("notify-send")
            .arg(message)
            .output()
            .expect("Failed to send notification");
        // println!("Notifying: {}", message);
    }
}
