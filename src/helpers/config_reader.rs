use config;
use config::Config;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct KittenConfig {
    pub token: String,
    pub verbosity: String,
    pub bot_admins: HashMap<String, config::Value>,
    pub non_admin_response: String,
}
pub fn get(path: PathBuf) -> KittenConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name(&path.to_str().unwrap()))
        .add_source(config::Environment::with_prefix("DISCORD"))
        .build()
        .unwrap();

    let kitten_config = KittenConfig {
        token: settings.get_string("token").unwrap_or_else(|_f| {
            eprintln!("Discord token not specified in config or environment!");
            std::process::exit(1)
        }),

        verbosity: settings.get_string("logging_level").unwrap_or("EMPTY".to_string()),

        bot_admins: settings.get_table("bot_admins").unwrap_or_else(|_f| {
            println!("\x1b[1;33mWarning:\x1b[1;0m bot_admins not specified, certain commands may not be available!");
            let empty: HashMap<String, config::Value> = HashMap::new();
            empty
        }),

        non_admin_response: settings.get_string("non_admin_response").unwrap_or("Not a bot admin!".to_string()),
    };

    return kitten_config;
}
