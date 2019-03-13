use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use ryptor::crypt::Encryptor;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Encrypts data")
                .arg(Arg::with_name("path").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Decrypts data")
                .arg(Arg::with_name("path").index(1).required(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("encrypt") => {
            println!(
                "Encrypting: {}",
                matches
                    .subcommand_matches("encrypt")
                    .unwrap()
                    .value_of("path")
                    .unwrap()
            );
            let encryptor = Encryptor::new();
            match encryptor.save_key() {
                Ok(result) => println!("Saved the key to: {}", result),
                Err(e) => println!("Error saving file: {}", e),
            }
        }
        Some("decrypt") => println!(
            "Decrypting: {}",
            matches
                .subcommand_matches("decrypt")
                .unwrap()
                .value_of("path")
                .unwrap()
        ),
        _ => println!("Unknown command"),
    }
}
