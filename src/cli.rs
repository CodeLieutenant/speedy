use clap::{Parser, Subcommand};

use crate::iperf3;
use lazy_static::lazy_static;

lazy_static! {
    static ref EUROPE_SERVERS: Vec<String> = vec![
        // "speedtest.wtnet.de:5200:5".to_owned(),
        // "speedtest.wtnet.de:5201:3".to_owned(),
        // "speedtest.wtnet.de:5202:3".to_owned(),
        // "speedtest.wtnet.de:5203:3".to_owned(),
        // "speedtest.wtnet.de:5204:2".to_owned(),
        // "speedtest.wtnet.de:5205:2".to_owned(),
        // "speedtest.wtnet.de:5206:1".to_owned(),
        // "speedtest.wtnet.de:5207:1".to_owned(),
        // "speedtest.wtnet.de:5208:1".to_owned(),
        // "speedtest.wtnet.de:5209:1".to_owned(),
        // "speedtest.ams1.novogara.net:5200:5".to_owned(),
        // "speedtest.ams1.novogara.net:5201:5".to_owned(),
        // "speedtest.ams1.novogara.net:5202:5".to_owned(),
        // "speedtest.ams1.novogara.net:5203:5".to_owned(),
        // "speedtest.ams1.novogara.net:5204:2".to_owned(),
        // "speedtest.ams1.novogara.net:5205:2".to_owned(),
        // "speedtest.ams1.novogara.net:5206:2".to_owned(),
        // "speedtest.ams1.novogara.net:5207:5".to_owned(),
        // "speedtest.ams1.novogara.net:5208:2".to_owned(),
        // "speedtest.ams1.novogara.net:5209:2".to_owned(),
        // "iperf.online.ne:5209:2".to_owned(),
        // "scaleway.testdebit.info:5200:2".to_owned(),
        // "a110.speedtest.wobcom.de:3".to_owned(),
        // "178.215.228.109:9201:1".to_owned(),
    ];
}

#[derive(Parser, Debug)]
#[command(
    author = "Dusan Malusev <dusan@dusanmalusev.dev",
    version = "0.1.0",
    about = "Network speed monitor.",
    long_about = "Monitor your network speed with iperf3 and store your data in InfluxDB for later processing."
)]
struct CLI {
    #[arg(short, long, default_value = None)]
    servers: Option<Vec<String>>,
    #[arg(short, long, required = false, default_value_t = 10)]
    duration: i32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {},
    Serve {},
}

pub async fn execute() {
    let cli = CLI::parse();
    let servers = cli.servers.as_ref().unwrap_or(EUROPE_SERVERS.as_ref());

    match cli.command {
        Commands::Run {} => {
            let result = iperf3::upload_speed(servers, cli.duration)
                .await
                .expect("failed to execute iperf");
            println!("{}", result.start.cookie);

            let result = iperf3::download_speed(servers, cli.duration)
                .await
                .expect("failed to execute iperf");
            println!("{}", result.start.cookie);
        }
        Commands::Serve {} => todo!(),
    }
}
