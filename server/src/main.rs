extern crate server;
#[macro_use] extern crate clap;

use server::errors::*;
use clap::{Arg, App, SubCommand, ArgMatches};


pub fn main() {
    let matches = App::new("transfer")
        .version(crate_version!())
        .subcommand(SubCommand::with_name("serve")
                    .about("Initialize Server")
                    .arg(Arg::with_name("database")
                         .long("db-url")
                         .takes_value(true)
                         .help("Postgres database URL to connect to"))
                    .arg(Arg::with_name("port")
                         .long("port")
                         .short("p")
                         .takes_value(true)
                         .help("Port to listen on. Defaults to 3000"))
                    .arg(Arg::with_name("silent")
                         .long("silent")
                         .takes_value(false)
                         .help("Suppress stdout logging"))
                    .arg(Arg::with_name("public")
                         .long("public")
                         .help("serve of '0.0.0.0' instead of 'localhost'")))
        .get_matches();

    if let Err(ref e) = run(matches) {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let stderr_msg = "Error writing to stderr";
        writeln!(stderr, "error: {}", e).expect(stderr_msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(stderr_msg);
        }

        // `RUST_BACKTRACE=1`
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(stderr_msg);
        }

        ::std::process::exit(1);
    }
}


fn run(matches: ArgMatches) -> Result<()> {
    if let Some(serve_matches) = matches.subcommand_matches("serve") {
        let port = serve_matches.value_of("port").unwrap_or("3000");
        let db_url = serve_matches.value_of("database");
        let log = !serve_matches.is_present("silent");
        let host_base = if serve_matches.is_present("public") { "0.0.0.0" } else { "localhost" };
        let host = format!("{}:{}", host_base, port);
        server::service::start(&host, db_url, log);
        return Ok(());
    }
    Ok(())
}
