use std::fmt::Display;

pub enum FileType {
    Toml,
    Json,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = match self {
            FileType::Toml => "toml",
            FileType::Json => "json",
        };
        write!(f, "{}", suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;

    #[test]
    fn test_display() {
        assert_eq!("toml", FileType::Toml.to_string());
    }
}
