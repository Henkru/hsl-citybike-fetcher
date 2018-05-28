extern crate geohash;
extern crate influent;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate cached;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

mod station;
mod importer;
mod fetcher;
mod config;

fn main() {
    env_logger::init();
    let config  = config::Config::from_env();
    let credentials = config.credentials();

    info!("fetcher config:");
    info!("\thost: {}", config.host);
    info!("\tuser: {}", credentials.username);
    info!("\tdatabase: {}", credentials.database);
    info!("\tinterval: {} minute(s)", config.interval);

    let imp = importer::Importer::new(credentials, &config.host);
    loop {
        match fetcher::fetch()
            .and_then(|stations| imp.add(stations)) {
                Ok(_) => debug!("Updated stations"),
                Err(err) => warn!("{}", err),
        };
        std::thread::sleep(std::time::Duration::from_secs(config.interval*60));
    }
}
