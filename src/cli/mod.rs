use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

static SDKS: [&str; 2] = ["javav2", "javascriptv2"];
static LANGUAGES: [&str; 4] = ["java", "kotlin", "javascript", "typescript"];

pub struct ParsedArguments {
    pub language: Option<String>,
    pub sdk: String,
    pub aws_cli_input: Vec<String>,
}

pub fn parse_command_line() -> ParsedArguments {
    let matches = App::new(NAME)
        .version(VERSION)
        .arg(
            Arg::new("sdk")
                .short('s')
                .long("sdk")
                .default_value("latest")
                .about("Which AWS SDK to use")
                .possible_values(&SDKS)
                .required(true),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .about("Which langugage to generate for. Options are based on the chosen AWS SDK.")
                .possible_values(&LANGUAGES),
        )
        .arg(Arg::new("AWS cli command").multiple(true).last(true))
        .get_matches();
    let language = matches.value_of("language");
    let sdk = match matches.value_of("sdk") {
        None => panic!("somehow did not get an sdk!"),
        Some(s) => s,
    };
    let aws_cli_input = match matches.values_of("AWS cli command") {
        Some(values) => values.map(|f| f.to_string()).collect::<Vec<String>>(),
        None => panic!("Need aws cli input!"),
    };
    ParsedArguments {
        sdk: sdk.to_string(),
        language: language.map(str::to_string),
        aws_cli_input,
    }
}
