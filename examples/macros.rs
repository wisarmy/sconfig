use std::path::PathBuf;

use config::Configurable;
use macros::Toml;
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Serialize, Deserialize, Toml)]
struct AppConfig {
    id: u32,
    name: String,
}
impl Configurable for AppConfig {
    fn config_dir(&self) -> std::path::PathBuf {
        PathBuf::new()
    }
}
fn main() {
    let toml_str = r#"
        id = 123
        name = "test"
        "#;
    println!("from toml string: {}", toml_str);
    let config = toml_str.parse::<AppConfig>().unwrap();
    println!("to config: {:#?}", config);
    config.init().unwrap();
    println!("to toml string: {}", config);
}
