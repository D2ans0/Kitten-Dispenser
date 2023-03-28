use std::collections::HashMap;
use tracing::*;

pub fn start(level_string: &String) {
    let level = map_level(level_string);

    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
    let _subscriber = tracing_subscriber::FmtSubscriber::new();

}

fn map_level(level_string: &String) -> Level{
    let map: HashMap<&str, Level> = HashMap::from([
        ("ERROR", Level::ERROR),
        ("WARN", Level::WARN),
        ("INFO", Level::INFO),
        ("DEBUG", Level::DEBUG),
        ("TRACE", Level::TRACE),
    ]);

    return map.get(level_string.as_str()).unwrap_or(&Level::INFO).to_owned();
}