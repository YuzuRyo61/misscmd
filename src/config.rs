use std::io::{BufReader, Read, Write};
use std::fs::{self, File};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub account: ConfigAccount
}

#[derive(Serialize, Deserialize)]
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

pub fn save_config(value: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let conf_dir = dirs::home_dir().unwrap().join(".misscmd");
    let conf_path = &conf_dir.join("config.toml");
    fs::create_dir_all(conf_dir)?;
    let mut file_stream = File::create(conf_path)?;
    write!(file_stream, "{}", toml::to_string(value).unwrap())?;
    file_stream.flush()?;
    Ok(())
}
