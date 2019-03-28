use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

use log::trace;
use simplelog::{LevelFilter, SimpleLogger};

use ryptor::decryptor::Decryptor;
use ryptor::encryptor::Encryptor;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> Result<(), std::io::Error> {
    if SimpleLogger::init(LevelFilter::Trace, simplelog::Config::default()).is_err() {
        println!("Failed to init logger. Exiting.");
        std::process::exit(1);
    }

    trace!("Application start");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Encrypts data")
                .arg(Arg::with_name("path").index(1).required(true))
                .arg(
                    Arg::with_name("to_secret")
                        .long("to-secret")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("from_secret")
                        .short("s")
                        .long("from-secret")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Decrypts data")
                .arg(
                    Arg::with_name("from_secret")
                        .required(true)
                        .short("s")
                        .long("from-secret")
                        .takes_value(true),
                )
                .arg(Arg::with_name("path").index(1).required(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("encrypt") => {
            let matcher = matches.subcommand_matches("encrypt").unwrap();

            let path = matcher.value_of("path").unwrap();

            let encryptor = if matcher.is_present("from_secret") {
                Encryptor::from_secret(Path::new(matcher.value_of("from_secret").unwrap()))?
            } else {
                let encryptor = Encryptor::new();
                encryptor.save_key(Path::new(
                    matcher.value_of("to_secret").unwrap_or("secret.key"),
                ))?;
                encryptor
            };

            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
            {
                encryptor.encrypt(entry.path())?;
            }
        }
        Some("decrypt") => {
            let matcher = matches.subcommand_matches("decrypt").unwrap();

            let path = matcher.value_of("path").unwrap();
            let secret = matcher.value_of("from_secret").unwrap();

            let decryptor = Decryptor::from_file(Path::new(secret))?;
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
            {
                decryptor.decrypt(entry.path())?;
            }
        }
        _ => println!("Unknown command"),
    }

    trace!("Application clean exit");
    Ok(())
}
