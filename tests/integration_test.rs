use toml_reader::{get, init};

#[test]
fn test_config() {
    init("tests/config.toml").expect("Failed to init config");
    let config = get();
    println!("Database host: {}", config.database.host);
    println!("Database port: {}", config.database.port);
}
