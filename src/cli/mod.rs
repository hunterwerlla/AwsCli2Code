use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
static LANGUAGES: [&str; 2] = ["java", "javascript"];

pub enum SdkVersion {
    Latest,
    Number { value: i32 },
}

pub struct ParsedArguments {
    pub version: SdkVersion,
    pub language: String,
    pub aws_cli_input: Vec<String>,
}

pub fn parse_command_line() -> ParsedArguments {
    let matches = App::new(NAME)
        .version(VERSION)
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .possible_values(&LANGUAGES)
                .required(true),
        )
        .arg(
            Arg::new("sdk version")
                .short('s')
                .long("sdk-version")
                .default_value("latest")
                .about("Which SDK version to use"),
        )
        .arg(Arg::new("AWS cli command").multiple(true).last(true))
        .get_matches();
    let language = match matches.value_of("language") {
        Some(language) => language,
        None => panic!("Somehow got no language!"),
    };
    let sdk_version = match matches.value_of("sdk version") {
        None => SdkVersion::Latest,
        Some(version) => {
            if version == "latest" {
                SdkVersion::Latest
            } else {
                match version.parse::<i32>() {
                    Ok(num) => SdkVersion::Number { value: num },
                    Err(_) => panic!("Sdk version must be an integer or latest"),
                }
            }
        }
    };
    let aws_cli_input = match matches.values_of("AWS cli command") {
        Some(values) => values.map(|f| f.to_string()).collect::<Vec<String>>(),
        None => panic!("Need aws cli input!"),
    };
    ParsedArguments {
        version: sdk_version,
        language: language.to_string(),
        aws_cli_input,
    }
}
