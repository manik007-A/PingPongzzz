use crate::config::app_config::AppConfig;
use crate::identity::identity_manager::Identity;
use crate::logging::logger::Logger;

pub fn start() {
    Logger::info("Application Started");
    Logger::info("Loading Configuration");

    let config = AppConfig::new();

    Logger::info("Configuration Loaded");
    Logger::info("Loading Identity");

    let identity = Identity::new();

    Logger::info("Identity Loaded");

    println!();
    println!("========== Application ==========");
    println!("Application Name   : {}", config.app_name);
    println!("Version            : {}", config.version);
    println!("UDP Port           : {}", config.udp_port);
    println!("Heartbeat Interval : {} Seconds", config.heartbeat_interval);
    println!("Max Connections    : {}", config.max_connections);

    println!();
    println!("========== Identity ==========");
    println!("Hostname           : {}", identity.hostname);
    println!("Nickname           : {}", identity.nickname);
    println!("Fingerprint        : {}", identity.fingerprint);
}