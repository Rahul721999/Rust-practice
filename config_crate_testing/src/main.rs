// src/main.rs
use config::{Config, File, FileFormat};
use secrecy::{Secret, ExposeSecret};

#[derive(Debug,serde::Deserialize)]
pub struct Settings {
pub database: DatabaseSettings,
pub application: ApplicationSettings,
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
        let config = Config::builder().add_source(File::new("config", FileFormat::Yaml));
        let config = config.build().expect("failed to get config");
        let settings = config.clone().try_deserialize::<Settings>().expect("failed to get the settings");
        let app :ApplicationSettings  = config.get::<ApplicationSettings>("application").expect("failed to get the Application settings");
        let db: DatabaseSettings = config.get::<DatabaseSettings>("database").expect("failed to get the db settings");
        // let name = app.get("app").expect("value not found").clone().try_deserialize::<AppConfig>().expect("failed");
        println!("app-port {}, app-host{}", app.port, app.host);
        println!("password : {:?}", db.password.expose_secret());
        println!("app-port {}, app-host{}, datahase-name:{}", 
        settings.application.port,
        settings.application.host,
        settings.database.database_name
        )
    }
}

fn main()  {
    Settings::from_file();
}
