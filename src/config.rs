use clap::Arg;

#[derive(Clone, Debug)]
pub enum AppMode { Prod, Dev }

impl From<String> for AppMode {
    fn from(v: String) -> Self {
        if v.eq("prod") {
            Self::Prod
        } else if v.eq("dev") {
            Self::Dev
        } else { Self::Dev }
    }
}

#[derive(Debug)]
pub struct AppConfig {
    pub args: AppArgs,
}

impl AppConfig {
    pub fn new (args:   AppArgs) -> Self { Self { args } }
}

#[derive(Debug)]
pub struct AppArgs { pub mode: AppMode }

impl From<clap::ArgMatches> for AppArgs {
    fn from(args: clap::ArgMatches) -> Self {
        let mode = args.get_one::<String>("mode").unwrap_or(&"dev".to_string()).to_owned();
        let mode = AppMode::from(mode);
        Self { mode }
    }
}

pub fn parse_args () -> AppConfig {
    let matches = clap::App::new("Bots tasks backend")
        .version("0.1.0")
        .about("Bots tasks backend")
        .arg(Arg::new("mode")
            .long("mode")
            .takes_value(true)
            .default_value("dev"))
    .get_matches();
    let args = AppArgs::from(matches);
    AppConfig::new(args)
}
