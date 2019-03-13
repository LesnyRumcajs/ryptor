use clap::{App, Arg, SubCommand, crate_version, crate_authors, crate_name, crate_description};
use ryptor::crypt::Encryptor;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("encrypt").about("Encrypts data").arg(
                Arg::with_name("path")
                    .index(1)
                    .required(true)
            ),
        )
        .subcommand(
            SubCommand::with_name("decrypt").about("Decrypts data").arg(
                Arg::with_name("path")
                    .index(1)
                    .required(true)
            ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("encrypt") => {
            println!("Encrypting: {}", matches.subcommand_matches("encrypt").unwrap().value_of("path").unwrap());
            let result = Encryptor::new();
        },
        Some("decrypt") => {
            println!("Decrypting: {}", matches.subcommand_matches("decrypt").unwrap().value_of("path").unwrap())
        },
        _ => println!("Unknown command")
    }
}
