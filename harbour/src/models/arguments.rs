use clap::Parser;
use clap::builder::PossibleValuesParser;

#[derive(Parser, Debug)]
#[command(
    version = "0.1", 
    author = "Harbour", 
    about = "🛟 Harbour is your rust powered port for container management"
)]
pub struct Arguments {
    #[arg(short, long, required = true)]
    pub service: String,

    #[arg(short, long)]
    pub volume: Option<String>,

    #[arg(short, long)]
    pub network: Option<String>,

    #[arg(short, long)]
    pub image: Option<String>,

    #[arg(short, long)]
    pub ports: Option<String>,

    #[arg(short, long, value_parser = PossibleValuesParser::new(
        ["no", "always", "on-failure", "unless-stopped"]
    ), default_value = "no")]
    pub restart: String,

    #[arg(short, long, default_value = "docker-compose.yml")]
    pub output: String
}

impl Arguments {
    
}
