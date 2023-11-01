use std::io;
use std::string::FromUtf8Error;

use rand::distributions::WeightedError;
use rand::seq::SliceRandom;

use crate::models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to execute iperf3 command: {0}")]
    CommandError(String),

    #[error(
        "Server format is invalid, expected following format \"<server>:<port:OPTIONAL>:<weight>\""
    )]
    InvalidServerFormat,

    #[error(transparent)]
    UTF8(#[from] FromUtf8Error),

    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    Random(#[from] WeightedError),

    #[error(transparent)]
    JSON(#[from] serde_json::Error),
}

pub static IPERF3_BINARY: &'static str = "iperf3";

pub async fn download_speed<T: AsRef<str>>(
    servs: &[T],
    duration: i32,
) -> Result<models::IPerf3, Error> {
    execute_speed_test(servs, duration, true).await
}

pub async fn upload_speed<T: AsRef<str>>(
    servs: &[T],
    duration: i32,
) -> Result<models::IPerf3, Error> {
    execute_speed_test(servs, duration, false).await
}

fn build_iperf3_command(
    addr: &str,
    port: &str,
    duration: i32,
    download: bool,
) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(IPERF3_BINARY);

    command.args(&[
        "-J",
        "-Z",
        "-c",
        addr,
        "-p",
        port,
        "--connect-timeout",
        "200ms",
        "--time",
        duration.to_string().as_ref(),
        if download { "-R" } else { "" },
    ]);

    command
}

fn pick_server<'a, 'b>(servers: &'a [impl AsRef<str>]) -> Result<(String, String), Error> {
    let mut rng = rand::thread_rng();

    let server = servers.choose_weighted(&mut rng, |item| match item.as_ref().rfind(":") {
        Some(idx) if idx + 2 >= item.as_ref().len() => item.as_ref()[idx + 1..]
            .parse::<i32>()
            .expect("Weight has tobe a number"),
        Some(_) | None => 0,
    })?;

    parse_server(server)
}

async fn execute_speed_test<T: AsRef<str>>(
    servers: &[T],
    duration: i32,
    download: bool,
) -> Result<models::IPerf3, Error> {
    let (addr, port) = pick_server(servers)?;

    let mut iperf3 = build_iperf3_command(&addr, &port, duration, download);

    let output = iperf3.output().await?;

    if output.status.success() {
        let data = output.stdout;
        let deserialized: models::IPerf3 = serde_json::from_slice(&data)?;
        Ok(deserialized)
    } else {
        Err(Error::CommandError(String::from_utf8(output.stdout)?))
    }
}

fn parse_server<'str, T>(server: T) -> Result<(String, String), Error>
where
    T: AsRef<str> + 'str,
{
    let mut items = server.as_ref().splitn(3, ':');

    let addr = items
        .next()
        .ok_or_else(|| Error::InvalidServerFormat)?
        .to_string();
    let port = items.next().unwrap_or("5001").to_string();

    Ok((addr, port))
}
