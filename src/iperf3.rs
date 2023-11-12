use rand::distributions::WeightedError;
use rand::seq::SliceRandom;
use std::io;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio_util::sync::CancellationToken;

use crate::models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to execute iperf3 command (Server {1}): {0}")]
    Command(String, String),

    #[error(
        "Server format is invalid, expected following format \"<server>:<port:OPTIONAL>:<weight>\""
    )]
    InvalidServerFormat,

    #[error("install iperf3 command")]
    IperfCommandDoesNotExist,

    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    Random(#[from] WeightedError),

    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("request sending canceled")]
    Canceled,
}

pub const IPERF3_BINARY: &str = "iperf3";
pub const IPERF3_DEFAULT_PORT: &str = "5001";

pub async fn download_speed<T: AsRef<str>>(
    servs: &[T],
    duration: i32,
) -> Result<models::IPerf3, Error> {
    check_iperf3_command().await?;
    execute_speed_test(servs, duration, true).await
}

pub async fn upload_speed<T: AsRef<str>>(
    servs: &[T],
    duration: i32,
) -> Result<models::IPerf3, Error> {
    check_iperf3_command().await?;
    execute_speed_test(servs, duration, false).await
}

async fn check_iperf3_command() -> Result<(), Error> {
    let mut command = tokio::process::Command::new(IPERF3_BINARY);
    command.kill_on_drop(true);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.arg("--version");

    match command.status().await {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::IperfCommandDoesNotExist),
    }
}

fn build_iperf3_command(
    addr: &str,
    port: &str,
    duration: i32,
    download: bool,
) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(IPERF3_BINARY);
    command.kill_on_drop(true);

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    let mut dur = if duration > 10 {
        duration - 5
    } else {
        duration - 2
    }
    .to_string();

    dur.extend(&['s']);

    command.args([
        "-J",
        "-Z",
        "--connect-timeout",
        "500", // 0.5s
        "-i",
        "1",
        "-t",
        &dur,
        if download { "-R" } else { "" },
        "-c",
        addr,
        "-p",
        port,
    ]);

    command
}

fn pick_server(servers: &[impl AsRef<str>]) -> Result<(String, String), Error> {
    let mut rng = rand::thread_rng();

    let server = servers.choose_weighted(&mut rng, |item| match item.as_ref().rfind(':') {
        Some(idx) if idx + 2 >= item.as_ref().len() => item.as_ref()[idx + 1..]
            .parse::<i32>()
            .expect("Weight needs to be a number"),
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
    let mut child = iperf3.spawn()?;
    let token = CancellationToken::new();

    let sub_token = token.clone();

    let worker_handle = tokio::spawn(async move {
        tokio::select! {
            _ = sub_token.cancelled() => {
                _ = child.kill().await;
                drop(child);
                Err(Error::Canceled)
            }
            result = child.wait() => {
                let data = match child.stdout {
                    Some(ref mut out) => {
                        let mut data = String::new();
                        out.read_to_string(&mut data).await?;
                        data
                    }
                    None => String::new(),
                };

                if result?.success() {
                    let deserialized: models::IPerf3 = serde_json::from_str(&data)?;
                    Ok(deserialized)
                } else {
                    Err(Error::Command(data, format!("{addr}:{port}")))
                }
            }
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs((duration + 3) as u64)).await;
        token.cancel();
    });

    worker_handle.await.unwrap()
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
    let port = items.next().unwrap_or(IPERF3_DEFAULT_PORT).to_string();

    match items.next() {
        Some(_) => Ok((addr, port)),
        None => Ok((addr, IPERF3_DEFAULT_PORT.to_string())),
    }
}
