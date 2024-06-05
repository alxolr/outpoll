use notify_rust::Notification;

pub struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Notifier
    }

    pub fn notify(&self, message: &str) {
        Notification::new()
            .summary("OutPoll Watcher")
            .body(message)
            .show()
            .unwrap();
    }
}
