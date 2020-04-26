#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use badger;
use glyph_bbox::dataset;
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

                    let factory = badger::badges::Factory::new(badger::badges::FactoryOptions {
                        render_dataset: dataset::DataSet::from_file(dataset::ReadOptions {
                            filename: args.value_of("bbox_dataset_path").unwrap().into(),
                            format: dataset::Format::JSON,
                        }),
                    });

                    badger::web::listen(bind_addr, factory).await;
                }
                _ => error!("unrecognized subcommand"),
            }
        }
        None => error!("no subcommand specified"),
    }
}
