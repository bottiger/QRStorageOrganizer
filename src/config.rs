extern crate config;

use config::Config;
use std::env;

lazy_static! {
    static ref SETTINGS: Config = Config::default()
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("STORAGE")).unwrap()
        .merge(config::Environment::with_prefix("AWS")).unwrap().clone();
}

pub fn get_config() -> Config {
    SETTINGS.clone()
}

pub fn init_env() {
    env::set_var(
        "AWS_ACCESS_KEY_ID",
        get_config().get::<String>(&"db_key").unwrap(),
    );
    env::set_var(
        "AWS_SECRET_ACCESS_KEY",
        get_config().get::<String>(&"db_secret").unwrap(),
    );
}
