use srm::{args::ArgParser, core::Srm};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: srm <file1> <file2> ...");
        std::process::exit(1);
    }

    let srm = Srm::new();
    match ArgParser::parse(&args) {
        Ok(parsed_args) => match srm.run(&parsed_args) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
