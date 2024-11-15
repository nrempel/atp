use atp::{Client, Config, Process, Server};
use clap::Parser;
use directories::BaseDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Options = Options::parse();
    let base_dirs = BaseDirs::new().expect("Unable to find home directory");
    let client = Client::new();

    match opts {
        Options::Server(cmd) => {
            let config = match cmd {
                Server::Login(_) => Config::default(),
                _ => Config::load(&base_dirs).await?,
            };
            let response = cmd.process(&client, &config, &base_dirs).await?;
            println!("{response}");
        }
        Options::Session => {
            let config = Config::load(&base_dirs).await?;
            println!("{config}");
        }
    }
    Ok(())
}

#[derive(Parser)]
enum Options {
    #[command(subcommand)]
    Server(Server),
    Session,
}
