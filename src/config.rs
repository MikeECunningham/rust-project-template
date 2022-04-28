/// This file contains a global instance of readonly values that can be used across the application.
/// 
/// Any fields added to the config struct will inform the application to attempt to load an environment
/// variable from the system by that name.
/// NOTE: The name of the environment variable it loads will be in all uppercase,
/// so 'example_key' becomes 'EXAMPLE_KEY' 
use serde::{Deserialize};


/// Describes configurations that originate from the applications environment
#[derive(Deserialize)]
pub struct Config {
    pub env: String,
    /// Test value
    pub test_value: String,
}

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env::<Config>().expect("Failed to load config from environment");
}