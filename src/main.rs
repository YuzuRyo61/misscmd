#[macro_use]
extern crate clap;
use webbrowser;

use clap::{App, SubCommand, Arg};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("login")
                .about("Login to Misskey instance")
                .arg(
                    Arg::with_name("address")
                        .help("Specify to address")
                        .required(true)
                )
        );

    let matches = app.get_matches();

    if let Some(ref matches_login) = matches.subcommand_matches("login") {
        let address = matches_login.value_of("address").unwrap();
        if let Err(_) = webbrowser::open(&format!("https://{}/", address)) {
            println!("Could not open browser. Please open URL below: ");
        };
    };

}
