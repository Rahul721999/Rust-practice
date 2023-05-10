// src/main.rs
use config::{Config, File};
use secrecy::{Secret};

#[derive(Debug,serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings
}
#[derive(Debug,serde::Deserialize)]
pub struct ApplicationSettings {
pub port: u16,
pub host: String,
}
#[derive(Debug,serde::Deserialize)]
pub struct DatabaseSettings {
pub username: String,
pub password: Secret<String>,
pub port: u16,
pub host: String,
pub database_name: String,
}
impl Settings {
    pub fn from_file(){
        let base_path =  std::env::current_dir().expect("Failed to get the curr dir");
        let config_dir = base_path.join("configuration");
        let config = Config::builder().add_source(File::from(config_dir.join("config")));
        let con = match config.add_source(File::from(config_dir.join("base"))).build(){
            Ok(config) => config,
            Err(err) => panic!("{}",err),
        };
        let settings = con.try_deserialize::<Settings>().expect("Failed");
        println!("{}", settings.application.host);
    }
}

fn main()  {
    Settings::from_file();
}
