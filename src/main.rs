#[macro_use]
extern crate clap;
use webbrowser;
use uuid::Uuid;

use clap::{SubCommand, Arg};
use misscmd::pause;
use misscmd::config::{get_config, Config, ConfigAccount, save_config};
use std::process;

use misscmd::model::MiAuthResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app_from_crate!()
        .subcommand(
            SubCommand::with_name("login")
                .about("Login to Misskey instance")
                .arg(
                    Arg::with_name("address")
                        .help("Specify to address")
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Make a new note")
                .arg(
                    Arg::with_name("body")
                        .help("What's happened?")
                        .required(true)
                )
        );

    let matches = app.get_matches();
    let config = get_config().unwrap();

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
        let auth_check_url = format!(
            "https://{}/api/miauth/{}/check",
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
        loop {
            pause("If you allowed authentication, Please press Enter key.\n");
            let http_cli = reqwest::Client::new();
            let res = http_cli.post(auth_check_url.as_str())
                .send().await;
            if let Ok(ok_res) = res {
                let res_json: MiAuthResponse = ok_res.json().await.unwrap();
                if res_json.ok == false {
                    println!("Authenticate error. Please try again.");
                    continue;
                } else {
                    let token = res_json.token.unwrap();
                    let save_cfg = Config {
                        account: ConfigAccount {
                            address: address.to_string(),
                            token
                        }
                    };
                    save_config(&save_cfg).unwrap();
                    println!("Authenticate Successfully!");
                    return Ok(())
                }
            } else {
                println!("Connection error. Please try again.");
            }
        };
    };

    if let Some(_) = matches.subcommand_matches("new") {
        if let None = config {
            eprintln!(
                "Please login to Misskey instance first!\n\
                Help: misscmd login <instance_address>"
            );
            process::exit(1);
        }
    }

    Ok(())
}
