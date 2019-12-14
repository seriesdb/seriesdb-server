use config::{Config, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_port: u32,
    pub data_dir: String,
    pub write_buffer_size: usize,
    pub max_write_buffer_number: i32,
    pub min_write_buffer_number_to_merge: i32,
    pub max_background_compactions: i32,
    pub max_background_flushes: i32,
    pub replication_enabled: bool,
    pub master_endpoint: String,
}

impl Settings {
    pub fn new() -> Self {
        let mut settings = Config::default();
        settings
            .merge(File::with_name("config/settings.toml"))
            .unwrap();
        settings.try_into().unwrap()
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = { Settings::new() };
}
