use hostname::get;

pub struct Identity {
    pub hostname: String,
    pub nickname: String,
    pub fingerprint: String,
}

impl Identity {
    pub fn new() -> Self {

        let hostname = get()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Self {
            hostname,
            nickname: String::from("Guest"),
            fingerprint: String::from("NOT_GENERATED"),
        }
    }
}