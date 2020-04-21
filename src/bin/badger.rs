#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use badger;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("BADGER_LOG_LEVEL");

    let rt = badger::cli::entrypoint().get_matches();

    match rt.subcommand_name() {
        Some(v) => {
            let args = rt.subcommand_matches(v).unwrap();

            match v {
                "server" => {
                    let bind_addr: SocketAddr = args
                        .value_of("bind")
                        .unwrap()
                        .parse()
                        .expect("Failed to parse bind address");

                    badger::web::listen(bind_addr).await;
                }
                _ => error!("unrecognized subcommand"),
            }
        }
        None => error!("no subcommand specified"),
    }
}
