use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command, ArgMatches};


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
    let mut main_cmd = command!() 
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

    let long_help = main_cmd.render_help();
    let matches = main_cmd.get_matches();
        

    let match_subcmd_vec = match matches.subcommand() {
        Some(x) => x,
        None => {
            println!("{long_help}");
            unreachable!("subcommand is required");
        }
    };

}
