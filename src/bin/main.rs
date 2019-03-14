use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use ryptor::crypt::{Decryptor, Encryptor};

fn main() -> Result<(), std::io::Error> {
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
                .arg(
                    Arg::with_name("secret")
                        .required(true)
                        .short("s")
                        .long("secret")
                        .takes_value(true),
                )
                .arg(Arg::with_name("path").index(1).required(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("encrypt") => {
            let path = matches
                .subcommand_matches("encrypt")
                .unwrap()
                .value_of("path")
                .unwrap();
            println!("Encrypting: {}", path);

            let encryptor = Encryptor::new();
            encryptor.save_key()?;
            encryptor.encrypt(path)?;
        }
        Some("decrypt") => {
            let matcher = matches.subcommand_matches("decrypt").unwrap();

            let path = matcher.value_of("path").unwrap();

            let secret = matcher.value_of("secret").unwrap();

            println!("Decrypting: {}", path);

            let decryptor = Decryptor::from_file(secret)?;
            decryptor.decrypt(path)?;
        }
        _ => println!("Unknown command"),
    }

    Ok(())
}
