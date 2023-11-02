use clap::{Parser, Subcommand};

use crate::iperf3;
use lazy_static::lazy_static;

lazy_static! {
    static ref EUROPE_SERVERS: Vec<String> = vec![
        "speedtest.init7.net:10".to_string(),
        "speedtest.lu.buyvm.net:5".to_string(),
        "iperf.online.net:5209:7".to_string(),
        "speedtest.serverius.net:5002:1".to_string(),
        "ams.speedtest.clouvider.net:5201:3".to_string(),
        "ams.speedtest.clouvider.net:5202:7".to_string(),
        "ams.speedtest.clouvider.net:5203:7".to_string(),
        "ams.speedtest.clouvider.net:5204:10".to_string(),
        "ams.speedtest.clouvider.net:5205:10".to_string(),
        "ams.speedtest.clouvider.net:5206:10".to_string(),
        "ams.speedtest.clouvider.net:5207:10".to_string(),
        "ams.speedtest.clouvider.net:5208:4".to_string(),
        "ams.speedtest.clouvider.net:5209:4".to_string(),
        "speedtest.ams1.novogara.net:5209:4".to_string(),
        "speedtest.ams1.novogara.net:5201:3".to_string(),
        "speedtest.ams1.novogara.net:5202:4".to_string(),
        "speedtest.ams1.novogara.net:5204:5".to_string(),
    ];
}

#[derive(Parser, Debug)]
#[command(
    author = "Dusan Malusev <dusan@dusanmalusev.dev",
    version = "0.1.0",
    about = "Network speed monitor.",
    long_about = "Monitor your network speed with iperf3 and store your data in InfluxDB for later processing."
)]
struct Cli {
    #[arg(short, long, default_value = None)]
    servers: Option<Vec<String>>,
    #[arg(short, long, required = false, default_value_t = 7)]
    timeout: i32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {},
    Serve {},
}

// 0-12 1h


pub async fn execute() {
    let cli = Cli::parse();
    let servers = cli.servers.as_ref().unwrap_or(EUROPE_SERVERS.as_ref());

    match cli.command {
        Commands::Run {} => {
            let download = iperf3::download_speed(servers, cli.timeout).await;
            let upload = iperf3::upload_speed(servers, cli.timeout).await;

            match download {
                Ok(result) => println!("Download samples: {}", result.intervals.len()),
                Err(err) => eprintln!("Failed to execute download: {}", err),
            }

            match upload {
                Ok(result) => println!("Upload samples: {}", result.intervals.len()),
                Err(err) => eprintln!("Failed to execute upload: {}", err),
            }
        }
        Commands::Serve {} => {},
    }
}
