use std::env;
use influent::client::Credentials;

pub struct Config {
    pub host: String,
    pub interval: u64,
    username: String,
    password: String,
    database: String,
}

impl Config {
    pub fn from_env() -> Config {
        let env = |var: &str, default: &str| env::var(var).unwrap_or(String::from(default));

        let host = env("INFLUX_URL", "http://localhost");
        let port = env("INFLUX_PORT", "8086");

        Config {
            host: format!("{}:{}", host, port),
            interval: env("INTERVAL", "3").parse().unwrap_or(3),
            username: env("INFLUX_USER", "admin"),
            password: env("INFLUX_PASSWORD", "admin"),
            database: env("INFLUX_DATABASE", "citybike")
        }
    }

    pub fn credentials(&self) -> Credentials {
        Credentials {
            username: &self.username,
            password: &self.password,
            database: &self.database
        }
    }
}
