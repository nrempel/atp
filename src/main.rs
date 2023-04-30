use atp::{Client, Config, Server};
use clap::Parser;
use directories::BaseDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Options = Options::parse();
    let base_dirs = BaseDirs::new().expect("Unable to find home directory");
    let client = Client::new();

    match opts {
        Options::Server(server) => match server {
            Server::Login(login) => {
                let response = login.process(&client).await?;
                let config = Config {
                    session: Some(response),
                };
                config.write(&base_dirs).await?;
            }
        },
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
