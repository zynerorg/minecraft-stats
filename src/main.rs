mod api;
mod db;
mod reader;
mod structs;

use clap::Parser;
use rpassword::prompt_password;
use std::net::IpAddr;
use url::Url;

use clap::Subcommand;

fn validate_host(val: &str) -> Result<String, String> {
    // First try to parse as IP address
    if val.parse::<IpAddr>().is_ok() {
        return Ok(val.to_string());
    }

    // Simple hostname validation: only letters, digits, dots, hyphens
    // Also check length constraints (max 255 chars, labels max 63)
    if val.len() > 255 {
        return Err("Hostname too long".into());
    }

    // Split by dots and check each label
    for label in val.split('.') {
        if label.is_empty() || label.len() > 63 {
            return Err(format!("Invalid label length in hostname: {}", label));
        }
        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            || label.starts_with('-')
            || label.ends_with('-')
        {
            return Err(format!(
                "Invalid characters in hostname label: {}",
                label
            ));
        }
    }

    Ok(val.to_string())
}

#[derive(Parser)]
#[command(name = "minecraft-stats")]
struct Cli {
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    password: Option<String>,

    #[arg(short, long)]
    db: String,

    #[arg(long, value_parser = validate_host)]
    host: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Reader {
        #[arg(short, long)]
        world_path: String,
    },
    Api {},
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let password = match cli.password {
        Some(pw) => pw,
        None => prompt_password("Password: ").expect("failed to read password"),
    };

    let mut url =
        Url::parse(&format!("postgres://{}", cli.host)).expect("Invalid host");
    url.set_username(&cli.user).unwrap();
    url.set_password(Some(&password)).unwrap();
    url.set_path(&cli.db);
    let url = url.to_string();

    db::init(&url)?;

    match cli.command {
        Commands::Reader { world_path } => {
            reader::main(world_path).await;
        }
        Commands::Api {} => {
            // api::main();
        }
    }
    Ok(())
}
