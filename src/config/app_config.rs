pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub udp_port: u16,
    pub heartbeat_interval: u64,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            app_name: String::from("Rubix - PingPongzzz"),
            version: String::from("1.0.0"),
            udp_port: 9876,
            heartbeat_interval: 10,
            max_connections: 10,
        }
    }
}