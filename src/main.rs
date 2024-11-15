use atp::{auth::Auth, bsky::Bsky, Client, Config, Process};
use clap::Parser;
use directories::BaseDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Options = Options::parse();
    let base_dirs = BaseDirs::new().expect("Unable to find home directory");
    let client = Client::new();

    match opts {
        Options::Auth(Auth::Login(cmd)) => {
            let response = cmd.process(&client).await?;
            let config = Config {
                session: Some(response),
            };
            config.write(&base_dirs).await?;
            println!("Login successful");
        }
        Options::Auth(Auth::Session) => {
            let config = Config::load(&base_dirs).await?;
            println!("{config}");
        }
        Options::Bsky(cmd) => {
            let config = Config::load(&base_dirs).await?;
            let response = cmd.process(&client, &config).await?;
            println!("{response}");
        }
    }
    Ok(())
}

#[derive(Parser)]
enum Options {
    #[command(subcommand)]
    Auth(Auth),
    #[command(subcommand)]
    Bsky(Bsky),
}
