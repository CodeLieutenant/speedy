use std::sync::Arc;

use clap::{Parser, Subcommand};
use tokio::task::JoinSet;

use crate::influxdb::{Client, Direction, Speed};
use crate::models::IPerf3;
use crate::{influxdb, iperf3, timetable};
use lazy_static::lazy_static;

lazy_static! {
    static ref EUROPE_SERVERS: Vec<String> = vec![
        "speedtest.init7.net:10".to_string(),
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
    #[arg(short, long, required = false, default_value_t = 3)]
    retries: i32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {},
    Serve { timetable: String },
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IPerf3 Error {0}")]
    IPerf3(#[from] iperf3::Error),

    #[error("InfluxDB Error {0}")]
    InfluxDB(#[from] crate::influxdb::Error),
}

async fn insert(
    client: &influxdb::Client,
    result: IPerf3,
    direction: influxdb::Direction,
    now: time::OffsetDateTime,
) -> Result<(), Error> {
    let speeds = result.intervals.iter().map(|interval| {
        let stream = &interval.streams[0];

        Speed::new(
            now + time::Duration::seconds_f64(stream.seconds),
            direction.clone(),
            stream.bits_per_second as u64,
        )
    });

    client.insert_multiple(speeds).await?;

    Ok(())
}

async fn run(
    servers: &[String],
    client: &crate::influxdb::Client,
    timeout: i32,
    retries: i32,
) -> Result<(), Error> {
    let now = time::OffsetDateTime::now_utc();

    let download = || async {
        for _ in 0..retries {
            let result = iperf3::download_speed(servers, timeout).await;

            if result.is_ok() {
                return result;
            }
        }

        Err(iperf3::Error::Canceled)
    };

    let upload = || async {
        for _ in 0..retries {
            let result = iperf3::upload_speed(servers, timeout).await;

            if result.is_ok() {
                return result;
            }
        }

        Err(iperf3::Error::Canceled)
    };

    match download().await {
        Ok(result) => {
            insert(client, result, Direction::Download, now).await?;
            println!("Values insert into InfluxDB");
        }
        Err(err) => eprintln!("Failed to execute download: {}", err),
    }

    match upload().await {
        Ok(result) => {
            insert(client, result, Direction::Upload, now).await?;
            println!("Values insert into InfluxDB");
        }
        Err(err) => eprintln!("Failed to execute upload: {}", err),
    }

    Ok(())
}

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let servers = Arc::new(cli.servers.unwrap_or(EUROPE_SERVERS.clone()));
    let influx_db_host =
        std::env::var("SPEEDY_INFLUX_HOST").unwrap_or("http://localhost:8086".to_string());
    let influx_db_bucket =
        std::env::var("SPEEDY_INFLUX_BUCKET").unwrap_or("network_speeds".to_string());
    let influx_db_token = std::env::var("SPEEDY_INFLUX_TOKEN")
        .expect("Provide an API Token for InfluxDB in SPEEDY_INFLUX_TOKEN environment variable");

    let client = Client::new(&influx_db_host, &influx_db_bucket, &influx_db_token);

    match cli.command {
        Commands::Run {} => {
            run(&servers, &client, cli.timeout, cli.retries).await?;
            Ok(())
        }
        Commands::Serve { timetable } => {
            let client = Arc::new(client);

            let path = tokio::fs::canonicalize(timetable.as_str()).await?;
            let file_content = tokio::fs::read_to_string(path).await?;
            let mut table = match timetable::parse(&file_content) {
                Ok((_, table)) => table,
                Err(_) => {
                    return Err(format!("Failed to parse timetable file ({}).", &timetable).into())
                }
            };

            let invalid: Vec<_> = table
                .iter()
                .filter(|item: &&timetable::Table| {
                    ((item.end_hour - item.start_hour) as i64) < item.duration.whole_hours()
                })
                .collect();

            if !invalid.is_empty() {
                invalid
                    .iter()
                    .for_each(|item| eprintln!("Invalid timespan {}", item));
                return Err("Timetable not provided".into());
            }

            let mut set = JoinSet::new();

            table.drain(..).for_each(|item| {
                let c = Arc::clone(&client);
                let servers = Arc::clone(&servers);
                let duration =
                    tokio::time::Duration::from_nanos(item.duration.whole_nanoseconds() as u64);
                set.spawn_local(async move {
                    loop {
                        let hour = time::OffsetDateTime::now_utc().hour();

                        if hour >= item.start_hour && hour < item.end_hour {
                            run(&servers, &c, cli.timeout, cli.retries).await.unwrap();
                        }

                        tokio::time::sleep(duration).await;
                    }
                });
            });

            while let Some(_res) = set.join_next().await {}

            Ok(())
        }
    }
}
