use std::path::PathBuf;

use sconfig::{Configurable, FileType, Toml};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Toml)]
#[toml(inline)]
struct AppConfig {
    id: u32,
    name: String,
    db_config: DBConfig,
    roles: Vec<Role>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DBConfig {
    host: String,
    name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Role {
    name: String,
}

impl Configurable for AppConfig {
    fn config_dir(&self) -> std::path::PathBuf {
        PathBuf::new()
    }

    fn config_type(&self) -> FileType {
        FileType::Toml
    }
}

fn main() {
    let toml_str = r#"
        id = 123
        name = "test"
        [db_config]
        host = "localhost"
        name = "test"
        [[roles]]
        name = "root"
        [[roles]]
        name = "operator"
        "#;
    println!("from toml string: {}", toml_str);
    let config = toml_str.parse::<AppConfig>().unwrap();
    println!("to config: {:#?}", config);
    config.init(true).unwrap();

    let toml_str = config.to_string();
    println!("to toml string: {}", toml_str);
}
