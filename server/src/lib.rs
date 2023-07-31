pub mod api;
pub mod configuration;
pub mod database;
pub mod logging;
pub mod startup;
pub mod swagger;

pub fn print_banner() {
    print!("{}", include_str!("../resources/banner.txt"));
    println!(
        "{} - {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
