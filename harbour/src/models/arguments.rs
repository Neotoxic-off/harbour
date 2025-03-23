use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Harbour", about = "🛟 Harbour is your rust powered port for container management")]
pub struct Arguments {
    #[arg(required = true)]
    pub dockerfiles: Vec<String>,

    #[arg(long)]
    pub network: Option<String>,

    #[arg(long, default_value = "no")]
    pub restart: String
}
