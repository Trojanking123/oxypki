use std::path::PathBuf;

use clap::{arg, command, value_parser, Command};

mod certificate;


fn main() {

    let subcmd_cert_about = "For operating certificate";
    let subcmd_cert = Command::new("cert")
                                .about(subcmd_cert_about)
                                .arg(
                                    arg!(
                                        --in <INFILE> "input file"
                                    )
                                    .required(true)
                                    .value_parser(value_parser!(PathBuf)),
                                );
    let main_cmd = command!() 
        .about("oxypki: a rust oxide tool for PKI")
        .arg(
            arg!(
                -d --debug ... "Turn debugging information on"
            )
            .action(clap::ArgAction::Count)
        )
        .subcommand_required(true)
        .subcommand(
            subcmd_cert
        );

    let matches = main_cmd.get_matches();

    if let Some(cert_matches) = matches.subcommand_matches("cert") {
        let p = cert_matches.get_one::<PathBuf>("in").unwrap();
        certificate::parser_cert(p);
    }


}
