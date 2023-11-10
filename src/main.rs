use std::path::PathBuf;

use clap::{arg, command, value_parser, Command, ArgAction};
use error::{PkiError, PkiResult};
use utils::FileFormat;

mod certificate;
mod error;
mod utils;

fn main() {
    let in_file_arg = arg!(
        --in <INFILE> "Input file, default pem file format"
    )
    .required(true)
    .value_parser(value_parser!(PathBuf));

    let in_file_form = arg!(
        --inform  "Specific the input file format, default to pem"
    )
    .value_parser(["pem", "der"])
    .default_value("pem")
    .action(ArgAction::Set);


    let subcmd_cert_about = "For operating certificate";
    let subcmd_cert = Command::new("cert")
        .about(subcmd_cert_about)
        .arg(in_file_arg.clone())
        .arg(in_file_form.clone());
    let main_cmd = command!()
        .about("oxypki: a rust oxide tool for PKI")
        .arg(
            arg!(
                -d --debug ... "Turn debugging information on"
            )
            .action(clap::ArgAction::Count),
        )
        .subcommand_required(true)
        .subcommand(subcmd_cert);

    let matches = main_cmd.get_matches();

    if let Some(cert_matches) = matches.subcommand_matches("cert") {
        let pb = cert_matches.get_one::<PathBuf>("in").unwrap();
        let tp = cert_matches.get_one::<String>("inform").unwrap();
        let tp: FileFormat = (&tp).parse().unwrap();
        certificate::parser_cert(pb, tp).unwrap();
    }
}
