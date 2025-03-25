use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "1.0", 
    author = "Harbour installer", 
    about = "🛟 Harbour installer is your rust powered installer for harbour"
)]
pub struct Arguments {
    #[arg(short, long, required = true)]
    pub binary: String,

    #[arg(short, long)]
    pub name: Option<String>,
}
