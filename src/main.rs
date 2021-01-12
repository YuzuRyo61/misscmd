#[macro_use]
extern crate clap;
use webbrowser;
use uuid::Uuid;

use clap::{SubCommand, Arg};

fn main() {
    let app = app_from_crate!()
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
        let miauth_uuid = Uuid::new_v4().to_string();
        let open_address = format!(
            "https://{}/miauth/{}\
            ?name=misscmd\
            &permission=\
            read:account,read:drive,write:drive,\
            read:following,write:notes,read:notifications",
            address,
            miauth_uuid
        );
        if let Err(_) = webbrowser::open(&open_address.as_str()) {
            println!(
                "Could not open browser. Please open URL below: \
                {}", &open_address);
        } else {
            println!(
                "Please authenticate with the following URL.\n\
                If you lose your browser, \
                please reopen it at the following URL:\n\
                {}", &open_address);
        };
        println!("If you allow authentication, enter some key.");
    };
}
