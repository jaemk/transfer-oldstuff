extern crate transfer;
#[macro_use] extern crate clap;

use transfer::errors::*;
use clap::{Arg, App, SubCommand, ArgMatches};


pub fn main() {
    let matches = App::new("transfer")
        .version(crate_version!())
        .author("James K. <james.kominick@gmail.com")
        .about("  ** Secure file transfer utility **")
        .arg(Arg::with_name("server-root")
             .long("server-root")
             .takes_value(true)
             .help("Proxy server root to use, overrides default ..."))
        .subcommand(SubCommand::with_name("up")
                    .about("Upload files")
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .takes_value(true)
                         .help("file to upload"))
                    .arg(Arg::with_name("secret")
                         .short("s")
                         .long("secret")
                         .takes_value(true)
                         .help("secret token"))
                    .arg(Arg::with_name("lifespan")
                         .short("l")
                         .long("lifespan")
                         .takes_value(true)
                         .help("lifespan (in seconds) of item on server")))
        .subcommand(SubCommand::with_name("down")
                    .about("Download files")
                    .arg(Arg::with_name("uid")
                         .long("id")
                         .takes_value(true))
                    .arg(Arg::with_name("secret")
                         .short("s")
                         .long("secret")
                         .takes_value(true)
                         .help("secret token")))
        .get_matches();

    if let Err(ref e) = run(matches) {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<()> {
    let server = matches.value_of("server-root").unwrap_or("transfer-root");
    println!("server: {}", server);
    if let Some(ref matches) = matches.subcommand_matches("up") {
        println!("file: {:?}, secret: {:?}, lifespan[s]: {:?}",
                 matches.value_of("file"), matches.value_of("secret"), matches.value_of("lifespan"));
        return Ok(())
    }
    if let Some(ref matches) = matches.subcommand_matches("down") {
        println!("down matches: {:?}", matches);
    }
    Ok(())
}
