use clap::{App, Arg, ArgMatches, SubCommand};

pub(crate) fn get_matches<'a>() -> ArgMatches<'a> {
  App::new({{crate_name}}_core::APPNAME)
    .version("0.0.20")
    .author(clap::crate_authors!())
    .about("A work in progress")
    .arg(
      Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("DIRECTORY")
        .help("Sets a custom config directory (defaults to \".\")")
        .takes_value(true)
    )
    .arg(
      Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .help("Display more output")
        .takes_value(false)
    )
    .subcommand(
      SubCommand::with_name("server")
        .arg(
          Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("IP_ADDRESS")
            .help("Defines the host to listen on (defaults to \"127.0.0.1\")")
            .takes_value(true)
        )
        .arg(
          Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT_NUM")
            .help("Configures the server to use the provided port (defaults to 5500)")
            .takes_value(true)
        )
    )
    .get_matches()
}
