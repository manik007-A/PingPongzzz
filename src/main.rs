mod app;
mod config;
mod identity;
mod logging;
mod database;
mod models;

fn main() {
    println!("=================================");
    println!("   Rubix - PingPongzzz");
    println!("=================================");

    app::start();
}