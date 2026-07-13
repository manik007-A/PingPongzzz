pub struct Identity {
    pub hostname: String,
    pub nickname: String,
    pub fingerprint: String,
}

impl Identity {
    pub fn new() -> Self {
        Self {
            hostname: String::from("UNKNOWN-PC"),
            nickname: String::from("Guest"),
            fingerprint: String::from("NOT_GENERATED"),
        }
    }
}