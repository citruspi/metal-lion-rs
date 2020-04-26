use clap::{App, Arg, SubCommand};

pub fn entrypoint<'b, 'a>() -> App<'a, 'b> {
    App::new("badger")
        .version(crate_version!())
        .author("Mihir Singh (@citruspi)")
        .subcommand(
            SubCommand::with_name("server")
                .about("run rendering server")
                .arg(
                    Arg::with_name("bind")
                        .takes_value(true)
                        .short("b")
                        .long("bind")
                        .help("address to bind to")
                        .default_value("127.0.0.1:4352"),
                )
                .arg(
                    Arg::with_name("bbox_dataset_path")
                        .takes_value(true)
                        .short("r")
                        .long("render-dataset")
                        .help("glyph bounding box dataset path")
                        .required(true),
                ),
        )
}
