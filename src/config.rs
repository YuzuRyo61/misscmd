use std::fs;
use std::io::{BufReader, Read};

#[derive(Deserialize)]
pub struct Config {
    pub account: ConfigAccount
}

#[derive(Deserialize)]
pub struct ConfigAccount {
    pub address: String,
    pub token: String
}

pub fn get_config() -> Result<Option<Config>, String> {
    let conf_path = dirs::home_dir().unwrap().join(".misscmd").join("config.toml");
    if conf_path.exists() {
        let mut raw_config_string = String::new();

        let mut file_stream = fs::File::open(conf_path)
            .map(|f| BufReader::new(f))
            .map_err(|e| e.to_string())?;

        file_stream.read_to_string(&mut raw_config_string)
            .map_err(|e| e.to_string())?;

        let conf: Config = toml::from_str(&raw_config_string)
            .map_err(|e| e.to_string())?;

        Ok(Some(conf))
    } else {
        Ok(None)
    }
}
