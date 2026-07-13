use crate::config::app_config::AppConfig;
use crate::database::database::Database;
use crate::database::schema::Tables;
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
    Logger::info("Initializing Database");

    match Database::initialize() {
        Ok(connection) => {
            Logger::info("Database Ready");

            if let Err(error) = Tables::create(&connection) {
                Logger::error(&format!("Schema Error: {}", error));
                return;
            }

            Logger::info("Database Schema Ready");

            if let Err(error) = Database::save_identity(
                &connection,
                &identity.hostname,
                &identity.nickname,
                &identity.fingerprint,
            ) {
                Logger::error(&format!("Save Error: {}", error));
                return;
            }

            Logger::info("Identity Saved Successfully");
        }

        Err(error) => {
            Logger::error(&format!("Database Error: {}", error));
            return;
        }
    }

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