use std::path::PathBuf;

use config::{Configurable, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Json)]
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
        FileType::Json
    }
}

fn main() {
    let json_str = r#"
    {
        "id": 123,
        "name": "test",
        "db_config": {
            "host": "localhost",
            "name": "test"
        },
        "roles": [{"name": "root"}, {"name": "operator"}]
    }
    "#;
    println!("from json string: {}", json_str);

    let config = json_str.parse::<AppConfig>().unwrap();
    println!("to config: {:#?}", config);
    config.init().unwrap();
    let json_str = serde_json::to_string_pretty(&config).unwrap();
    println!("to json string: {}", json_str);
}
