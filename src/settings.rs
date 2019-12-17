use config::{Config, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_port: u32,
    pub data_dir: String,

    pub table_cache_num_shard_bits: i32,
    pub write_buffer_size: usize,
    pub max_write_buffer_number: i32,
    pub min_write_buffer_number_to_merge: i32,
    pub max_bytes_for_level_base: u64,
    pub max_bytes_for_level_multiplier: f64,
    pub target_file_size_base: u64,
    pub target_file_size_multiplier: i32,
    pub level_zero_file_num_compaction_trigger: i32,
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
