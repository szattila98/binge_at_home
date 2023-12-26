#![allow(clippy::print_stdout)]

pub mod api;
pub mod configuration;
pub mod crud;
pub mod database;
pub mod elastic;
pub mod file_access;
pub mod logging;
pub mod model;
pub mod startup;

pub fn print_banner() {
    print!("{}", include_str!("../resources/banner.txt"));
    println!("{} - {}\n", get_app_name(), get_app_version());
}

pub const fn get_app_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub const fn get_app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
