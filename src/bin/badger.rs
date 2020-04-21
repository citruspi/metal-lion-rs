#[macro_use]
extern crate log;

extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init_custom_env("BADGER_LOG_LEVEL");

    let _ = badger::cli::entrypoint().get_matches();
}
